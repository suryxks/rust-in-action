#[allow(arithmetic_overflow)]
pub fn data() {
    let a: u16 = 50115;
    let b: i16 = -15421;
    println!("a: {:016b} {}", a, a);
    println!("b: {:016b} {}", b, b);

    let a: f32 = 42.42;

    let frankentype: u32 = unsafe { std::mem::transmute(a) };

    println!("{}", frankentype);
    println!("{:032b}", frankentype);

    let b: f32 = unsafe { std::mem::transmute(frankentype) };

    println!("{}", b);

    let mut i: u16 = 0;
    print!("{}...", i);

    // loop {
    //     i += 1000;
    //     print!("{}...", i);
    //     if i % 10_000 == 0 {
    //         print!("\n");
    //     }
    // }
    let zero: u16 = 0b0000_0000_0000_0000;
    let one: u16 = 0b0000_0000_0000_0001;
    let two: u16 = 0b0000_0000_0000_0010;
    let sixtyfivethousand_533: u16 = 0b1111_1111_1111_1101;
    let sixtyfivethousand_534: u16 = 0b1111_1111_1111_1110;
    let sixtyfivethousand_535: u16 = 0b1111_1111_1111_1111;

    print!("{}, {}, {}, ..., ", zero, one, two);
    println!("{}, {}, {}", sixtyfivethousand_533, sixtyfivethousand_534, sixtyfivethousand_535);

    let big_endian: [u8; 4] = [0xaa, 0xbb, 0xcc, 0xdd];
    let little_endian: [u8; 4] = [0xdd, 0xcc, 0xbb, 0xaa];

    let a: i32 = unsafe { std::mem::transmute(big_endian) };
    let b: i32 = unsafe { std::mem::transmute(little_endian) };

    println!("{} vs {}", a, b);
    floating_representation()
}

fn floating_representation() {
    // to isolate the sign bit , shift the other bits out of the way.
    // for f32 , this involves a right shift of 31 places (>>31)

    let n: f32 = -42.42;
    let n_bits: u32 = n.to_bits();
    let sign_bits = n_bits >> 31;

    println!("{}", sign_bits); //0

    // to isolate the exponent, two bit manipulations are required. First, perform
    // right shift to overwrite the mantissa's bits (>>23) then use an AND mask (& 0xff) to exclude the sign bit
    let exponent_ = n_bits >> 23;
    let exponent_ = exponent_ & 0xff;
    let exponent_ = (exponent_ as i32) - 127;
}
