fn main() {
    let mut x: u8 = 255;
    x += 1;  // causes a panic
    println!("x is {}", x);
}
