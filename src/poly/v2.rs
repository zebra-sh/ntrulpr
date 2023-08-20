use crate::poly::traits::TryFrom;
use num::{traits::Euclid, FromPrimitive, One, ToPrimitive, Zero};
use std::ops::{AddAssign, Mul, Neg};

#[derive(Debug)]
pub struct PolyInt<N: Sized, const SIZE: usize> {
    coeffs: [N; SIZE],
}

impl<N, const SIZE: usize> PolyInt<N, SIZE>
where
    N: Sized + Copy + Default,
{
    pub fn from(coeffs: [N; SIZE]) -> Self {
        Self { coeffs }
    }

    pub fn new() -> Self {
        Self {
            coeffs: [N::default(); SIZE],
        }
    }
}

impl<N, const SIZE: usize> PolyInt<N, SIZE>
where
    N: Sized,
{
    /// Gets the slice of internal data.
    #[inline]
    pub fn get_coeffs(&self) -> &[N; SIZE] {
        &self.coeffs
    }

    // Gets size of coeffs or P of Poly
    #[inline]
    pub fn len(&self) -> usize {
        self.coeffs.len()
    }
}

impl<N, const SIZE: usize> PolyInt<N, SIZE>
where
    N: Sized + One + Zero + PartialOrd<N> + Neg<Output = N>,
{
    pub fn equals_zero(&self) -> bool {
        for item in self.coeffs.iter() {
            if *item == N::zero() {
                continue;
            } else {
                return false;
            }
        }

        true
    }

    pub fn is_small(&self) -> bool {
        self.coeffs
            .iter()
            .all(|value| *value <= N::one() && *value >= -N::one())
    }
}

impl<N, const SIZE: usize> PartialEq for PolyInt<N, SIZE>
where
    N: Zero + PartialEq + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.get_coeffs() == other.get_coeffs()
    }
}

impl<N, const SIZE: usize> PolyInt<N, SIZE>
where
    N: Sized
        + Default
        + Zero
        + One
        + Copy
        + Neg<Output = N>
        + Mul<Output = N>
        + AddAssign
        + Euclid
        + ToPrimitive
        + FromPrimitive
        + TryFrom,
{
    // TODO: Need stack and thread optimisation.
    // TODO: Add Result.
    pub fn mult_poly(&mut self, a: &PolyInt<N, SIZE>, b: &PolyInt<N, SIZE>) {
        let mut fg = vec![N::zero(); SIZE * 2 - 1];

        for i in 0..SIZE {
            let mut r = N::zero();

            for j in 0..=i {
                r = self.plus(r, a.coeffs[j], b.coeffs[i - j]);
            }

            fg[i] = r;
        }

        for i in SIZE..(SIZE * 2 - 1) {
            let mut r = N::zero();

            for j in (i - SIZE + 1)..SIZE {
                r = self.plus(r, a.coeffs[j], b.coeffs[i - j]);
            }

            fg[i] = r;
        }

        for i in (SIZE..(SIZE * 2) - 1).rev() {
            let a32 = N::to_i32(&fg[i - SIZE]).expect("a: cannot convert to i32");
            let b32 = N::to_i32(&fg[i]).expect("b: cannot convert to i32");
            let tmp1 = self.freeze(a32 + b32);

            fg[i - SIZE] = tmp1;

            let a32 = N::to_i32(&fg[i - SIZE + 1]).expect("a: cannot convert to i32");
            let b32 = N::to_i32(&fg[i]).expect("b: cannot convert to i32");
            let tmp2 = self.freeze(a32 + b32);

            fg[i - SIZE + 1] = tmp2;
        }

        self.coeffs[..SIZE].clone_from_slice(&fg[..SIZE]);
    }

    pub fn mult_int(&mut self, n: N) {
        for i in 0..SIZE {
            self.coeffs[i] = self.coeffs[i] * n;
        }
    }

    // pub fn inv(&self) -> PolyInt<N, SIZE> {
    //     // const loops = N::us
    //     let loops = 2 * SIZE + 1;
    //     let mut r = [N::zero(); SIZE];
    //     let mut f = vec![N::zero(); SIZE + 1];
    //
    //     f[0] = -N::one();
    //     f[1] = -N::one();
    //     f[761] = N::one();
    //
    //     let mut g = vec![N::zero(); SIZE + 1];
    //
    //     for i in 0..SIZE {
    //         g[i] = 3 * s[i];
    //     }
    //
    //     let mut d = 761;
    //     let mut e = 761;
    //     let mut u = [0i16; LOOPS + 1];
    //     let mut v = [0i16; LOOPS + 1];
    //
    //     v[0] = 1;
    //
    //     for _ in 0..LOOPS {
    //         let c = modq::quotient(g[761], f[761]);
    //         vector::minus_product(&mut g, 761 + 1, &f, c);
    //         vector::shift(&mut g, 761 + 1);
    //         vector::minus_product(&mut v, LOOPS + 1, &u, c);
    //         vector::shift(&mut v, LOOPS + 1);
    //         e -= 1;
    //         let m = smaller_mask(e, d) & modq::mask_set(g[761]);
    //         let (e_tmp, d_tmp) = swap_int(e, d, m);
    //         e = e_tmp;
    //         d = d_tmp;
    //         vector::swap(&mut f, &mut g, 761 + 1, m);
    //         vector::swap(&mut u, &mut v, LOOPS + 1, m);
    //     }
    //
    //     vector::product(&mut r, 761, &u[761..], modq::reciprocal(f[761]));
    //
    //     smaller_mask(0, d);
    //
    //     r;
    //
    //     PolyInt::from(r)
    // }

    fn plus(&self, a: N, b: N, c: N) -> N {
        let a32 = N::to_i32(&a).expect("a: cannot convert to i32");
        let b32 = N::to_i32(&b).expect("b: cannot convert to i32");
        let c32 = N::to_i32(&c).expect("c: cannot convert to i32");

        self.freeze(a32 + b32 * c32)
    }

    fn freeze(&self, value: i32) -> N {
        let bs = value - (3 * ((10923 * value) >> 15));
        let bc = bs - (3 * ((89_478_485 * bs + 134_217_728) >> 28));

        N::try_from_i32(bc).expect("i32 convert overflow")
    }
}

