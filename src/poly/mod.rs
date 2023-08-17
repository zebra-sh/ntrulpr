use crate::math::euclid_inv_num::euclid_num_mod_inverse;
use num::traits::Euclid;
use num::{CheckedSub, FromPrimitive};
use std::cmp::PartialOrd;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};
use std::string::ToString;

#[derive(Clone, Debug)]
pub struct PolyInt<T> {
    pub coeffs: Vec<T>,
}

impl<T> ToString for PolyInt<T>
where
    T: Copy + Ord + FromPrimitive + std::fmt::Display + std::fmt::Debug,
{
    fn to_string(&self) -> String {
        let mut coeffs = self.coeffs.to_vec();
        // let p = coeffs.len();
        let mut result = String::new();

        coeffs.sort();

        for (i, c) in coeffs.iter().enumerate() {
            if *c == T::from_u8(0).unwrap() {
                continue;
            }
            if *c < T::from_u8(0).unwrap() && i != 0 {
                result.push('-');
            }

            result.push_str(&format!("{}xp~^{}", c, i));
        }

        result
    }
}

impl<T> PolyInt<T>
where
    T: Copy
        + Euclid
        + Mul<Output = T>
        + Div<Output = T>
        + Sub<Output = T>
        + Add<Output = T>
        + CheckedSub
        + AddAssign
        + SubAssign
        + PartialOrd<T>
        + FromPrimitive
        + std::fmt::Debug,
{
    pub fn empty() -> Self {
        let coeffs = vec![];

        PolyInt { coeffs }
    }

    pub fn from(coeffs: &[T]) -> Self {
        PolyInt {
            coeffs: Vec::from(coeffs),
        }
    }

    pub fn from_zero(n: usize) -> Self {
        // Zeros a polynomial and sets the number of coefficients
        let coeffs = vec![T::from_u8(0).unwrap(); n];

        PolyInt { coeffs }
    }

    pub fn is_small(&self) -> bool {
        self.coeffs
            .iter()
            .all(|&value| value <= T::from_i8(1).unwrap() && value >= T::from_i8(-1).unwrap())
    }

    // Modifies a polynomial by taking each coefficient modulo the given modulus.
    pub fn mod_poly(&mut self, modulus: T) -> Self {
        self.coeffs = self
            .coeffs
            .iter_mut()
            .map(|coeff| coeff.rem_euclid(&modulus))
            .collect();

        self.to_owned()
    }

    pub fn mult_mod(&mut self, factor: T, modulus: T) {
        self.coeffs.iter_mut().for_each(|coeff| {
            *coeff = (*coeff as T * factor).rem_euclid(&modulus);
        });
    }

    // Multiplies a polynomial by x^(-1) in (Z/qZ)[x][x^p-x-1] where p=a->N, q=modulus
    pub fn div_x(&mut self, modulus: T) {
        let n = self.coeffs.len();
        let a0 = self.coeffs[0];

        self.coeffs.rotate_left(1);
        self.coeffs[n - 1] = a0;

        self.coeffs[0] = self.coeffs[0].sub(a0).add(modulus).rem_euclid(&modulus)
    }

    pub fn equals_zero(&self) -> bool {
        for item in self.coeffs.iter() {
            if *item == T::from_u8(0).unwrap() {
                continue;
            } else {
                return false;
            }
        }

        true
    }

    // Subtracts one polynomial from another coefficient-wise.
    pub fn sub_poly(&mut self, poly: &[T]) -> Self {
        for (c1, &c2) in self.coeffs.iter_mut().zip(poly.iter()) {
            *c1 -= c2;
        }

        self.to_owned()
    }

    pub fn mult_int(&mut self, n: T) -> Self {
        self.coeffs = self.coeffs.iter_mut().map(|v| *v * n).collect();

        self.to_owned()
    }

    // Multiplies two polynomials using convolution of coefficients.
    pub fn mult_poly(&mut self, poly: &[T]) -> Self {
        let len_result = self.coeffs.len() + poly.len() - 1;
        let mut result: Vec<T> = vec![T::from_u8(0).unwrap(); len_result];

        for (i, &c1) in self.coeffs.iter().enumerate() {
            for (j, &c2) in poly.iter().enumerate() {
                result[i + j] += c1 * c2;
            }
        }

        self.coeffs.clear();
        self.coeffs.extend_from_slice(&result);

        self.to_owned()
    }

    // Adds polynomials coefficient-wise.
    pub fn add_poly(&mut self, poly: &[T]) -> Self {
        for (c1, &c2) in self.coeffs.iter_mut().zip(poly.iter()) {
            *c1 += c2;
        }

        self.to_owned()
    }

    // create a new poly from factor ring for poly x^p - x - 1
    pub fn create_factor_ring(&self, x: &[T], modulus: T) -> PolyInt<T> {
        let x_deg1: PolyInt<T> = PolyInt::from(&x);
        let mut x_deg_p: PolyInt<T> = PolyInt::from(&[T::from_u8(1).unwrap()]);
        let modulus_poly = self.clone().mod_poly(modulus);

        for &coeff in &self.coeffs {
            x_deg_p = x_deg_p.mult_poly(&x_deg1.coeffs);
            x_deg_p = x_deg_p.mod_poly(modulus);

            let mut one_times_coeff_deg = PolyInt::from(&[coeff]);

            one_times_coeff_deg.mult_poly(&x_deg1.coeffs);
            one_times_coeff_deg.mod_poly(modulus);

            x_deg_p.add_poly(&one_times_coeff_deg.coeffs);
            x_deg_p.mod_poly(modulus);
        }

        modulus_poly
    }

    pub fn newton_inversion(&self) -> PolyInt<T> {
        let one = T::from_u8(1).unwrap();
        let n = self.coeffs.len() - 1;
        let mut inv = PolyInt::from_zero(n + 1);

        inv.coeffs[0] = one.div(self.coeffs[0]);

        for _ in 1..=n + 1 {
            let mut self_times_inv = self.clone().mult_poly(&inv.coeffs);
            let one_minus_self_times_inv = self_times_inv.sub_poly(&[T::from_u8(1).unwrap()]);

            let clone_inv = inv.clone().mult_poly(&one_minus_self_times_inv.coeffs);

            inv = inv.sub_poly(&clone_inv.coeffs);
        }

        inv
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_empty_poly() {
        let empty: PolyInt<u8> = PolyInt::empty();

        assert!(empty.coeffs.len() == 0);
    }

    #[test]
    fn test_init_from_coeffs() {
        let coefficients = [0, -1, -2, 2];
        let test_poly = PolyInt::from(&coefficients);

        assert!(test_poly.coeffs == coefficients);
    }

    #[test]
    fn test_is_small() {
        let coefficients_big = [0, -1, -2, 2];
        let coefficients_small = [0, -1, -1, 1];

        let poly = PolyInt::from(&coefficients_big);

        assert!(!poly.is_small());

        let poly = PolyInt::from(&coefficients_small);
        assert!(poly.is_small());
    }

    #[test]
    fn test_mod_poly() {
        let polynomial = [5, 3, 2, 1];
        let modulus = 7;
        let expected_result = [5 % modulus, 3 % modulus, 2 % modulus, 1 % modulus];
        let result = PolyInt::from(&polynomial).mod_poly(modulus);

        assert!(result.coeffs == expected_result);
    }

    #[test]
    fn test_poly_add() {
        let polynomial1 = [5, 3, 2, 1];
        let polynomial2 = [2, -1, 0, 4];
        let expected_result = [5 + 2, 3 - 1, 2 + 0, 1 + 4];
        let result = PolyInt::from(&polynomial1).add_poly(&polynomial2);

        assert!(result.coeffs == expected_result);
    }

    #[test]
    fn test_poly_mult() {
        let polynomial1 = [1, 2, 3]; // x^2 + 2x + 3
        let polynomial2 = [2, -1]; // 2x - 1
        let expected_result = [2, 3, 4, -3]; // 2x^3 + 3x^2 + 4x - 3
        let result = PolyInt::from(&polynomial1).mult_poly(&polynomial2);

        assert!(result.coeffs == expected_result);
    }

    #[test]
    fn test_mult_poly_int() {
        let polynomial1 = [1, -1, 0, -1, 1];
        let expected_result = [1 * 3, -1 * 3, 0 * 3, -1 * 3, 1 * 3];
        let result = PolyInt::from(&polynomial1).mult_int(3);

        assert!(result.coeffs == expected_result);
    }

    #[test]
    fn test_create_factor_ring() {
        let coefficients = [
            -1, 1, 0, 1, -1, 0, 0, 1, 1, -1, 0, 1, -1, 1, -1, -1, 0, -1, 0, -1, -1, -1, 0, 0, 0,
        ];
        let poly: PolyInt<i16> = PolyInt::from(&coefficients);
        let x: Vec<i16> = vec![0, 1];
        let modulus: i16 = 4591;
        let fq_ring = poly.create_factor_ring(&x, modulus);

        assert!(
            fq_ring.coeffs
                == [
                    4590, 1, 0, 1, 4590, 0, 0, 1, 1, 4590, 0, 1, 4590, 1, 4590, 4590, 0, 4590, 0,
                    4590, 4590, 4590, 0, 0, 0
                ]
        );

        let coefficients: Vec<i64> = vec![
            3513768263, 2914455508, 1644203955, 2998019489, 2134992655, 310005361, 267242615,
            3554143560, 1516911024, 206811649, 3707389687,
        ];
        let poly: PolyInt<i64> = PolyInt::from(&coefficients);
        let x: Vec<i64> = vec![0, 1];
        let modulus: i64 = 4591;
        let fq_ring: PolyInt<i64> = poly.create_factor_ring(&x, modulus);

        assert!(fq_ring.coeffs == [503, 1479, 1579, 78, 3197, 2677, 505, 2546, 3305, 872, 1093]);
    }

    #[test]
    fn test_to_string() {
        // let coefficients = [1, -1, -2, -3, -6, -8];
        // let poly: PolyInt<i16> = PolyInt::from(&coefficients);

        // dbg!(poly.to_string());
    }

    #[test]
    fn test_is_zeros() {
        let coeffs = vec![0; 716];
        let mut poly = PolyInt::from(&coeffs);

        assert!(poly.equals_zero());

        poly.coeffs[1] = 1;

        assert!(!poly.equals_zero());

        poly.coeffs[1] = -1;

        assert!(!poly.equals_zero());
    }

    #[test]
    fn test_mult_mod() {
        let mut test_poly = PolyInt::from(&[1, 2, 2, 0, 0, 1, 2, 2, 2]);

        test_poly.mult_mod(3845, 9829);

        assert!(test_poly.coeffs == [3845, 7690, 7690, 0, 0, 3845, 7690, 7690, 7690]);
    }

    #[test]
    fn test_div_x() {
        let mut test_poly = PolyInt::from(&[7756, 7841, 1764, 7783, 4731, 2717, 1132, 1042, 273]);
        let k = 1475;

        for _ in 0..k {
            test_poly.div_x(9829);
        }

        assert!(test_poly.coeffs == [5018, 6408, 7987, 4832, 1047, 387, 1857, 4668, 2577]);
    }

    #[test]
    fn test_get_inv_poly() {
        let q = 4591;
        let mut k = 0;
        let g: PolyInt<i128> = PolyInt::from(&[1, 44, 99, 112, 193, 1235, 908, 285, 9475]);

        dbg!(g.newton_inversion().mult_poly(&g.coeffs));

        // loop {
        //     k += 1;
        //     match g.get_inv_poly(q) {
        //         Some(g_inv) => {
        //             assert_eq!(
        //                 g_inv.coeffs,
        //                 [2745, 2258, 3329, 2984, 1550, 2900, 700, 3283, 2267]
        //             );
        //             break;
        //         }
        //         None => {
        //             assert!(k < 100, "Incorrect inv poly!");
        //             continue;
        //         }
        //     }
        // }
    }

    #[test]
    fn test_inverse_poly() {
        // let x: Vec<i64> = vec![0, 1]; // x^p - x - 1
        //
        // let polynomial_coeffs1 = vec![1, 2, 3]; // x^2 + 2x + 3
        // let expected_inverse1 = vec![1, 3, 4]; // x^2 + 3x + 4
        //
        // let polynomial_coeffs2 = vec![1, 1, 1, 1]; // x^3 + x^2 + x + 1
        // let expected_inverse2 = vec![1, 6, 3, 5]; // x^3 + 6x^2 + 3x + 5
        // let polynomial_coeffs3 = vec![-1, -1, 0, 2, -1];
        //
        // // let inverse_coeffs1 = PolyInt::from(&polynomial_coeffs1).inverse_poly(5);
        // // let inverse_coeffs2 = PolyInt::from(&polynomial_coeffs2).inverse_poly(7);
        // // let inverse_coeffs3 = PolyInt::from(&polynomial_coeffs3).inverse_poly(3);
        // let ring_f3: PolyInt<i64> = PolyInt::from(&[
        //     1, -1, -1, -1, 1, 0, -1, 1, 1, 0, 0, -1, 1, 0, 0, 1, 0, -1, 1, 0, -1, 0, 0, 0, 1, 0, 1,
        //     1, 0, 1, -1, 0, -1, -1, 1, 0, 0, 1, -1, 0, 1, 1, 0, -1, 0, -1, 0, -1, -1, 1, 1, 0, 1,
        //     0, 1, 0, 0, -1, 1, 1, 1, 0, 0, 0, -1, 0, 0, 0, 0, -1, 0, 1, 1, 0, -1, 0, 0, -1, -1, 1,
        //     1, 0, -1, 0, -1, -1, 1, 1, -1, 1, -1, -1, 1, 1, 0, -1, 0, 0, 1, 1, -1, 0, 0, -1, 0, 0,
        //     -1, 0, -1, 0, -1, -1, 1, 1, 1, 0, -1, 0, 1, 1, 0, 1, -1, 0, -1, 0, 0, 1, 0, 0, 0, 0,
        //     -1, 0, -1, 0, -1, 0, 1, -1, 0, 0, -1, 0, 1, 1, 0, 1, -1, 0, 0, 1, 1, 0, 0, -1, 1, 1, 0,
        //     -1, 1, 0, -1, 0, -1, -1, -1, -1, 1, -1, 1, 1, 0, -1, -1, 1, 0, 0, 0, 1, 0, -1, 0, 0,
        //     -1, 1, 0, -1, 1, 1, -1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, -1, 0, 0, 1, 1, -1, -1, 1, 1,
        //     -1, 1, 0, 1, 0, 0, -1, -1, -1, -1, -1, -1, 0, 0, -1, 0, 1, -1, -1, -1, 0, 0, 1, 0, 1,
        //     -1, 0, 1, 0, 0, -1, -1, 1, 1, 0, 0, 1, 0, -1, 1, 1, -1, 1, 0, 0, -1, -1, -1, 1, 0, 1,
        //     -1, -1, 0, 0, 1, 1, 0, 1, 0, 1, 1, -1, -1, -1, -1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0,
        //     0, 0, 0, 0, 1, -1, 1, 0, -1, -1, 0, 0, 0, 1, 1, 0, -1, 1, 1, 0, -1, 1, 0, 0, 1, -1, 1,
        //     0, 0, 0, 1, 1, 0, 0, -1, 1, 1, 0, 0, 0, -1, -1, 1, 1, -1, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1,
        //     0, 0, -1, 0, -1, 1, -1, 0, -1, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, -1, -1, 1, 0, 0,
        //     -1, 1, -1, -1, 0, 1, -1, 0, -1, 1, 0, -1, 1, 1, -1, 1, -1, -1, 1, -1, 0, 1, 0, 1, 1,
        //     -1, 1, 1, -1, 0, 0, 1, 0, 1, 1, -1, 0, 0, 1, 0, -1, -1, 1, 1, -1, 1, 1, -1, 1, 1, 0, 1,
        //     1, 1, 0, 1, 1, -1, 1, 1, -1, 1, 0, 0, 1, 1, 1, -1, 0, 0, 0, 0, -1, 0, -1, 0, 1, 1, -1,
        //     0, 0, 1, 0, 0, 0, 0, 0, 1, 1, -1, -1, 1, 0, 1, 0, 0, 1, 1, -1, 0, 0, 0, 0, -1, -1, -1,
        //     -1, 0, -1, -1, 0, 1, 0, 1, 0, -1, -1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1, 0, -1, 1, -1, -1,
        //     0, 0, 1, 1, 1, 0, 1, -1, 1, 0, 0, 1, 1, -1, -1, 1, 0, -1, -1, 1, 1, 0, 1, 0, -1, 0, 0,
        //     0, -1, -1, 1, 1, 1, 0, 1, -1, -1, 0, 0, -1, -1, -1, -1, -1, -1, 1, 0, 1, 0, -1, 0, 0,
        //     1, -1, -1, -1, 0, 0, 1, 0, -1, 0, 1, 1, 1, 0, 1, 1, -1, -1, -1, -1, 0, 0, 0, 1, 0, 0,
        //     0, -1, 1, 0, 0, 0, -1, 1, 1, 1, -1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 1, -1, -1, 0, 1,
        //     -1, 1, -1, 1, 1, -1, -1, 0, -1, 1, 0, -1, 0, -1, 1, 1, 0, 1, -1, 0, -1, 0, -1, 0, 0, 1,
        //     1, 1, 1, 1, -1, 0, 1, 0, 0, 1, 0, 0, -1, 0, 0, 0, -1, -1, -1, 0, 0, 1, 1, 1, -1, -1, 0,
        //     0, -1, -1, 0, 0, -1, -1, -1, -1, 0, 0, -1, -1, 0, 1, -1, 1, 1, 1, 1, -1, 0, -1, -1, -1,
        //     -1, 0, -1, 0, 1, 0, 1, -1, -1, 1, -1, -1, -1, -1, 0, 0, 1, 0, 1, -1, 0, 0, -1, 0, 1, 0,
        //     -1, 1, -1, 1, 1, 1, 0, 1, 0, -1, 1, 1, 1, 1, 0, -1, -1, -1, 1, -1, -1, 1, 1, 0, -1, -1,
        //     0, 1, 0, 0, -1, 1, -1, 0, 0, 1, 0, 0, 1, 1, 0, -1, 0, 0, 1, 0, -1, -1, 1, -1,
        // ])
        // .create_factor_ring(&x, 3);
        // let inv_f3 = ring_f3.inverse_poly(761);

        // assert!(vec![2, 0, 0, 0, 0] == inverse_coeffs3.coeffs);
        // assert_eq!(inverse_coeffs1.coeffs, expected_inverse1);
        // assert_eq!(inverse_coeffs2.coeffs, expected_inverse2);

        // dbg!(inv_f3);
        // assert!(inverse_coeffs4.coeffs == [2745, 2258, 3329, 2984, 1550, 2900, 700, 3283, 2267]);
    }
}
