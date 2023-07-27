use std::{println, todo};

use rand::prelude::*;

#[derive(Debug)]
pub struct NtruIntPoly {
    pub n: usize,
    pub coeffs: Vec<i16>,
}

pub fn ntruprime_mult_poly(
    a: &NtruIntPoly,
    b: &NtruIntPoly,
    c: &mut NtruIntPoly,
    modulus: u16,
) -> bool {
    let n = a.n;

    if n != b.n {
        return false;
    }

    c.n = n;
    c.coeffs = vec![0; n];

    for k in 0..n {
        let mut ck1 = 0;

        for i in 0..=k {
            ck1 += (a.coeffs[i] as u64) * (b.coeffs[k - i] as u64);
        }

        let mut ck2 = 0;

        for i in (k + 1)..n {
            ck2 += (a.coeffs[i] as u64) * (b.coeffs[k + n - i] as u64);
        }

        let ck = c.coeffs[k] as u64 + ck1 + ck2;

        c.coeffs[k] = (ck % (modulus as u64)) as i16;

        if k < n - 1 {
            let ck = c.coeffs[k + 1] as u64 + ck2;

            c.coeffs[k + 1] = (ck % (modulus as u64)) as i16;
        }
    }

    true
}

fn ntruprime_inv_int(mut a: u16, modulus: u16) -> u16 {
    let mut x: i16 = 0;
    let mut lastx: i16 = 1;
    let mut y: i16 = 1;
    let mut lasty: i16 = 0;
    let mut b: i16 = modulus as i16;

    while b != 0 {
        let quotient = (a as i16) / b;

        let temp = a as i16;
        a = b as u16;
        b = temp % b;

        let temp = x;
        x = lastx - quotient * x;
        lastx = temp;

        let temp = y;
        y = lasty - quotient * y;
        lasty = temp;
    }

    if lastx < 0 {
        lastx += modulus as i16;
    }

    lastx as u16
}

impl NtruIntPoly {
    // Add here random method
    pub fn new(n: usize) -> Self {
        let mut rng = thread_rng();
        let coeffs: Vec<i16> = (0..n)
            .map(|_| {
                let entropy = rng.gen::<u32>();

                (entropy % 3) as i16
            })
            .collect();

        NtruIntPoly { n, coeffs }
    }

    pub fn from_zero(n: usize) -> Self {
        // Zeros a polynomial and sets the number of coefficients
        let coeffs = vec![0i16; n];

        NtruIntPoly { n, coeffs }
    }

    pub fn equals_zero(&self) -> bool {
        let sum: i16 = self.coeffs.iter().sum();

        sum == 0
    }

    pub fn get_poly_degree(&self) -> usize {
        for i in (0..=self.n - 1).rev() {
            if self.coeffs[i] != 0 {
                return i;
            }
        }

        0
    }

    pub fn get_inv_poly(&self, modulus: u16) {
        let n = self.n;
        let im = modulus as i16;
        let mut inv = NtruIntPoly::from_zero(n);
        let mut k = 0;
        let mut b = NtruIntPoly::from_zero(n + 1);

        b.coeffs[0] = 1;

        let mut c = NtruIntPoly::from_zero(n + 1);

        // f = a
        let mut f = NtruIntPoly::from_zero(n + 1);

        f.coeffs[..n].copy_from_slice(&self.coeffs[..n]);
        f.coeffs[n] = 0;

        // g = x^p - x - 1
        let mut g = NtruIntPoly::from_zero(n + 1);

        g.coeffs[0] = im - 1;
        g.coeffs[1] = im - 1;
        g.coeffs[n] = 1;

        loop {
            while f.coeffs[0] == 0 {
                // f(x) = f(x) / x
                for i in 1..=n {
                    f.coeffs[i - 1] = f.coeffs[i];
                }

                f.coeffs[n] = 0;

                // c(x) = c(x) * x
                for i in (1..n).rev() {
                    c.coeffs[i] = c.coeffs[i - 1];
                }

                c.coeffs[0] = 0;
                k += 1;

                if f.equals_zero() {
                    // return None
                    return ();
                }
            }

            //
        }
    }
}

#[test]
fn test_ntru_poly() {
    let mut poly = NtruIntPoly::new(761);

    // dbg!(poly);
}

#[test]
fn test_ntruprime_zero() {
    let poly = NtruIntPoly::from_zero(761);

    // dbg!(poly);
}

#[test]
fn ntruprime_inv_int_test() {
    let a: u16 = 7175;
    let mod0: u16 = 9829;
    let res = ntruprime_inv_int(a, mod0);

    assert!(res == 2885);
}

#[test]
fn test_from_zero() {
    let non_zero_poly = NtruIntPoly::new(761);
    let zero_poly = NtruIntPoly::from_zero(761);

    assert!(!non_zero_poly.equals_zero());
    assert!(zero_poly.equals_zero());
}

#[test]
fn test_get_poly_degre() {
    let zero_poly = NtruIntPoly::from_zero(740);
    let mut non_zero_poly = NtruIntPoly::from_zero(740);

    non_zero_poly.coeffs[non_zero_poly.n - 10] = 9;

    assert!(zero_poly.get_poly_degree() == 0);
    assert!(non_zero_poly.get_poly_degree() == 730);
}