#[cfg(test)]
mod test_poly_v2 {
    use super::*;

    #[test]
    fn test_init_from_arr() {
        let a = PolyInt::from([1, 2, 3]);

        assert_eq!(a.get_coeffs(), &[1, 2, 3]);
    }

    #[test]
    fn test_init_zeros() {
        let a: PolyInt<u8, 3> = PolyInt::new();

        assert_eq!(a.len(), 3);
    }

    #[test]
    fn test_is_small() {
        let coefficients_big = [0, -1, -2, 2];
        let coefficients_small = [0, -1, -1, 1];

        let poly = PolyInt::from(coefficients_big);

        assert!(!poly.is_small());

        let poly = PolyInt::from(coefficients_small);
        assert!(poly.is_small());
    }

    #[test]
    fn test_is_zeros() {
        let coeffs = [0; 716];
        let mut poly = PolyInt::from(coeffs);

        assert!(poly.equals_zero());

        poly.coeffs[1] = 1;

        assert!(!poly.equals_zero());

        poly.coeffs[1] = -1;

        assert!(!poly.equals_zero());
    }

    #[test]
    fn test_mult_poly_int() {
        let expected_result = [1 * 3, -1 * 3, 0 * 3, -1 * 3, 1 * 3];
        let mut poly = PolyInt::from([1, -1, 0, -1, 1]);

        poly.mult_int(3);

        assert_eq!(poly.get_coeffs(), &expected_result);
    }

