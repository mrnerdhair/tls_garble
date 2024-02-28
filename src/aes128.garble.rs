// fn u8_rotate_left(x: u8, i: u8) -> u8 {
//     (x << i) | (x >> (8u8 - i))
// }

// fn make_sbox() -> [u8; 256] {
//     let mut sbox = [0u8; 256];

//     let mut p = 1u8;
//     let mut q = 1u8;
    
//     // loop invariant: p * q == 1 in the Galois field
//     for i in 0usize..256usize {
//         if (i == 0usize || p != 1u8) {
//             // multiply p by 3
//             p = p ^ (p << 1u8) ^ (if (p & 128u8 != 0u8) { 27u8 } else { 0u8 });

//             // divide q by 3 (equals multiplication by 0xf6)
//             q = q ^ (q << 1u8);
//             q = q ^ (q << 2u8);
//             q = q ^ (q << 4u8);
//             q = q ^ (if (q & 128u8 != 0u8) { 9u8 } else { 0u8 });

//             // compute the affine transformation
//             sbox[p as usize] = q ^ u8_rotate_left(q, 1u8) ^ u8_rotate_left(q, 2u8) ^ u8_rotate_left(q, 3u8) ^ u8_rotate_left(q, 4u8) ^ 99u8;
//         }
//     }

//     // 0 is a special case since it has no inverse
//     sbox[0] = 99u8;

//     sbox
// }

// fn make_sbox() -> [u8; 256] {
//     [99u8, 124u8, 119u8, 123u8, 242u8, 107u8, 111u8, 197u8, 48u8, 1u8, 103u8, 43u8, 254u8, 215u8, 171u8, 118u8, 202u8, 130u8, 201u8, 125u8, 250u8, 89u8, 71u8, 240u8, 173u8, 212u8, 162u8, 175u8, 156u8, 164u8, 114u8, 192u8, 183u8, 253u8, 147u8, 38u8, 54u8, 63u8, 247u8, 204u8, 52u8, 165u8, 229u8, 241u8, 113u8, 216u8, 49u8, 21u8, 4u8, 199u8, 35u8, 195u8, 24u8, 150u8, 5u8, 154u8, 7u8, 18u8, 128u8, 226u8, 235u8, 39u8, 178u8, 117u8, 9u8, 131u8, 44u8, 26u8, 27u8, 110u8, 90u8, 160u8, 82u8, 59u8, 214u8, 179u8, 41u8, 227u8, 47u8, 132u8, 83u8, 209u8, 0u8, 237u8, 32u8, 252u8, 177u8, 91u8, 106u8, 203u8, 190u8, 57u8, 74u8, 76u8, 88u8, 207u8, 208u8, 239u8, 170u8, 251u8, 67u8, 77u8, 51u8, 133u8, 69u8, 249u8, 2u8, 127u8, 80u8, 60u8, 159u8, 168u8, 81u8, 163u8, 64u8, 143u8, 146u8, 157u8, 56u8, 245u8, 188u8, 182u8, 218u8, 33u8, 16u8, 255u8, 243u8, 210u8, 205u8, 12u8, 19u8, 236u8, 95u8, 151u8, 68u8, 23u8, 196u8, 167u8, 126u8, 61u8, 100u8, 93u8, 25u8, 115u8, 96u8, 129u8, 79u8, 220u8, 34u8, 42u8, 144u8, 136u8, 70u8, 238u8, 184u8, 20u8, 222u8, 94u8, 11u8, 219u8, 224u8, 50u8, 58u8, 10u8, 73u8, 6u8, 36u8, 92u8, 194u8, 211u8, 172u8, 98u8, 145u8, 149u8, 228u8, 121u8, 231u8, 200u8, 55u8, 109u8, 141u8, 213u8, 78u8, 169u8, 108u8, 86u8, 244u8, 234u8, 101u8, 122u8, 174u8, 8u8, 186u8, 120u8, 37u8, 46u8, 28u8, 166u8, 180u8, 198u8, 232u8, 221u8, 116u8, 31u8, 75u8, 189u8, 139u8, 138u8, 112u8, 62u8, 181u8, 102u8, 72u8, 3u8, 246u8, 14u8, 97u8, 53u8, 87u8, 185u8, 134u8, 193u8, 29u8, 158u8, 225u8, 248u8, 152u8, 17u8, 105u8, 217u8, 142u8, 148u8, 155u8, 30u8, 135u8, 233u8, 206u8, 85u8, 40u8, 223u8, 140u8, 161u8, 137u8, 13u8, 191u8, 230u8, 66u8, 104u8, 65u8, 153u8, 45u8, 15u8, 176u8, 84u8, 187u8, 22u8]
// }

