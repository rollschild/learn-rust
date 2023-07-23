use std::mem::transmute;

fn main() {
    let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];

    let a: i32 = unsafe { transmute(big_endian) };
    let b: i32 = unsafe { transmute(little_endian) };

    println!("{} vs. {}", a, b);

    let n: f32 = -42.42;
    let n_bits: u32 = n.to_bits();
    let sign_bit = n_bits >> 31;

    println!("sign of {} is {}", n, sign_bit);
}
