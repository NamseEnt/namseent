// Test file to verify that we can access MIR from std library
fn main() {
    let a = 5i32;
    // This will call i32's Display::fmt implementation
    println!("{}", a);

    // Additional test: call some std library functions
    let v = vec![1, 2, 3];
    println!("{:?}", v);
}
