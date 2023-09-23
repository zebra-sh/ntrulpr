#[cfg(feature = "ntrulpr1013")]
use crate::params::params1013::{Q, Q12};
#[cfg(feature = "ntrulpr1277")]
use crate::params::params1277::{Q, Q12};
#[cfg(feature = "ntrulpr653")]
use crate::params::params653::{Q, Q12};
#[cfg(feature = "ntrulpr761")]
use crate::params::params761::{Q, Q12};
#[cfg(feature = "ntrulpr857")]
use crate::params::params857::{Q, Q12};
#[cfg(feature = "ntrulpr953")]
use crate::params::params953::{Q, Q12};

use crate::math::nums::i32_mod_u14;

pub fn freeze(x: i32) -> i16 {
    let r = i32_mod_u14(x + Q12 as i32, Q as u16);

    r as i16 - Q12 as i16
}

pub fn recip(a1: i16) -> i16 {
    let mut i = 1;
    let mut ai = a1;

    while i < Q - 2 {
        ai = freeze((a1 as i32) * (ai as i32)) as i16;
        i += 1;
    }

    ai
}

#[cfg(test)]
mod tests_fq {
    use super::*;

    #[cfg(feature = "ntrulpr653")]
    const RQ_FREEZE: i32 = 58_078;
    #[cfg(feature = "ntrulpr761")]
    const RQ_FREEZE: i32 = 58_470;
    #[cfg(feature = "ntrulpr857")]
    const RQ_FREEZE: i32 = 51_949;
    #[cfg(feature = "ntrulpr953")]
    const RQ_FREEZE: i32 = 42_319;
    #[cfg(feature = "ntrulpr1013")]
    const RQ_FREEZE: i32 = 37_402;
    #[cfg(feature = "ntrulpr1277")]
    const RQ_FREEZE: i32 = 34_069;

    #[test]
    fn test_freeze() {
        use rand::prelude::*;

        let mut rng = thread_rng();

        fn test_freeze(a: i32) -> i16 {
            let mut b = a;
            let q = Q as i32;

            b -= q * ((228 * b) >> 20);
            b -= q * ((RQ_FREEZE * b + 134_217_728) >> 28);

            b as i16
        }

        for _ in i16::MIN..i16::MAX {
            let r = rng.gen::<i16>() as i32;

            let t1 = test_freeze(r);
            let t2 = freeze(r);

            assert_eq!(t2, t1);
        }
    }

    #[test]
    fn test_recip() {
        assert_eq!(recip(42), recip(-42) * -1);
        assert_eq!(recip(-42), recip(42) * -1);
    }
}
