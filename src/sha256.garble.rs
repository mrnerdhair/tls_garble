fn rotate_right(x: u32, i: u8) -> u32 {
    (x >> i) | (x << (32u8 - i))
}

fn wrapping_add(x: u32, y: u32) -> u32 {
    ((x as u64) + (y as u64)) as u32
}

fn sha256(m: [u32; 16], mut h: [u32; 8]) -> [u32; 8] {
    let k = [1116352408u32, 1899447441u32, 3049323471u32, 3921009573u32, 961987163u32, 1508970993u32, 2453635748u32, 2870763221u32, 3624381080u32, 310598401u32, 607225278u32, 1426881987u32, 1925078388u32, 2162078206u32, 2614888103u32, 3248222580u32, 3835390401u32, 4022224774u32, 264347078u32, 604807628u32, 770255983u32, 1249150122u32, 1555081692u32, 1996064986u32, 2554220882u32, 2821834349u32, 2952996808u32, 3210313671u32, 3336571891u32, 3584528711u32, 113926993u32, 338241895u32, 666307205u32, 773529912u32, 1294757372u32, 1396182291u32, 1695183700u32, 1986661051u32, 2177026350u32, 2456956037u32, 2730485921u32, 2820302411u32, 3259730800u32, 3345764771u32, 3516065817u32, 3600352804u32, 4094571909u32, 275423344u32, 430227734u32, 506948616u32, 659060556u32, 883997877u32, 958139571u32, 1322822218u32, 1537002063u32, 1747873779u32, 1955562222u32, 2024104815u32, 2227730452u32, 2361852424u32, 2428436474u32, 2756734187u32, 3204031479u32, 3329325298u32];

    let mut x = h;

    let mut w = [
        m[0], m[1], m[2], m[3], m[4], m[5], m[6], m[7], m[8], m[9], m[10], m[11], m[12], m[13], m[14], m[15],
        0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
        0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
        0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
    ];

    for i in 16usize..64usize {
        let s0 = rotate_right(w[i - 15usize], 7u8) ^ rotate_right(w[i - 15usize], 18u8) ^ (w[i - 15usize] >> 3u8);
        let s1 = rotate_right(w[i - 2usize], 17u8) ^ rotate_right(w[i - 2usize], 19u8) ^ (w[i - 2usize] >> 10u8);
        w[i] = wrapping_add(wrapping_add(wrapping_add(w[i - 16usize], s0), w[i - 7usize]), s1);
    }

    for i in 0usize..64usize {
        let s1 = rotate_right(x[4], 6u8) ^ rotate_right(x[4], 11u8) ^ rotate_right(x[4], 25u8);
        let ch = (x[4] & x[5]) ^ (!x[4] & x[6]);
        let temp1 = wrapping_add(wrapping_add(wrapping_add(wrapping_add(x[7], s1), ch), k[i]), w[i]);
        let s0 = rotate_right(x[0], 2u8) ^ rotate_right(x[0], 13u8) ^ rotate_right(x[0], 22u8);
        let maj = (x[0] & x[1]) ^ (x[0] & x[2]) ^ (x[1] & x[2]);
        let temp2 = wrapping_add(s0, maj);

        x = [wrapping_add(temp1, temp2), x[0], x[1], x[2], wrapping_add(x[3], temp1), x[4], x[5], x[6]];
    }

    for i in 0usize..8usize {
        h[i] = wrapping_add(h[i], x[i]);
    }

    h
}

pub fn main(mh: ([u32; 16], [u32; 8])) -> [u32; 8] {
    let m = mh.0;
    let h = mh.1;
    sha256(m, h)
}
