use core::time;
use std::{io, thread};

use rand::Rng;

fn main() {

    let pause_time = time::Duration::from_millis(1500);
    thread::sleep(pause_time);

    println!("Hello, world!");
    thread::sleep(pause_time);
    println!("Apperently rust decided to explode for a bit before it got working");
    thread::sleep(pause_time);
    println!("Well look, I got variables working.");
    thread::sleep(pause_time);
    let x = 24;
    let y = 35;
    let z = x + y;
    println!("24+35={}",z);
    thread::sleep(pause_time);
    println!("apperently I have to use werid bracket thingys, I still have not figured out escape charaters");
    thread::sleep(pause_time);

    let mut muddy = 456;
    muddy = 5843;
    println!("Now we got mutables, here is muddy: {}",muddy);
    thread::sleep(pause_time);
    println!("Ah yes, the complier complains");
    thread::sleep(pause_time);

    let x = 123;
    println!("Guess what, x is no more and redefined {}",x);
    thread::sleep(pause_time);

    
    println!("Now we are getting to strings, its similar to C++. We are also dealing with input");
    thread::sleep(pause_time);
    println!("Now type in abracadabra:");
    thread::sleep(pause_time);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You typed in {}",guess);
    thread::sleep(pause_time);

    let secret_number = rand::thread_rng().gen_range(1..=100);
    thread::sleep(pause_time);
    println!("{secret_number}");
    thread::sleep(pause_time);
    println!("hehehe, funny number. Its random.");
    thread::sleep(pause_time);
    

    println!("Now I discovered how to pause the program and make it a nice story");
    thread::sleep(pause_time);
    thread::sleep(pause_time);
    thread::sleep(pause_time);
    println!("I paused it for around 4.5 seconds");
}
