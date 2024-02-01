use std::convert::TryInto;
use num::Complex;
use std::time::{ Duration, Instant };
use std::ops::{ Add };
fn main() {
    let a = 10;
    let b: i32 = 20;
    let c = 30i32;
    let d = 30_i32;
    let e = add(add(a, b), add(d, c));
    println!("( a + b ) + ( c + d ) = {}", e);

    // integers deimals and floating point numbers
    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22i32;

    let addition = twenty + twenty_one + twenty_two;
    println!("{}+{}+{}={}", twenty, twenty_one, twenty_two, addition);

    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    let forty_twos = [42.0, 42f32, 42.0_f32];
    println!("{:02}", forty_twos[0]);

    //integers with base2 , base 8 and base 16 notation

    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 0x12c;
    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2:  {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8:  {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);

    let a: i32 = 10;
    let b: u16 = 100;

    let b_ = b.try_into().unwrap();

    if a < b_ {
        println!("Ten is less than on hundred");
    }

    let result: f32 = 0.1 + 0.2;
    let expected: f32 = 0.3;
    let absolute_difference = (expected - result).abs();
    println!("{} ,{} , {}", absolute_difference, result, expected);

    // let x: f32 = 1.0 / 0.0;
    // assert!(x.is_nan());

    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let complex_result = a + b;

    println!("{} + {}i", complex_result.re, complex_result.im);

    //flow control

    let mut count = 0;
    let time_limit = Duration::new(1, 0);
    let start = Instant::now();

    while Instant::now() - start < time_limit {
        count += 1;
    }

    println!("{}", count);

    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in &haystack {
        let result = match item {
            42 | 132 => "hit!",
            _ => "miss",
        };
        if result == "hit!" {
            println!("{}:{}", item, result);
        }
    }

    let a = 42;
    let r = &a;
    let b = a - *r;

    println!("a-a={}", b);

    let needle = 0o204;

    for item in &haystack {
        if *item == needle {
            println!("{}", item);
        }
    }

    let res = add_with_lifetimes(&a, &b);
    println!("{}", res);

    let floats = add_generic(1.2, 1.4);
    let ints = add_generic(a, b);
    let durations = add_generic(Duration::new(5, 0), Duration::new(6, 0));

    println!("{}", floats);
    println!("{}", ints);
    println!("{:?}", durations);
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}

fn add_generic<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