// fn sbox(x: u8) -> u8 {
//     let sbox = make_sbox();
//     sbox[x as usize]
// }

// Boyer/Perala circuit from [A depth-16 circuit for the AES S-box](https://eprint.iacr.org/2011/332.pdf)
fn sbox(x: u8) -> u8 {
    let u0 = (x & 128u8) != 0u8;
    let u1 = (x & 64u8) != 0u8;
    let u2 = (x & 32u8) != 0u8;
    let u3 = (x & 16u8) != 0u8;
    let u4 = (x & 8u8) != 0u8;
    let u5 = (x & 4u8) != 0u8;
    let u6 = (x & 2u8) != 0u8;
    let u7 = (x & 1u8) != 0u8;

    let t1 = u0 ^ u3;
    let t2 = u0 ^ u5;
    let t3 = u0 ^ u6;
    let t4 = u3 ^ u5;
    let t5 = u4 ^ u6;
    let t6 = t1 ^ t5;
    let t7 = u1 ^ u2;
    let t8 = u7 ^ t6;
    let t9 = u7 ^ t7;
    let t10 = t6 ^ t7;
    let t11 = u1 ^ u5;
    let t12 = u2 ^ u5;
    let t13 = t3 ^ t4;
    let t14 = t6 ^ t11;
    let t15 = t5 ^ t11;
    let t16 = t5 ^ t12;
    let t17 = t9 ^ t16;
    let t18 = u3 ^ u7;
    let t19 = t7 ^ t18;
    let t20 = t1 ^ t19;
    let t21 = u6 ^ u7;
    let t22 = t7 ^ t21;
    let t23 = t2 ^ t22;
    let t24 = t2 ^ t10;
    let t25 = t20 ^ t17;
    let t26 = t3 ^ t16;
    let t27 = t1 ^ t12;

    let d = u7;

    let m1 = t13 & t6;
    let m2 = t23 & t8;
    let m3 = t14 ^ m1;
    let m4 = t19 & d;
    let m5 = m4 ^ m1;
    let m6 = t3 & t16;
    let m7 = t22 & t9;
    let m8 = t26 ^ m6;
    let m9 = t20 & t17;
    let m10 = m9 ^ m6;
    let m11 = t1 & t15;
    let m12 = t4 & t27;
    let m13 = m12 ^ m11;
    let m14 = t2 & t10;
    let m15 = m14 ^ m11;
    let m16 = m3 ^ m2;
    let m17 = m5 ^ t24;
    let m18 = m8 ^ m7;
    let m19 = m10 ^ m15;
    let m20 = m16 ^ m13;
    let m21 = m17 ^ m15;
    let m22 = m18 ^ m13;
    let m23 = m19 ^ t25;
    let m24 = m22 ^ m23;
    let m25 = m22 & m20;
    let m26 = m21 ^ m25;
    let m27 = m20 ^ m21;
    let m28 = m23 ^ m25;
    let m29 = m28 & m27;
    let m30 = m26 & m24;
    let m31 = m20 & m23;
    let m32 = m27 & m31;
    let m33 = m27 ^ m25;
    let m34 = m21 & m22;
    let m35 = m24 & m34;
    let m36 = m24 ^ m25;
    let m37 = m21 ^ m29;
    let m38 = m32 ^ m33;
    let m39 = m23 ^ m30;
    let m40 = m35 ^ m36;
    let m41 = m38 ^ m40;
    let m42 = m37 ^ m39;
    let m43 = m37 ^ m38;
    let m44 = m39 ^ m40;
    let m45 = m42 ^ m41;
    let m46 = m44 & t6;
    let m47 = m40 & t8;
    let m48 = m39 & d;
    let m49 = m43 & t16;
    let m50 = m38 & t9;
    let m51 = m37 & t17;
    let m52 = m42 & t15;
    let m53 = m45 & t27;
    let m54 = m41 & t10;
    let m55 = m44 & t13;
    let m56 = m40 & t23;
    let m57 = m39 & t19;
    let m58 = m43 & t3;
    let m59 = m38 & t22;
    let m60 = m37 & t20;
    let m61 = m42 & t1;
    let m62 = m45 & t4;
    let m63 = m41 & t2;

    let l0 = m61 ^ m62;
    let l1 = m50 ^ m56;
    let l2 = m46 ^ m48;
    let l3 = m47 ^ m55;
    let l4 = m54 ^ m58;
    let l5 = m49 ^ m61;
    let l6 = m62 ^ l5;
    let l7 = m46 ^ l3;
    let l8 = m51 ^ m59;
    let l9 = m52 ^ m53;
    let l10 = m53 ^ l4;
    let l11 = m60 ^ l2;
    let l12 = m48 ^ m51;
    let l13 = m50 ^ l0;
    let l14 = m52 ^ m61;
    let l15 = m55 ^ l1;
    let l16 = m56 ^ l0;
    let l17 = m57 ^ l1;
    let l18 = m58 ^ l8;
    let l19 = m63 ^ l4;
    let l20 = l0 ^ l1;
    let l21 = l1 ^ l7;
    let l22 = l3 ^ l12;
    let l23 = l18 ^ l2;
    let l24 = l15 ^ l9;
    let l25 = l6 ^ l10;
    let l26 = l7 ^ l9;
    let l27 = l8 ^ l10;
    let l28 = l11 ^ l14;
    let l29 = l11 ^ l17;
    let s0 = l6 ^ l24;
    let s1 = !(l16 ^ l26);
    let s2 = !(l19 ^ l28);
    let s3 = l6 ^ l21;
    let s4 = l20 ^ l22;
    let s5 = l25 ^ l29;
    let s6 = !(l13 ^ l27);
    let s7 = !(l6 ^ l23);

    ((s0 as u8) << 7u8)
        | ((s1 as u8) << 6u8)
        | ((s2 as u8) << 5u8)
        | ((s3 as u8) << 4u8)
        | ((s4 as u8) << 3u8)
        | ((s5 as u8) << 2u8)
        | ((s6 as u8) << 1u8)
        | ((s7 as u8) << 0u8)
}

