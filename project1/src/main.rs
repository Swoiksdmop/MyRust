fn main() {
    let x = 100 * 10 * 300;
    println!("{x}");

    if x == 300000 {
        println!("great");
    }else {
        panic!("Failed to read line");
    }
}