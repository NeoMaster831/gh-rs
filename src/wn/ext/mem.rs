use std::{mem::*, cmp::*};

use num::Complex;
use winapi::{vc::vadefs::*, um::{memoryapi::*, errhandlingapi::GetLastError}, shared::{ntdef::*, basetsd::*, minwindef::*}};

use crate::utils::fft::{VPoly, Cpx, mul_poly};

use super::structs::Proc;

impl Proc {

    // readn. Basic function of rpm
    pub fn readn(&self, wh: uintptr_t, storeat: LPVOID, size: SIZE_T) -> Option<DWORD> {

        let res = unsafe {
            ReadProcessMemory(
                self.handle, 
                wh as LPCVOID, 
                storeat, 
                size, 
                NULL as *mut SIZE_T,
            )
        };

        if res == 0 {
            let s = unsafe { GetLastError() };
            return Some(s);
        }
        None
        
    }

    // read mem. If error occurs, return last error. check msdn.
    pub fn read<T>(&self, wh: uintptr_t, storeat: *mut T) -> Option<DWORD> {
        let size = size_of::<T>();
        return self.readn(wh, storeat as LPVOID, size);
    }

    // writen. basic func of write
    pub fn writen(&self, wh: uintptr_t, what: LPCVOID, size: SIZE_T) -> Option<DWORD> {

        let res = unsafe {
            WriteProcessMemory(
                self.handle, 
                wh as LPVOID, 
                what,
                size,
                NULL as *mut SIZE_T
            )
        };

        if res == 0 {
            let s = unsafe { GetLastError() };
            return Some(s);
        }
        None
        
    }

    // write mem. If error occurs, return last error. check msdn.
    pub fn write<T>(&self, wh: uintptr_t, what: *const T) -> Option<DWORD> {
        let size = size_of::<T>();
        return self.writen(wh, what as LPCVOID, size);
    }

    // dump. If error occurs, return GetLastError caused by rpm.
    pub fn dump(&self, start: uintptr_t, end: uintptr_t) -> Result<Vec<u8>, DWORD> {
        
        let mut toret: Vec<u8> = vec![];
        for i in start..end {
            let mut store: u8 = 0;
            match self.read::<u8>(i, &mut store) {
                None => toret.push(store),
                Some(dw) => return Err(dw),
            };
        }
        Ok(toret)
        
    }

    fn _pat_basic(&self, pat: &[u8], mask: &[u8], start: uintptr_t, size: SIZE_T) -> Result<Option<uintptr_t>, DWORD> {

        let length: usize = pat.len();
        let mut origin: Vec<u8> = vec![0; length];
        let store = origin.as_mut_ptr();

        // 0 is invalid value for sure.
        let mut found: uintptr_t = 0;

        for i in start..(start+size-length) {
            match self.readn(i, store as LPVOID, length) {
                None => {
                    let mut exact = true;
                    for j in 0..length {
                        if origin[j] != pat[j] && mask[j] != b'?' {
                            exact = false;
                            break;
                        }
                    }
                    if exact {
                        found = i;
                        break;
                    }
                },
                Some(dw) => return Err(dw),
            };
        }

        if found == 0 {
            Ok(None)
        } else {
            Ok(Some(found))
        }

    }

    // use fft for **FASTER** search
    // NlogM
    // Wildcard is 0x0, +1 to every others.
    pub fn _pat_dump(&self, pat: &[u8], mask: &[u8], start: uintptr_t, dump: &[u8]) -> Option<uintptr_t> {
        
        let mut mod_pat: Vec<u16> = vec![];
        let mut mod_dump: Vec<u16> = vec![];
        for i in 0..pat.len() {
            let rev_idx = pat.len() - 1 - i;
            mod_pat.push(match mask[rev_idx] {
                b'?' => 0,
                _ => pat[rev_idx] as u16 + 1,
            });
        } for i in 0..dump.len() {
            mod_dump.push(dump[i] as u16 + 1);
        }

        for j in (0..dump.len()).step_by(pat.len()) {
            let md_slice = &mod_dump.as_slice()[j..min(j + 2 * pat.len(), mod_dump.len())];
            let mut t = vec![Vec::<Cpx>::new(); 4];
            let mut s = vec![Vec::<Cpx>::new(); 4];

            for i in 0..mod_pat.len() {
                t[1].push(Complex{ re: mod_pat[i] as f64, im: 0.0 });
            } for i in 0..md_slice.len() {
                s[1].push(Complex{ re: md_slice[i] as f64, im: 0.0 });
            }

            for i in 0..s[1].len() {
                let square = Complex{ re: s[1][i].re * s[1][i].re, im: 0.0 };
                let cube = Complex{ re: square.re * s[1][i].re, im: 0.0 };
                s[2].push(square); s[3].push(cube);
            } for i in 0..t[1].len() {
                let square = Complex{ re: t[1][i].re * t[1][i].re, im: 0.0 };
                let cube = Complex{ re: square.re * t[1][i].re, im: 0.0 };
                t[2].push(square); t[3].push(cube);
            }

            let mut r = vec![Vec::<Cpx>::new(); 4];
            for k in 1..4 {
                r[k] = mul_poly(&mut t[4 - k], &mut s[k]);
            }

            let mut check: VPoly = vec![];
            for k in 0..r[1].len() {
                check.push(Complex{re: r[1][k].re + -2.0 * r[2][k].re + r[3][k].re, im: 0.0});
            }
            for i in (pat.len() - 1)..md_slice.len() {
                if check[i].re.abs() < 1e-6 {
                    return Some(start + j + i - pat.len() + 1);
                }
            }
        }
        return None

    }

    // TODO: add pattern search
    // TODO: add patch

}