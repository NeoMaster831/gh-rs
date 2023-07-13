use num::Complex;

pub type Cpx = Complex<f64>;
pub type VPoly = Vec<Cpx>;
pub type SPoly<'a> = &'a mut [Cpx];

pub fn fft(poly: SPoly, inverse: bool) {

    let n: usize = poly.len();
    if n <= 1 {
        return;
    }

    let mut even: VPoly = vec![Complex{ re: 0.0, im: 0.0 }; n / 2];
    let mut odd: VPoly = vec![Complex{ re: 0.0, im: 0.0 }; n / 2];
    for i in 0..(n/2) {
        even[i] = poly[i * 2];
        odd[i] = poly[i * 2 + 1];
    }

    fft(even.as_mut_slice(), inverse);
    fft(odd.as_mut_slice(), inverse);
    
    let angle: f64 = 2.0 * std::f64::consts::PI / (n as f64) * match inverse {
        false => 1.0,
        true => -1.0,
    };

    let mut w = Complex{ re: 1.0, im: 0.0 };
    let wn = Complex{ re: angle.cos(), im: angle.sin() };

    for i in 0..(n/2) {
        let tmp = w * odd[i];
        poly[i] = even[i] + tmp;
        poly[i + n / 2] = even[i] - tmp;
        w *= wn;
    }

    if inverse {
        for i in 0..n {
            poly[i] /= Complex{ re: 2.0, im: 0.0 };
        }
    }

}

pub fn mul_poly(t: SPoly, s: SPoly) -> VPoly {

    let mut n = 1;
    while n < t.len() + s.len() {
        n <<= 1;
    }

    let mut a = vec![Complex{ re: 0.0, im: 0.0 }; n];
    let mut b = vec![Complex{ re: 0.0, im: 0.0 }; n];
    let mut c = vec![Complex{ re: 0.0, im: 0.0 }; n];

    for i in 0..t.len() {
        a[i] = t[i];
    } for i in 0..s.len() {
        b[i] = s[i];
    }

    fft(a.as_mut_slice(), false);
    fft(b.as_mut_slice(), false);

    for i in 0..n {
        c[i] = a[i] * b[i];
    }

    fft(c.as_mut_slice(), true);
    c
    
}