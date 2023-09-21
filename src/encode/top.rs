#[cfg(feature = "ntrulpr1013")]
use crate::params::params1013::{I, INPUTS_BYTES, P, TAU0, TAU1, TOP_BYTES};
#[cfg(feature = "ntrulpr1277")]
use crate::params::params1277::{I, INPUTS_BYTES, P, TAU0, TAU1, TOP_BYTES};
#[cfg(feature = "ntrulpr653")]
use crate::params::params653::{I, INPUTS_BYTES, P, TAU0, TAU1, TOP_BYTES};
#[cfg(feature = "ntrulpr761")]
use crate::params::params761::{I, INPUTS_BYTES, P, TAU0, TAU1, TOP_BYTES};
#[cfg(feature = "ntrulpr857")]
use crate::params::params857::{I, INPUTS_BYTES, P, TAU0, TAU1, TOP_BYTES};
#[cfg(feature = "ntrulpr953")]
use crate::params::params953::{I, INPUTS_BYTES, P, TAU0, TAU1, TOP_BYTES};

pub fn top_encode<const SIZE: usize, const START: usize>(s: &mut [u8; SIZE], t: &[i8; I]) {
    for i in 0..TOP_BYTES {
        let v = t[2 * i] + (t[2 * i + 1] << 4);

        s[i + START] = v as u8;
    }
}

pub fn top_decode<const SIZE: usize>(t: &mut [i8; I], s: &[u8; SIZE]) {
    for i in 0..TOP_BYTES {
        t[2 * i] = (s[i] & 15) as i8;
        t[2 * i + 1] = (s[i] >> 4) as i8;
    }
}

pub fn top(c: i16) -> i8 {
    let tau0 = TAU0 as i32;
    let tau1 = TAU1 as i32;
    let c32 = c as i32;
    let value = (tau1 * (c32 + tau0) + 16384) >> 15;

    value as i8
}
pub fn inputs_encode(r: &[i8; P]) -> [u8; INPUTS_BYTES] {
    let mut s = [0u8; INPUTS_BYTES];

    for i in 0..I {
        s[i >> 3] |= (r[i] << (i & 7)) as u8;
    }

    s
}

#[cfg(test)]
mod tests_top {
    use super::*;

