use num::Complex;
use winapi::vc::vadefs::uintptr_t;
use core::cmp::min;

use super::fft::*;

// Pattern Scanning with dump, fft.
// <summary>O(NlogM)</summary>
pub fn pat_fft(pat: &[u8], mask: &[u8], start: uintptr_t, dump: &[u8]) -> Option<uintptr_t> {
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