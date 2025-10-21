enum MyA {
    MyB,
    MyC(String),
    MyD { my_e: f64 },
}

fn main() {
    let a = [
        MyA::MyB,
        MyA::MyC(String::from("hello")),
        MyA::MyD { my_e: 1.2 },
    ];
    for i in a {
        match i {
            MyA::MyB => println!("MyB"),
            MyA::MyC(s) => println!("MyC: {}", s),
            MyA::MyD { my_e } => println!("MyD: {}", my_e),
        }
    }
}
