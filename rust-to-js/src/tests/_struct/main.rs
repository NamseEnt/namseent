struct MyA {
    my_b: i32,
    my_c: String,
    my_d: f64,
}

fn main() {
    let a = MyA {
        my_b: 1,
        my_d: 2.3,
        my_c: String::from("hello"),
    };
    println!("{} {} {}", a.my_b, a.my_c, a.my_d);
}
