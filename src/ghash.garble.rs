fn make_last4() -> [u64; 16] {
    [0u64, 7200u64, 14400u64, 9312u64, 28800u64, 27808u64, 18624u64, 21728u64, 57600u64, 64800u64, 55616u64, 50528u64, 37248u64, 36256u64, 43456u64, 46560u64]
}

fn last4(i: u8) -> u64 {
    make_last4()[i as usize]
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

pub fn gcm_setup(h: [u8; 16]) -> [u64; 32] {
    let mut ctxhl = [0u64; 16];
    let mut ctxhh = [0u64; 16];

    let mut vh = ((u32_from_be_bytes([h[0], h[1], h[2], h[3]]) as u64) << 32u8) | (u32_from_be_bytes([h[4], h[5], h[6], h[7]]) as u64);
    let mut vl = ((u32_from_be_bytes([h[8], h[9], h[10], h[11]]) as u64) << 32u8) | (u32_from_be_bytes([h[12], h[13], h[14], h[15]]) as u64);

    ctxhl[8] = vl;
    ctxhh[8] = vh;
    ctxhh[0] = 0u64;
    ctxhl[0] = 0u64;

    for i in 0u8..2u8 {
        let i = 4usize >> i;
        let t = (((vl & 1u64) as u32) * 3774873600u32) as u64;
        vl = (vh << 63u8) | (vl >> 1u8);
        vh = (vh >> 1u8) ^ (t << 32u8);
        ctxhl[i] = vl;
        ctxhh[i] = vh;
    }

    for i in 0u8..2u8 {
        let i = 2usize << i;
        let vh = ctxhh[i];
        let vl = ctxhl[i];
        for j in 1usize..16usize {
            if j < i {
                ctxhh[i + j] = vh ^ ctxhh[j];
                ctxhl[i + j] = vl ^ ctxhl[j];
            }
        }
    }

    [
        ctxhl[0],
        ctxhl[1],
        ctxhl[2],
        ctxhl[3],
        ctxhl[4],
        ctxhl[5],
        ctxhl[6],
        ctxhl[7],
        ctxhl[8],
        ctxhl[9],
        ctxhl[10],
        ctxhl[11],
        ctxhl[12],
        ctxhl[13],
        ctxhl[14],
        ctxhl[15],
        ctxhh[0],
        ctxhh[1],
        ctxhh[2],
        ctxhh[3],
        ctxhh[4],
        ctxhh[5],
        ctxhh[6],
        ctxhh[7],
        ctxhh[8],
        ctxhh[9],
        ctxhh[10],
        ctxhh[11],
        ctxhh[12],
        ctxhh[13],
        ctxhh[14],
        ctxhh[15],
    ]
}

pub fn gcm_mult(ctx: [u64; 32], x: [u8; 16]) -> [u8; 16] {
    let ctxhl = [
        ctx[0],
        ctx[1],
        ctx[2],
        ctx[3],
        ctx[4],
        ctx[5],
        ctx[6],
        ctx[7],
        ctx[8],
        ctx[9],
        ctx[10],
        ctx[11],
        ctx[12],
        ctx[13],
        ctx[14],
        ctx[15],
    ];
    let ctxhh = [
        ctx[16],
        ctx[17],
        ctx[18],
        ctx[19],
        ctx[20],
        ctx[21],
        ctx[22],
        ctx[23],
        ctx[24],
        ctx[25],
        ctx[26],
        ctx[27],
        ctx[28],
        ctx[29],
        ctx[30],
        ctx[31],
    ];

    let lo = x[15] & 15u8;
    let hi = x[15] >> 4u8;
    let mut zh = ctxhh[lo as usize];
    let mut zl = ctxhl[lo as usize];

    for i in 0usize..16usize {
        let i = 15usize - i;

        let lo = x[i] & 15u8;
        let hi = x[i] >> 4u8;

        if i != 15usize {
            let rem = (zl & 15u64) as u8;
            zl = (zh << 60u8) | (zl >> 4u8);
            zh = (zh >> 4u8);
            zh = zh ^ (last4(rem) << 48u8);
            zh = zh ^ ctxhh[lo as usize];
            zl = zl ^ ctxhl[lo as usize];
        }

        let rem = (zl & 15u64) as u8;
        zl = (zh << 60u8) | (zl >> 4u8);
        zh = (zh >> 4u8);
        zh = zh ^ (last4(rem) << 48u8);
        zh = zh ^ ctxhh[hi as usize];
        zl = zl ^ ctxhl[hi as usize];
    }

    let out0 = u32_to_be_bytes((zh >> 32u8) as u32);
    let out1 = u32_to_be_bytes(zh as u32);
    let out2 = u32_to_be_bytes((zl >> 32u8) as u32);
    let out3 = u32_to_be_bytes(zl as u32);

    [
        out0[0],
        out0[1],
        out0[2],
        out0[3],
        out1[0],
        out1[1],
        out1[2],
        out1[3],
        out2[0],
        out2[1],
        out2[2],
        out2[3],
        out3[0],
        out3[1],
        out3[2],
        out3[3],
    ]
}

pub fn main(hx: ([u8; 16], [u8; 16])) -> [u8; 16] {
    let h = hx.0;
    let x = hx.1;

    let ctx = gcm_setup(h);
    
    gcm_mult(ctx, x)

    // let mut p = [0u8; 16];
    // p[0] = 128u8;
    // p[15] = 128u8 + 4u8 + 2u8 + 1u8;
    // mod_polynomial_mult(h, x, p)
}

fn shift_128_left_one(x: [u8; 16]) -> [u8; 16] {
    [
        (x[0] << 1u8) ^ ((x[1] & 128u8) >> 7u8),
        (x[1] << 1u8) ^ ((x[2] & 128u8) >> 7u8),
        (x[2] << 1u8) ^ ((x[3] & 128u8) >> 7u8),
        (x[3] << 1u8) ^ ((x[4] & 128u8) >> 7u8),
        (x[4] << 1u8) ^ ((x[5] & 128u8) >> 7u8),
        (x[5] << 1u8) ^ ((x[6] & 128u8) >> 7u8),
        (x[6] << 1u8) ^ ((x[7] & 128u8) >> 7u8),
        (x[7] << 1u8) ^ ((x[8] & 128u8) >> 7u8),
        (x[8] << 1u8) ^ ((x[9] & 128u8) >> 7u8),
        (x[9] << 1u8) ^ ((x[10] & 128u8) >> 7u8),
        (x[10] << 1u8) ^ ((x[11] & 128u8) >> 7u8),
        (x[11] << 1u8) ^ ((x[12] & 128u8) >> 7u8),
        (x[12] << 1u8) ^ ((x[13] & 128u8) >> 7u8),
        (x[13] << 1u8) ^ ((x[14] & 128u8) >> 7u8),
        (x[14] << 1u8) ^ ((x[15] & 128u8) >> 7u8),
        (x[15] << 1u8),
    ]
}

pub fn mod_polynomial_mult(a: [u8; 16], b: [u8; 16], p: [u8; 16]) -> [u8; 16] {
    let mut result = [0u8; 16];

    for i in 0usize..16usize {
        let x = b[i];
        for j in 0u8..8u8 {
            let bit = (x << j) & 128u8 != 0u8;
            let highbit = (result[0] & 128u8) != 0u8;
            result = shift_128_left_one(result);
            if bit {
                for i in 0usize..16usize {
                    result[i] = result[i] ^ a[i];
                }
            }
            if highbit {
                for i in 0usize..16usize {
                    result[i] = result[i] ^ p[i];
                }
            }
        }
    }

    result
}
