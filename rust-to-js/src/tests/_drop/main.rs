fn main() {
    let a = [1, 2, 3];
    {
        let b = [2, 3, 4];
        println!("{} {} {}", a[0], a[1], a[2]);
        println!("{} {} {}", b[0], b[1], b[2]);
    }
    println!("{} {} {}", a[0], a[1], a[2]);
}
