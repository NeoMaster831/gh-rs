use std::mem::*;

use winapi::{vc::vadefs::*, um::{memoryapi::*, errhandlingapi::GetLastError, winnt::MEMORY_BASIC_INFORMATION}, shared::{ntdef::*, basetsd::*, minwindef::*}};

use crate::utils::alg::pat_fft;

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

    fn _pat_dump(&self, pat: &[u8], mask: &[u8], start: uintptr_t, dump: &[u8]) -> Option<uintptr_t> {
        return pat_fft(pat, mask, start, dump);
    }

    pub fn pat_scan(&self, pat: &[u8], mask: &[u8], start: uintptr_t, end: uintptr_t, mode: &str) -> Option<uintptr_t> {
        let mut mbi = MEMORY_BASIC_INFORMATION { AllocationBase: unsafe { zeroed() }, RegionSize: 0, BaseAddress: unsafe { zeroed() }, AllocationProtect: 0, State: 0, Protect: 0, Type: 0 };
        
        let mut cur = start;
        while cur < end {
            cur += mbi.RegionSize;
            if unsafe { VirtualQueryEx(self.handle, cur as LPCVOID, &mut mbi, size_of::<MEMORY_BASIC_INFORMATION>()) } == 0 || (
                mbi.State != 0x1000 || 
                mbi.Protect&0x100 != 0 ||
                mbi.Protect&0x1 != 0
            ) {
                continue;
            }

            if mode == "basic" {
                match self._pat_basic(pat, mask, cur, mbi.RegionSize) {
                    Ok(res) => match res {
                        Some(k) => return Some(k),
                        None => (),
                    }
                    Err(_) => (),
                }
            } else if mode == "dump" {
                match self.dump(cur, cur + mbi.RegionSize) {
                    Ok(dump) => {
                        match self._pat_dump(pat, mask, cur, dump.as_slice()) {
                            Some(k) => return Some(k),
                            None => (),
                        }
                    } Err(_) => (),
                }
            }
        }
        None
    }

    // TODO: add patch

}