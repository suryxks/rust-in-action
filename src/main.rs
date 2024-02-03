use std::convert::TryInto;
// use std::fs::File;
use num::Complex;
use std::time::{ Duration, Instant };
use std::ops::{ Add };
use std::fmt::Debug;
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

    let one = [1, 2, 4];
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    let arrays = [one, two, blank1, blank2];
    for array in &arrays {
        println!("{:?}", array);
        for n in array.iter() {
            print!("\t{} + 10 = {}", n, n + 10);
        }
        let mut sum = 0;
        for i in 0..array.len() {
            sum += array[i];
        }
        println!("\t({:?} = {})", a, sum);
    }
    main2();
    main3()
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
#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }
    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }
    fn read(self: &File, save_to: &mut Vec<u8>) -> usize {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        read_length
    }
}
struct Hostname(String);

fn connect(host: Hostname) {
    println!("connected to {}", host.0)
}

fn open(f: &mut File) -> bool {
    true
}
fn close(f: &mut File) -> bool {
    false
}

fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    let mut tmp = f.data.clone();
    let read_length = tmp.len();

    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    read_length
}
fn main2() {
    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    println!("{:?}", f1);

    println!("{} is {} bytes long", f1_name, f1_length);

    let mut f2 = File { name: String::from("2.txt"), data: Vec::new() };

    let mut buffer: Vec<u8> = vec![];

    open(&mut f2);
    let f2_length = read(&f2, &mut buffer);
    close(&mut f2);

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f2);
    println!("{} is {} bytes long", &f2.name, f2_length);
    println!("{}", text);

    let f3 = File::new("f3.txt");

    let f3_name = &f3.name;
    let f3_length = f3.data.len();

    println!("{:?}", f3);
    println!("{} is {} bytes long", f3_name, f3_length);

    let f4_data: Vec<u8> = vec![114, 117, 115, 116, 33];

    let mut f4 = File::new_with_data("2.txt", &f4_data);

    let mut buffer4: Vec<u8> = vec![];

    open(&mut f4);
    let f4_length = f4.read(&mut buffer);
    close(&mut f4);

    let text = String::from_utf8_lossy(&buffer4);

    println!("{:?}", f4);
    println!("{} is {} bytes long", &f4.name, f4_length);
    println!("{}", text)
}

fn main3() {
    let ordinary_string = String::from("localhost");
    let host = Hostname(ordinary_string.clone());
    connect(host);
}
