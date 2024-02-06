mod data_in_depth;

use std::convert::TryInto;
// use std::fs::File;
use num::Complex;
use std::time::{ Duration, Instant };
use std::ops::{ Add };
use std::fmt::{ write, Debug };
use rand::prelude::*;
use std::fmt;
use std::fmt::{ Display };
use std::rc::Rc;
fn main() {
    // basics();
    // main2();
    // main3();
    // borrow();
    data_in_depth::data()
}
fn basics() {
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
fn one_in(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}
#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}
#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>,
    pub state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}
impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}
impl File {
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
    pub fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }
    pub fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open"));
        }

        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}
fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}

fn main2() {
    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
        state: FileState::Closed,
    };

    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    println!("{:?}", f1);

    println!("{} is {} bytes long", f1_name, f1_length);

    let mut f2 = File { name: String::from("2.txt"), data: Vec::new(), state: FileState::Closed };

    let mut buffer: Vec<u8> = vec![];

    // open(&mut f2);
    // let f2_length = read(&f2, &mut buffer);
    // close(&mut f2);

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f2);
    // println!("{} is {} bytes long", &f2.name, f2_length);
    println!("{}", text);

    let f3 = File::new("f3.txt");

    let f3_name = &f3.name;
    let f3_length = f3.data.len();

    println!("{:?}", f3);
    println!("{} is {} bytes long", f3_name, f3_length);

    let f4_data: Vec<u8> = vec![114, 117, 115, 116, 33];

    let mut f4 = File::new_with_data("2.txt", &f4_data);

    let mut buffer4: Vec<u8> = vec![];

    // open(&mut f4);
    let f4_length = f4.read(&mut buffer);
    // close(&mut f4);

    let text = String::from_utf8_lossy(&buffer4);

    println!("{:?}", f4);
    // println!("{} is {} bytes long", &f4.name, f4_length);
    println!("{}", text)
}

fn main3() {
    let f4_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f4 = File::new_with_data("4.txt", &f4_data);

    let mut buffer: Vec<u8> = vec![];

    f4 = open(f4).unwrap();
    let f4_length = f4.read(&mut buffer).unwrap();

    f4 = close(f4).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f4);
    println!("{} is {} bytes long", &f4.name, f4_length);

    println!("{}", text);

    let log =
        "BEGIN Transaction XK342
UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\": 40.00}
DELETE 342:LO/22111";

    for line in log.lines() {
        let parse_result = parse_log(line);
        println!("{:?}", parse_result);
    }

    let mut f5 = File::new("5.txt");
    let mut buffer: Vec<u8> = vec![];

    if f5.read(&mut buffer).is_err() {
        println!("error checking is working");
    }
    f5 = open(f5).unwrap();

    let f5_length = f5.read(&mut buffer).unwrap();

    f5 = close(f5).unwrap();

    let text = String::from_utf8_lossy(&buffer);
    println!("{:?}", f5);
    println!("{} is {} bytes long", &f5.name, f5_length);

    println!("{}", text);
}

#[derive(Debug)]
enum Event {
    Update,
    Delete,
    Unknown,
}

type Messages = String;

fn parse_log(line: &str) -> (Event, Messages) {
    let parts: Vec<_> = line.splitn(2, ' ').collect();

    if parts.len() == 1 {
        return (Event::Unknown, String::from(line));
    }

    let event = parts[0];
    let rest = String::from(parts[1]);

    match event {
        "UPDATE" | "update" => (Event::Update, rest),
        "DELETE" | "delete" => (Event::Delete, rest),
        _ => (Event::Unknown, String::from(line)),
    }
}

#[derive(Debug)]
enum StatusMessage {
    OK,
}
#[derive(Debug, Clone, Copy)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}
#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}
struct Groundstation;
impl Groundstation {
    fn send(&self, msg: Message, mailbox: &mut Mailbox) {
        mailbox.post(msg)
    }
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id }
    }
}

impl CubeSat {
    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg)
    }
    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }
        None
    }
}
fn check_status(sat_id: CubeSat) -> CubeSat {
    // StatusMessage::OK
    sat_id
}

fn borrow() {
    let sat_a = CubeSat { id: 0 };
    let sat_b = CubeSat { id: 1 };
    let sat_c = CubeSat { id: 2 };

    let sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);

    println!("a: {:?}, b: {:?}, c: {:?}", sat_a, sat_b, sat_c);

    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);

    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

    let base = Groundstation {};
    let mut sat_a = CubeSat {
        id: 0,
        // mailbox: Mailbox {
        //     messages: vec![],
        // },
    };
    println!("t0: {:?}", sat_a);
    // base.send(&mut sat_a, Message::from("hellot ther!"));

    println!("t1: {:?}", sat_a);

    // let msg = sat_a.recv();
    println!("t2: {:?}", sat_a);
    // println!("msg: {:?}", msg);

    let mut mail = Mailbox { messages: vec![] };
    let base = Groundstation {};
    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = Message { to: sat_id, content: String::from("hello") };
        base.send(msg, &mut mail);
    }
    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = sat.recv(&mut mail);
        println!("{:?}: {:?}", sat, msg);
    }
}

// Resolving ownership issues
// * use references where full ownership is not required
// * Duplicate the value
// * Refactor code to reduce the number of long-lived objects
// * Wrap your data in a type designed to assist with movement issues.

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}
