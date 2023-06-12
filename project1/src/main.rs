fn main() {
    let x = 100 * 10 * 300;
    println!("{x}");

    if x == 300000 {
        println!("great\nIf you experience issues, pull it up on github");
    } else {
        panic!("Failed to read line");
    }
}