    #[test]
    fn test_mult_poly() {
        let coeffs = [0i16; 761];
        let mut h = PolyInt::from(coeffs);
        let g: PolyInt<i16, 761> = PolyInt::from([
            1, 0, -1, 1, 0, 0, 0, -1, 1, 1, 1, 0, -1, -1, 1, -1, -1, -1, -1, -1, 0, -1, -1, 0, 1,
            1, -1, 0, 0, 1, 0, 0, 1, -1, 0, 1, -1, 0, 0, 1, -1, -1, 0, 0, 1, 1, 1, 0, -1, -1, 1,
            -1, 1, -1, 1, 1, -1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, -1, 0, 0, 0, 0, 1, 1, -1, -1, -1, 1,
            -1, -1, -1, -1, 0, 0, 0, -1, 0, -1, 0, -1, 1, 0, -1, 0, -1, 1, 0, -1, 0, 1, 0, -1, -1,
            1, 0, 0, 1, -1, -1, 1, -1, 1, 1, -1, 1, 1, 0, -1, 1, 1, 1, 1, 1, -1, -1, -1, 1, 1, 0,
            1, -1, -1, 0, 1, -1, -1, 1, -1, -1, 1, 1, 1, 1, 0, 0, 1, 0, -1, 0, -1, -1, -1, -1, -1,
            0, 0, -1, -1, -1, 1, -1, 1, 1, -1, -1, -1, 1, 0, 1, 0, 1, 0, -1, 1, -1, -1, 1, 1, 0,
            -1, 0, 1, -1, 1, -1, -1, 0, 1, -1, 1, 0, -1, 1, -1, -1, 0, -1, -1, 1, 0, 1, -1, 0, 0,
            -1, -1, 0, -1, 1, 0, 1, -1, 1, -1, 1, 1, 0, -1, -1, -1, 0, 0, -1, 1, -1, 0, 0, 1, -1,
            -1, 0, -1, 0, 1, 0, -1, 1, -1, 1, -1, -1, 1, -1, 0, 0, 1, 1, -1, 1, 1, 1, 0, 0, -1, 1,
            -1, 1, 1, 0, -1, 1, -1, 1, -1, -1, 0, 0, 1, 0, 0, 0, -1, -1, 1, -1, 1, 1, 1, -1, -1, 1,
            1, 1, 0, -1, -1, 1, 1, 0, -1, -1, -1, -1, 1, 0, -1, 1, 0, 1, 0, 1, 1, -1, 1, -1, 1, 1,
            -1, -1, -1, 1, -1, 1, -1, -1, 0, 1, 1, 1, -1, 0, 0, -1, 0, 0, 1, -1, 1, -1, 0, 1, 1, 1,
            1, -1, -1, 0, -1, 1, -1, 1, -1, 0, -1, -1, 0, 1, -1, 1, 1, 0, 0, 1, 0, 1, 1, -1, 1, -1,
            0, -1, 1, 0, 1, 0, 0, 1, -1, 1, 0, -1, -1, 0, -1, 1, 1, 1, -1, -1, 1, -1, 1, -1, -1, 1,
            -1, 1, 1, -1, 0, -1, 1, -1, -1, 1, 0, 0, 0, 1, -1, 1, 0, 0, -1, 0, 0, 1, 0, 1, 0, 1, 1,
            0, 0, 0, 1, 0, 1, 1, -1, 1, -1, 1, -1, -1, 1, -1, 1, 1, 1, 0, -1, 1, 1, 0, 1, 0, -1,
            -1, -1, 0, 1, -1, 1, 0, -1, -1, 1, 0, 0, -1, 0, -1, 0, -1, -1, 1, -1, 1, -1, 1, -1, -1,
            0, 1, 0, -1, 0, 1, -1, 1, 1, -1, 0, -1, -1, 1, 0, -1, 0, 0, 0, 1, 1, 1, 0, -1, 0, 1,
            -1, -1, 0, 0, 0, 0, -1, 0, 0, 1, -1, -1, 0, 0, 1, 0, 0, -1, 1, 0, -1, -1, 1, -1, 1, 1,
            -1, -1, 1, 1, 0, 1, 1, 0, 1, -1, 1, -1, 1, 1, -1, 0, 1, 0, 1, 0, -1, 0, 0, 0, 0, 1, -1,
            -1, 0, 1, -1, 0, 1, 1, 1, 0, -1, -1, -1, -1, 0, 0, 0, 0, 0, 1, -1, -1, 1, 0, 0, 0, 1,
            -1, 0, -1, -1, -1, 0, 0, 0, 1, -1, -1, 1, -1, -1, 0, -1, 1, 0, -1, 0, 1, 0, -1, 0, 0,
            0, 0, 0, 0, 0, 1, 0, -1, 1, -1, 0, -1, -1, -1, 1, 0, -1, -1, -1, 0, 0, -1, -1, -1, 0,
            -1, -1, 0, -1, 0, 1, 1, 0, -1, 1, -1, 1, -1, 1, -1, 1, 0, 1, 1, 0, 1, -1, 0, 1, 0, 1,
            0, -1, -1, -1, 0, -1, -1, 0, 0, 1, 1, -1, -1, 0, -1, 1, -1, 0, 1, 1, 1, -1, 0, 0, -1,
            0, 1, -1, 1, -1, -1, -1, 0, 1, 0, -1, 1, 1, 0, 0, -1, -1, 0, -1, -1, 1, 1, -1, 1, 0, 1,
            1, 1, -1, -1, 1, 0, -1, 1, 1, -1, -1, 1, 1, 1, -1, 0, 0, -1, -1, -1, 1, 0, 1, -1, 0, 1,
            1, 0, 0, 1, 0, 1, 1, 1, -1, 1, 1, -1, 1, -1, 0, 0, -1, -1, -1, -1, -1, 1, 1, -1, -1, 0,
            -1, -1, -1, 1, 1, 1, 0, -1, -1, 1, -1, 1, 1, 0,
        ]);
        let f3r: PolyInt<i16, 761> = PolyInt::from([
            -3, 3, 3, 0, 0, 0, 0, -3, 0, -3, 0, 0, 0, 0, -3, 0, 0, 3, 3, -3, -3, 0, 0, 0, 3, 3, 0,
            -3, 3, 0, -3, 0, 0, 3, 0, 0, 0, 0, -3, 3, 0, 3, 0, 0, 0, 0, 0, 0, 3, 3, 0, 0, -3, 3,
            -3, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, -3, 0, 3, 0, 0, 0, 0, -3, -3, -3, 0, 0, 0,
            0, 3, 0, 0, 0, 0, 0, 0, 0, 0, -3, 3, -3, 0, 0, 3, 0, 0, 0, 0, 0, 0, -3, 0, 3, 3, 0, 0,
            -3, 0, -3, 0, 0, 0, 0, 0, 0, 3, -3, 0, 0, 0, 3, 0, 3, 0, -3, 0, -3, 3, 3, -3, 0, -3, 3,
            -3, 3, -3, 0, 0, 0, 0, 3, 0, 0, -3, 0, 0, 0, 0, 0, -3, 0, 3, 0, 0, 0, -3, 0, 3, 0, 0,
            -3, 3, 0, -3, 0, 0, 0, 0, 0, 0, 0, -3, 0, 0, 0, 0, 0, -3, 0, 0, 0, 3, 0, 0, 3, 0, 0, 0,
            0, 0, 0, 0, 0, 0, -3, -3, 0, 0, 0, 3, 3, 0, 0, 3, -3, 0, 0, -3, 0, 3, 0, 0, -3, 0, 0,
            3, -3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 3, -3, 0, 0, 0, 0, 0, -3, 0, 0, 3, -3, 0, -3, 0,
            0, 0, 0, -3, -3, 0, -3, -3, 0, 0, -3, -3, 0, 3, 0, 0, 0, -3, 0, 0, 3, 0, 3, 0, 3, 0, 0,
            0, 0, 0, 0, -3, 0, 0, 0, 0, 3, 0, 3, 3, 0, 0, 0, -3, -3, 0, 0, -3, -3, 3, -3, 0, 0, 0,
            -3, 0, 0, 3, -3, 3, 0, 0, 0, -3, -3, 0, -3, 0, 0, 0, -3, -3, 0, 0, -3, 0, 0, -3, 0, 0,
            0, 3, 0, 0, 0, 3, 0, 0, -3, 0, -3, 0, 3, -3, 3, -3, 0, -3, 0, 0, -3, 0, 0, 0, 0, 0, 3,
            3, 3, -3, 0, -3, 0, -3, 0, 0, 0, 0, 0, -3, 0, 0, 3, 3, 3, 3, 0, 0, 0, 3, 0, -3, 0, 0,
            0, 0, 0, 0, 3, 0, 0, -3, 3, 3, 0, 0, 3, 0, 0, 3, -3, 0, 3, 0, -3, -3, -3, 0, 0, 0, 0,
            3, 3, 0, 0, 0, 0, 0, 0, 0, 0, -3, 0, 0, 3, 0, 0, 0, 0, 3, -3, 0, -3, 0, -3, -3, 0, -3,
            -3, 0, -3, 3, 0, 0, 3, 0, 0, 0, 0, 0, 0, -3, 0, 0, -3, 0, 0, 0, 0, 0, 0, -3, -3, 0, -3,
            0, 0, 0, 0, -3, 0, 0, 3, 3, 0, 3, 0, 3, 0, 0, 0, 0, -3, 0, 0, 0, -3, 0, 3, 0, -3, 0, 0,
            0, 0, 0, 0, 3, 3, -3, 0, 0, 3, 0, 0, -3, 0, 3, 0, 0, 3, 0, -3, -3, -3, 0, 0, 3, 0, 0,
            0, 0, 0, 0, 3, -3, -3, 3, 0, 3, 0, 0, -3, -3, 3, 0, 0, 3, -3, 0, 3, 0, 0, 0, 0, 3, 0,
            0, 0, 3, -3, 0, 0, 3, 3, 3, 0, -3, -3, 0, 3, 0, 0, 0, 3, 0, 0, 3, 0, 3, 0, -3, -3, 0,
            0, -3, 0, 0, -3, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, -3, -3, -3, 0, 0,
            0, 3, 0, -3, 0, 0, 0, 3, -3, 0, 0, 0, -3, 0, 3, 0, -3, 0, 0, -3, -3, 0, 3, 0, 0, 0, -3,
            0, -3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 3, -3, 0, 0, 0, 3, 3, 0, 0, -3, -3,
            0, 3, -3, 0, 0, 0, 3, 0, -3, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, -3, -3, 3, -3, 0, 3, 0, 0,
            3, -3, 0, 3, 0, -3, 3, 0, 0, 3, -3, 3, 0, -3, 0, -3, 3, -3, 3, 0, 0, 0, 0, -3, -3, 0,
            3, -3, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 3, 0, 0, 0, -3, 3, 3, 3, 0, 0, 0,
            0, 0, 0, 0, 3, 0, 0, 0, 3, 3, -3, 0, 0, 0, 0, 0, -3, 0, -3, -3, 0, 0, 0, 3, -3, 0, 3,
            0, -1,
        ]);

        h.mult_poly(&f3r, &g);

        assert_eq!(
            h.coeffs,
            [
                0, 1, 0, -1, 0, 0, 1, 0, 1, 1, -1, 1, -1, 0, 0, -1, -1, -1, -1, 1, 1, -1, 1, -1, 1,
                0, 1, 0, -1, -1, 0, -1, 0, 1, -1, 0, 1, 0, -1, 0, -1, 1, 0, -1, 1, 1, -1, 1, -1, 0,
                0, 0, 0, 0, 1, 0, 1, -1, -1, -1, 1, 1, -1, -1, 1, 1, 0, 1, 0, 0, 0, -1, 1, 0, -1,
                -1, 0, 0, -1, -1, -1, 1, 0, 0, 1, 1, 1, 1, 1, 0, -1, 1, 1, 1, 0, -1, 1, 1, -1, -1,
                1, -1, 0, -1, 0, -1, 0, -1, 0, 0, 0, 1, 0, 0, 1, -1, 1, 0, 1, 1, 1, 1, 0, -1, -1,
                0, 1, -1, -1, 0, -1, 1, -1, 0, -1, 0, 0, -1, 0, 1, 1, 1, -1, 0, -1, -1, 1, 1, 1,
                -1, -1, -1, -1, 1, 0, 1, -1, -1, 0, 0, 0, 1, 0, -1, -1, 0, -1, -1, -1, -1, -1, 1,
                0, 0, -1, 0, 1, -1, 1, 1, -1, 0, 0, 0, -1, 1, -1, 0, 0, -1, 1, 0, 0, -1, 1, 1, -1,
                0, -1, -1, 0, 1, 0, 1, -1, 1, 1, 0, -1, -1, 0, 0, 0, 0, 1, -1, 1, -1, -1, 1, 0, 1,
                0, 0, 1, 0, -1, 0, -1, 1, 1, 1, -1, -1, 1, 0, 0, 0, 0, -1, 0, 0, 1, 0, -1, 1, 0, 0,
                1, 1, -1, 0, 1, 0, 0, 0, 1, -1, 1, 0, 0, 0, 0, -1, 1, 0, -1, -1, 0, 0, 1, -1, 0, 0,
                0, 1, 1, 0, -1, 0, 1, 1, -1, 1, -1, 0, 1, -1, 1, -1, -1, -1, 0, -1, 1, 0, -1, -1,
                -1, -1, 1, 0, 0, 0, 0, 1, 0, -1, -1, 0, 0, 0, 0, -1, 1, -1, 1, 1, 0, 1, 0, 1, 1, 0,
                -1, 0, 0, 0, 1, -1, 1, 1, 1, 0, -1, 1, 1, 0, 0, 0, 0, 1, 1, -1, 1, -1, 0, 0, 1, -1,
                0, -1, -1, -1, 1, 0, 0, 0, 1, 1, 0, -1, -1, -1, 0, -1, 0, 0, -1, 1, -1, 1, 1, 0, 1,
                1, 0, -1, 0, 0, 0, 0, -1, 0, 0, 0, 1, 0, 1, 1, 0, 0, -1, 0, -1, 0, 0, -1, 0, 0, -1,
                0, 1, 1, 0, -1, -1, -1, -1, -1, 1, -1, 0, 0, -1, -1, -1, 1, 0, 0, 0, 0, 0, -1, 0,
                0, 0, 1, 1, -1, 1, 0, 1, -1, -1, -1, 1, -1, -1, 1, -1, 0, 0, -1, 1, -1, 0, -1, 0,
                1, 1, 1, 1, 1, -1, 0, 0, 0, 0, 0, 0, -1, 1, -1, -1, 1, 1, -1, 0, 0, 1, 0, 1, 1, -1,
                0, -1, 1, 1, 0, 0, -1, 1, 1, -1, 1, 1, -1, 0, -1, 1, 0, 0, 0, 1, 1, 0, -1, 0, -1,
                1, 0, -1, -1, 0, 1, 0, -1, 1, -1, 0, 0, 0, 1, 0, -1, 0, 1, -1, -1, 1, -1, -1, 0, 0,
                0, 0, 1, 0, 1, -1, -1, -1, -1, 1, 1, 0, 0, 0, -1, 0, -1, 1, -1, 0, 1, -1, 1, 1, -1,
                1, -1, -1, -1, 1, 0, 0, 0, 0, -1, 0, -1, 0, -1, 0, 0, -1, 0, 1, 1, -1, -1, 1, 0, 0,
                -1, 0, -1, 0, 0, -1, 1, 1, 0, -1, 1, 1, -1, -1, 1, 1, 0, 0, 0, 0, 0, 0, -1, -1, 1,
                0, 0, 1, 1, -1, -1, 0, -1, 1, -1, -1, 1, 0, 1, -1, -1, 1, 1, -1, 1, 1, 1, -1, 1,
                -1, 1, 0, 0, 0, 0, 0, 0, 0, -1, -1, 1, -1, -1, 0, 1, -1, -1, -1, -1, 1, -1, -1, 1,
                1, -1, 1, 0, -1, 1, 0, -1, 1, 1, 0, 0, 1, -1, 1, 1, 0, 1, 0, 1, 1, -1, 0, 0, 0, -1,
                -1, 1, -1, -1, 1, 0, 1, -1, 0, 1, -1, 1, 1, -1, 0, 1, 0, 0, -1, -1, 1, 1, 0, -1, 0,
                -1, 1, 0, 1, 0, -1, 0, 1, 1, 0, 1, 0, 1, -1, -1, 0, -1, -1, 0, 1, -1, 1, -1, 0, -1,
                -1, -1, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1, -1, -1, -1, -1, 0, 1, 0, -1, 1, 1, -1, -1,
                0, 1, 1, -1, 1, -1, 0, 0, 0, 1, -1, -1,
            ]
        );
    }
}