fn u32_from_be_bytes(x: [u8; 4]) -> u32 {
    ((x[0] as u32) << 24u8) | ((x[1] as u32) << 16u8) | ((x[2] as u32) << 8u8) | ((x[3] as u32) << 0u8)
}

fn u32_to_be_bytes(x: u32) -> [u8; 4] {
    [
        ((x >> 24u8) as u8),
        ((x >> 16u8) as u8),
        ((x >> 8u8) as u8),
        ((x >> 0u8) as u8),
    ]
}

fn sub_bytes(mut x: [u32; 4]) -> [u32; 4] {
    for i in 0usize..4usize {
        let mut xi = u32_to_be_bytes(x[i]);
        for j in 0usize..4usize {
            xi[j] = sbox(xi[j]);
        }
        x[i] = u32_from_be_bytes(xi);
    }

    x
}

fn shift_rows(x: [u32; 4]) -> [u32; 4] {
    let x0 = u32_to_be_bytes(x[0]);
    let x1 = u32_to_be_bytes(x[1]);
    let x2 = u32_to_be_bytes(x[2]);
    let x3 = u32_to_be_bytes(x[3]);
    let mut y0 = x0;
    let mut y1 = x1;
    let mut y2 = x2;
    let mut y3 = x3;

    y0[1] = x1[1];
    y1[1] = x2[1];
    y2[1] = x3[1];
    y3[1] = x0[1];
    y0[2] = x2[2];
    y1[2] = x3[2];
    y2[2] = x0[2];
    y3[2] = x1[2];
    y0[3] = x3[3];
    y1[3] = x0[3];
    y2[3] = x1[3];
    y3[3] = x2[3];

    [
        u32_from_be_bytes(y0), 
        u32_from_be_bytes(y1),
        u32_from_be_bytes(y2),
        u32_from_be_bytes(y3),
    ]
}

// https://en.wikipedia.org/w/index.php?title=Rijndael_MixColumns&oldid=1204191266#Implementation_example
fn gmix_column(mut r: [u8; 4]) -> [u8; 4] {
    let mut a = [0u8; 4];
    let mut b = [0u8; 4];

    for c in 0usize..4usize {
        a[c] = r[c];
        let h = r[c] >> 7u8;
        b[c] = r[c] << 1u8;
        b[c] = b[c] ^ (h * 27u8);
    }

    r[0] = b[0] ^ a[3] ^ a[2] ^ b[1] ^ a[1]; /* 2 * a0 + a3 + a2 + 3 * a1 */
    r[1] = b[1] ^ a[0] ^ a[3] ^ b[2] ^ a[2]; /* 2 * a1 + a0 + a3 + 3 * a2 */
    r[2] = b[2] ^ a[1] ^ a[0] ^ b[3] ^ a[3]; /* 2 * a2 + a1 + a0 + 3 * a3 */
    r[3] = b[3] ^ a[2] ^ a[1] ^ b[0] ^ a[0]; /* 2 * a3 + a2 + a1 + 3 * a0 */

    r
}

