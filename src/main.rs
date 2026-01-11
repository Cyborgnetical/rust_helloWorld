fn main() {
    println!("Hello, world!");
    println!("Apperently rust decided to explode for a bit before it got working");

    println!("Well look, I got variables working.");
    let x = 24;
    let y = 35;
    let z = x + y;
    println!("{}",z);
    println!("apperently I have to use werid bracket thingys, I still have not figured out escape charaters");

    let mut muddy = 456;
    muddy = 5843;
    println!("Now we got mutables, here is muddy: {}",muddy);
    println!("Ah yes, the complier complains");

    let x = 123;
    println!("Guess what, x is no more and redefined {}",x)
}
