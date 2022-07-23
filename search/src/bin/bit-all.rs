fn main() {
    let n = 5;
    for i in 0..1 << n {
        println!("{:b}", i);
    }
}