fn mix_columns(mut x: [u32; 4]) -> [u32; 4] {
    for i in 0usize..4usize {
        x[i] = u32_from_be_bytes(gmix_column(u32_to_be_bytes(x[i])));
    }

    x
}

fn add_round_key(mut a: [u32; 4], rk: [u32; 44], i: u8) -> [u32; 4] {
    for j in 0usize..4usize {
        a[j] = a[j] ^ rk[((i << 2u8) + (j as u8)) as usize];
    }

    a
}

fn to_aes_state(x: [u8; 16]) -> [u32; 4] {
    [
        u32_from_be_bytes([x[0], x[1], x[2], x[3]]),
        u32_from_be_bytes([x[4], x[5], x[6], x[7]]),
        u32_from_be_bytes([x[8], x[9], x[10], x[11]]),
        u32_from_be_bytes([x[12], x[13], x[14], x[15]]),
    ]
}

fn from_aes_state(x: [u32; 4]) -> [u8; 16] {
    let x0 = u32_to_be_bytes(x[0]);
    let x1 = u32_to_be_bytes(x[1]);
    let x2 = u32_to_be_bytes(x[2]);
    let x3 = u32_to_be_bytes(x[3]);

    [
        x0[0],
        x0[1],
        x0[2],
        x0[3],
        x1[0],
        x1[1],
        x1[2],
        x1[3],
        x2[0],
        x2[1],
        x2[2],
        x2[3],
        x3[0],
        x3[1],
        x3[2],
        x3[3],
    ]
}

fn rot_word(x: u32) -> u32 {
    let x = u32_to_be_bytes(x);
    u32_from_be_bytes([x[1], x[2], x[3], x[0]])
}

fn sub_word(x: u32) -> u32 {
    let x = u32_to_be_bytes(x);
    u32_from_be_bytes([sbox(x[0]), sbox(x[1]), sbox(x[2]), sbox(x[3])])
}

pub fn aes128_key_schedule(k: [u8; 16]) -> [u32; 44] {
    let k = to_aes_state(k);
    let rcon = [
        u32_from_be_bytes([0u8; 4]),
        u32_from_be_bytes([1u8, 0u8, 0u8, 0u8]),
        u32_from_be_bytes([2u8, 0u8, 0u8, 0u8]),
        u32_from_be_bytes([4u8, 0u8, 0u8, 0u8]),
        u32_from_be_bytes([8u8, 0u8, 0u8, 0u8]),
        u32_from_be_bytes([16u8, 0u8, 0u8, 0u8]),
        u32_from_be_bytes([32u8, 0u8, 0u8, 0u8]),
        u32_from_be_bytes([64u8, 0u8, 0u8, 0u8]),
        u32_from_be_bytes([128u8, 0u8, 0u8, 0u8]),
        u32_from_be_bytes([27u8, 0u8, 0u8, 0u8]),
        u32_from_be_bytes([54u8, 0u8, 0u8, 0u8]),
    ];

    let mut w = [0u32; 44];
    for i in 0usize..4usize {
        w[i] = k[i];
    }

    for i in 1usize..11usize {
        for j in 0usize..4usize {
            let ij = (i << 2u8) + j;
            if (j == 0usize) {
                w[ij] = w[ij - 4usize] ^ sub_word(rot_word(w[ij - 1usize])) ^ rcon[i];
            } else {
                w[ij] = w[ij - 4usize] ^ w[ij - 1usize];
            }
        }
    }

    w
}

// fn extract_rk(rk: [u32; 44], i: usize) -> [u8; 16] {
//     let mut out = [0u8; 16];
//     for j in 0usize..4usize {
//         let x = u32_to_be_bytes(rk[(i << 2u8) + j]);
//         for k in 0usize..4usize {
//             out[(j << 2u8) + k] = x[k];
//         }
//     }
//     out
// }

pub fn aes128_with_key_schedule(rk: [u32; 44], a: [u8; 16]) -> [u8; 16] {
    let mut a = to_aes_state(a);
    a = add_round_key(a, rk, 0u8);

    for i in 1u8..10u8 {
        a = sub_bytes(a);
        a = shift_rows(a);
        a = mix_columns(a);
        a = add_round_key(a, rk, i);
    }

    a = sub_bytes(a);
    a = shift_rows(a);
    a = add_round_key(a, rk, 10u8);

    from_aes_state(a)
}

pub fn aes128(k: [u8; 16], a: [u8; 16]) -> [u8; 16] {
    let rk = aes128_key_schedule(k);
    aes128_with_key_schedule(rk, a)
}

pub fn main(ka: ([u8; 16], [u8; 16])) -> [u8; 16] {
    let k = ka.0;
    let a = ka.1;
    aes128(k, a)
}
