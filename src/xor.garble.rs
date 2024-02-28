pub fn main(x: (u8, u8)) -> u8 {
    x.0 ^ (x.1 << 1u8)
}