    #[test]
    fn test_inputs_encode() {
        let r: [i8; P] = [
            1, 0, 0, 0, 1, 0, 0, 1, -1, -1, -1, 0, 0, -1, 0, 1, -1, 1, -1, -1, 1, 1, 1, -1, -1, -1,
            -1, 0, 1, 1, 1, -1, -1, 1, 1, 0, 0, -1, -1, 1, -1, 0, -1, -1, -1, 1, 0, 0, 1, 0, 0, -1,
            1, -1, 0, 1, -1, 0, 0, -1, 1, 0, 1, -1, 1, 1, 0, 0, 1, -1, 1, 0, 0, -1, 0, 1, -1, 1, 0,
            0, -1, 0, 0, 0, 0, -1, -1, 1, 1, 0, 0, 1, -1, 0, -1, 0, 0, 0, 1, 1, 1, 0, -1, 0, -1, 0,
            -1, -1, 0, -1, 1, 1, 1, 1, -1, 0, 0, 0, -1, -1, 0, 0, 0, 0, 0, -1, -1, 1, -1, 0, 0, 0,
            -1, -1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, -1, -1, -1, -1, 1, -1, 0, -1, 1, -1, -1, 0,
            1, -1, 0, 1, -1, -1, 1, 0, 0, 1, 1, 1, 1, -1, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, -1, 0, -1,
            -1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 1, 0, 0, -1, 1, -1, 1, -1, 1, -1, 1, 0, 1, 1,
            0, -1, 1, -1, 1, 0, -1, 0, 0, 1, 1, -1, -1, 0, 1, -1, -1, 0, 1, -1, 0, -1, 0, -1, -1,
            0, -1, 1, 1, -1, 1, 0, 1, 1, 1, 1, 0, 0, -1, 1, -1, 1, 1, 1, -1, -1, 0, 1, 1, -1, 0, 1,
            -1, -1, 1, -1, -1, 1, 1, -1, 1, -1, -1, 0, -1, -1, 0, -1, -1, -1, -1, -1, 0, 1, 0, -1,
            -1, 0, -1, 0, 0, 1, 1, 0, -1, 1, 0, 0, -1, 0, 1, 1, 0, -1, 0, 1, 1, 0, 0, -1, 1, 1, 0,
            1, -1, 0, 0, 1, 0, 0, -1, -1, 1, 0, -1, -1, -1, 1, 1, 0, 1, 0, -1, -1, -1, 1, -1, -1,
            0, -1, -1, 1, 1, -1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, -1, 1, -1, -1, 0, 1, 1, -1, 0, 1,
            -1, -1, -1, 1, 1, -1, -1, 1, 1, 0, -1, 0, 0, -1, -1, -1, 1, 0, -1, 1, -1, 1, 1, -1, -1,
            0, -1, 0, -1, 1, 1, 1, 0, -1, 1, -1, 0, 0, 1, 1, 1, -1, -1, -1, -1, 1, 1, -1, 0, 0, 1,
            1, 0, 0, 0, -1, 1, -1, -1, -1, 1, 1, 0, -1, -1, 0, 1, 1, 0, 0, 1, -1, 1, 1, 1, 1, -1,
            1, 0, 0, 1, 1, 0, -1, -1, 1, -1, 0, 1, -1, -1, 1, 0, -1, 0, 0, -1, 0, -1, -1, 1, 0, 1,
            0, 0, 0, -1, -1, 1, 1, -1, 1, 0, 1, -1, 1, 1, 1, -1, 0, 0, 0, 0, -1, -1, 0, 1, 0, 1, 1,
            -1, 0, 0, 1, -1, 1, -1, 0, -1, 1, 0, 0, 1, 1, 1, -1, 0, -1, -1, 0, -1, 0, 0, -1, 1, -1,
            1, -1, 0, 0, -1, 1, 0, -1, 0, 1, 0, 0, -1, 0, -1, 0, 0, 1, 1, -1, -1, 1, 1, 1, 1, -1,
            1, 1, 1, 1, 1, 0, 0, -1, 1, 1, 1, 1, 0, -1, 1, 1, -1, 0, -1, 1, -1, 1, 0, 1, -1, 1, 1,
            -1, 0, 1, 1, -1, 0, -1, -1, 1, 1, 0, -1, -1, 0, 1, 0, -1, 1, 1, 1, 1, 1, 1, -1, 0, 0,
            1, -1, 1, 0, -1, 1, 0, 1, 0, 1, 0, 0, 0, 1, -1, 1, 0, -1, 1, 0, 0, -1, -1, 0, -1, 0, 1,
            0, 0, 0, -1, 0, 1, 1, 1, 1, 1, 0, 1, -1, 1, -1, -1, -1, -1, 1, 1, -1, 0, 0, -1, 0, 0,
            -1, 1, -1, 1, 0, 0, 0, 0, 0, 1, 0, -1, -1, -1, -1, 0, -1, -1, -1, -1, 0, 0, -1, 0, 0,
            -1, -1, -1, 1, -1, -1, 1, 0, -1, -1, -1, 0, 1, 0, -1, 0, -1, -1, 0, 0, -1, 0, -1, -1,
            0, 0, -1, 1, 1, 0, -1, 0, -1, -1, -1, 0, 0, -1, -1, -1, 0, 0, 0, 1, -1, 0, 1, -1, 1, 1,
            -1, -1, 1, 0, -1, -1, -1, 0, 0, 1, -1, 1, -1, 0, -1, 1, -1, 0, 0, 1, 1, 0, -1, -1, -1,
            1, 1, -1, -1, 1, -1,
        ];
        let out = inputs_encode(&r);

        assert_eq!(
            out,
            [
                145, 255, 255, 255, 255, 255, 249, 255, 243, 254, 255, 249, 220, 255, 255, 224,
                255, 143, 252, 254, 254, 255, 229, 255, 94, 254, 237, 253, 255, 254, 255, 252,
            ]
        );
    }

