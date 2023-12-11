fn main() {
    std::fs::write("target/output.txt", "Hello".as_bytes()).unwrap();
}
