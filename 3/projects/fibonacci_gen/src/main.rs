use std::io;

fn main() {
    let mut n = String::new();

    println!("Enter a number: ");

    io::stdin()
        .read_line(&mut n)
        .expect("Please enter a number");

    let n: u32 = n.trim().parse().expect("NAN");

    if n == 1 {
        println! {"Answer is 1"}
    } else {
        fibonacci(n)
    }
}

fn fibonacci(n: u32) {
    // let mut count = 2;
    let mut a = 0;
    let mut b = 1;
    let mut c: u32;

    // while count < (n ) {
    //     c = a + b;
    //     a = b;
    //     b = c;

    //     count += 1
    // }

    for _ in 2..n{
        c = a + b;
        a = b;
        b = c;
    }

    println!("{n}th Fibonacci sequence number is {b}")
}