    #[test]
    fn test_top() {
        assert_eq!(top(4325), 23);
        assert_eq!(top(0), 8);
        assert_eq!(top(-30), 7);
        assert_eq!(top(i16::MAX), 121);
        assert_eq!(top(i16::MIN), -106);
    }

    #[test]
    fn test_top_encode() {
        let mut s: [u8; 1039] = [
            56, 34, 181, 213, 105, 213, 157, 187, 54, 231, 40, 167, 152, 249, 125, 74, 129, 229,
            62, 142, 142, 230, 250, 193, 181, 147, 100, 236, 65, 151, 171, 89, 230, 6, 232, 118,
            139, 207, 233, 119, 184, 151, 50, 143, 181, 37, 229, 254, 222, 214, 12, 178, 160, 114,
            50, 113, 155, 192, 220, 30, 214, 162, 159, 113, 191, 75, 15, 53, 70, 206, 120, 198,
            176, 230, 136, 223, 169, 84, 87, 182, 65, 11, 121, 136, 3, 226, 177, 142, 183, 147, 58,
            158, 8, 88, 255, 241, 243, 69, 159, 103, 118, 157, 242, 255, 19, 121, 66, 67, 136, 124,
            100, 238, 77, 41, 206, 253, 110, 231, 186, 16, 88, 116, 242, 188, 211, 38, 37, 97, 220,
            172, 216, 207, 63, 102, 53, 34, 196, 170, 188, 224, 56, 233, 97, 191, 12, 85, 251, 101,
            89, 162, 176, 25, 244, 97, 159, 39, 5, 62, 150, 78, 32, 212, 158, 25, 192, 11, 90, 155,
            190, 165, 172, 75, 123, 205, 96, 151, 138, 140, 168, 207, 113, 172, 139, 234, 86, 159,
            4, 160, 181, 104, 252, 38, 98, 128, 115, 20, 164, 152, 163, 254, 108, 254, 29, 206,
            237, 118, 90, 43, 87, 51, 165, 179, 192, 254, 217, 237, 106, 29, 223, 222, 19, 185,
            203, 85, 7, 43, 176, 185, 10, 34, 109, 66, 51, 68, 221, 163, 33, 220, 46, 140, 184,
            235, 7, 91, 112, 12, 56, 243, 74, 70, 61, 14, 231, 223, 49, 73, 229, 109, 89, 131, 163,
            26, 60, 163, 74, 58, 16, 255, 183, 215, 4, 249, 118, 26, 34, 152, 206, 250, 151, 27,
            70, 115, 92, 163, 254, 104, 18, 98, 171, 136, 225, 219, 176, 221, 184, 16, 213, 187,
            236, 188, 64, 101, 226, 51, 135, 99, 198, 182, 26, 74, 50, 181, 219, 133, 122, 179,
            140, 71, 67, 157, 158, 77, 136, 116, 141, 189, 179, 54, 226, 217, 139, 217, 85, 178,
            187, 87, 228, 200, 123, 90, 55, 32, 138, 219, 197, 230, 1, 0, 97, 9, 90, 247, 172, 185,
            231, 62, 165, 38, 133, 21, 153, 117, 161, 176, 224, 63, 229, 137, 131, 39, 116, 32, 10,
            139, 48, 209, 239, 159, 48, 39, 44, 56, 6, 133, 184, 111, 194, 248, 165, 237, 30, 149,
            216, 199, 34, 212, 189, 181, 63, 3, 93, 45, 104, 245, 75, 88, 27, 116, 184, 146, 203,
            59, 31, 35, 249, 183, 59, 194, 250, 42, 140, 182, 245, 156, 14, 226, 100, 141, 32, 62,
            128, 38, 108, 82, 189, 179, 232, 193, 16, 67, 210, 48, 171, 215, 210, 58, 208, 85, 147,
            190, 225, 151, 57, 65, 129, 145, 74, 186, 66, 73, 255, 60, 93, 112, 146, 228, 159, 11,
            56, 198, 225, 194, 250, 62, 223, 141, 35, 252, 163, 216, 41, 233, 59, 122, 61, 252, 8,
            38, 58, 255, 238, 119, 112, 159, 83, 236, 56, 231, 32, 77, 29, 172, 166, 169, 115, 13,
            178, 40, 100, 135, 83, 211, 114, 220, 129, 213, 8, 156, 207, 28, 29, 73, 35, 106, 156,
            225, 130, 207, 106, 206, 212, 148, 139, 175, 76, 102, 242, 226, 152, 111, 146, 45, 58,
            190, 188, 121, 38, 222, 104, 66, 229, 14, 16, 60, 59, 234, 11, 185, 47, 52, 136, 47, 7,
            190, 251, 31, 216, 158, 95, 5, 110, 254, 151, 9, 36, 229, 110, 205, 241, 244, 19, 166,
            49, 57, 175, 240, 191, 140, 113, 85, 187, 61, 61, 226, 105, 96, 251, 246, 201, 207,
            188, 159, 101, 218, 101, 142, 179, 127, 37, 28, 220, 244, 52, 62, 14, 48, 247, 251,
            216, 178, 146, 180, 21, 48, 62, 222, 87, 187, 0, 207, 171, 13, 230, 231, 30, 37, 134,
            38, 225, 190, 148, 45, 240, 198, 63, 155, 175, 84, 240, 17, 36, 116, 200, 179, 12, 143,
            53, 183, 137, 240, 35, 76, 164, 81, 161, 191, 53, 193, 122, 227, 235, 218, 89, 152, 78,
            21, 164, 5, 137, 137, 119, 129, 64, 195, 10, 218, 211, 190, 218, 94, 241, 116, 65, 219,
            168, 133, 139, 241, 152, 54, 159, 44, 111, 90, 198, 191, 141, 158, 119, 245, 222, 65,
            146, 170, 147, 68, 217, 157, 47, 179, 25, 239, 105, 61, 115, 188, 223, 13, 189, 27,
            134, 28, 6, 51, 228, 139, 172, 104, 121, 66, 35, 117, 114, 125, 245, 6, 16, 23, 161,
            170, 61, 188, 0, 156, 29, 222, 91, 168, 224, 53, 101, 119, 67, 44, 86, 30, 42, 38, 35,
            111, 197, 148, 251, 174, 195, 161, 204, 2, 84, 128, 224, 35, 227, 174, 68, 255, 84,
            231, 255, 209, 227, 161, 114, 154, 238, 170, 85, 113, 90, 223, 111, 237, 40, 11, 42,
            77, 27, 18, 249, 74, 170, 254, 81, 114, 176, 68, 189, 63, 119, 166, 150, 243, 226, 176,
            169, 32, 232, 38, 56, 165, 99, 82, 92, 230, 28, 156, 44, 236, 214, 140, 250, 4, 36,
            194, 92, 221, 132, 120, 81, 73, 142, 223, 25, 68, 206, 226, 172, 234, 138, 251, 89, 39,
            190, 190, 122, 46, 1, 189, 92, 12, 79, 185, 125, 148, 198, 85, 83, 25, 203, 130, 156,
            70, 58, 9, 114, 217, 51, 93, 183, 202, 146, 164, 159, 113, 251, 126, 213, 196, 50, 119,
            243, 23, 125, 20, 180, 129, 32, 189, 202, 26, 107, 129, 74, 139, 49, 230, 89, 91, 43,
            13, 158, 228, 35, 99, 220, 211, 215, 41, 64, 82, 0, 219, 173, 253, 225, 101, 238, 49,
            137, 59, 149, 90, 114, 197, 156, 90, 138, 84, 73, 221, 89, 206, 168, 166, 236, 230,
            196, 182, 76, 209, 36, 49, 194, 48, 117, 241, 201, 185, 92, 217, 223, 92, 202, 130,
            134, 153, 130, 81, 30, 60, 128, 149, 197, 125, 92, 17, 182, 213, 255, 171, 175, 125,
            89, 133, 156, 67, 2, 39, 15, 91, 178, 98, 221, 140, 227, 27, 243, 118, 225, 110, 120,
            124, 233, 182, 202, 170, 111, 96, 79, 91, 221, 180, 14, 44, 81, 138, 236, 100, 83, 174,
            139, 17, 78, 159, 113, 106, 230, 110, 4,
        ];
        let mut t: [i8; I] = [
            -1, 0, 1, -1, 1, 0, 0, 0, -1, 0, 1, -1, 0, 1, 1, 0, 0, 0, -1, 0, -1, 0, 0, 1, -1, 1,
            -1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, -1, 1, 0, 0, 0, 0, 0, -1, 1, 1, 0, 0, 0, 0, 0, 1,
            1, -1, 0, 0, 1, -1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, -1, -1, 0, 0, 1,
            1, 1, -1, 0, -1, -1, 0, 0, -1, -1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
            0, 0, 1, 0, 0, -1, 0, 0, 0, -1, 0, 0, 1, 1, -1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, -1, 0,
            -1, 0, 0, -1, -1, 1, 0, 1, 0, -1, 1, -1, 1, 0, 0, 0, 0, -1, -1, 0, 0, -1, -1, -1, 0,
            -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, -1, 1, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, -1, 0,
            -1, 0, -1, 0, -1, -1, 0, 0, -1, 0, 0, 0, 0, 0, -1, 0, 1, 0, -1, 0, 0, 0, 0, -1, 0, -1,
            1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, -1, -1, 1, 0, -1, 0, 0, 0, 0, -1, -1, -1, 0, -1, 0,
            0, -1, 1, 0, 0, 0, 1, -1, -1, -1, 0, 0, 0, 1,
        ];

        top_encode::<1039, 0>(&mut s, &t);
        top_decode::<1039>(&mut t, &s);

        assert_eq!(
            t,
            [
                15, 15, 1, 15, 1, 0, 0, 0, 15, 15, 1, 15, 0, 1, 1, 0, 0, 0, 15, 15, 15, 15, 0, 1,
                15, 0, 15, 15, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 15, 0, 0, 0, 0, 0, 0, 15, 1, 1, 0, 0,
                0, 0, 0, 1, 1, 15, 0, 0, 1, 15, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0,
                15, 15, 15, 0, 1, 1, 1, 15, 15, 15, 14, 0, 0, 15, 14, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0,
                1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 15, 15, 0, 0, 15, 15, 0, 1, 1, 15, 0, 0, 0, 0,
                1, 0, 0, 0, 0, 1, 15, 15, 15, 15, 0, 15, 15, 0, 0, 1, 0, 15, 1, 15, 1, 0, 0, 0, 0,
                15, 15, 15, 0, 15, 15, 14, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 15, 1, 0,
                0, 0, 0, 0, 0, 0, 0, 15, 0, 15, 0, 15, 0, 15, 0, 15, 15, 15, 0, 15, 0, 0, 0, 0, 0,
                15, 0, 1, 0, 15, 0, 0, 0, 0, 15, 15, 15, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 15,
                15, 0, 0, 15, 0, 0, 0, 0, 15, 14, 15, 15, 15, 15, 0, 15, 1, 0, 0, 0, 1, 15, 15, 14,
                0, 0, 0, 1,
            ]
        )
    }
}
