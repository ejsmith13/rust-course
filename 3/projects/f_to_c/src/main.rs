use std::io;
fn main() {
    // (F − 32) × 5/9 = C
    // °F = (°C × 9/5) + 32
    let mut scale = String::new();
    let mut temp = String::new();

    println!("Enter the Scale you are starting from (F/C):");
    io::stdin()
        .read_line(&mut scale)
        .expect("Failed to read line");
    
    let scale: &str= scale.trim();
    println!("Enter your temp:");
    
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: i32 = temp.trim().parse().expect("NAN");

    let c = (temp - 32) * 5 / 9;
    let f = (temp * 9 / 5) + 32;

    // println!("Scale:{scale}");

    if scale == "F" {
        println!("Temp in C: {c}")
    } else if scale == "C" {
        println!("Temp in F: {f}")
    } else {
        println!("Unrecognized scale")
    }

     
}
