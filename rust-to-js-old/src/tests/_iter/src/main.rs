trait First {
    fn print(&self);
}
struct StructA;
impl First for StructA {
    fn print(&self) {
        println!("StructA");
    }
}

trait FirstB {
    fn print(&self);
}
impl FirstB for StructA {
    fn print(&self) {
        println!("StructA");
    }
}

fn main() {
    let b = 4;
    let a = &[b, 2, 3];
    let a = StructA;
    First::print(&a);
    FirstB::print(&a);
}

// struct HaveDrop {
//     i: i32,
// }
// impl Drop for HaveDrop {
//     fn drop(&mut self) {
//         println!("drop {}", self.i);
//     }
// }

// struct InnerDrop {
//     i: HaveDrop,
// }

// fn main() {
//     let a = || {
//         println!("hello");
//     };
//     a();

//     let mut b = 5;
//     let mut captures = || {
//         b += 1;
//         println!("{}", b);
//     };
//     captures();

//     let b = 5;
//     let move_captures = move || {
//         let mut b = b;
//         b += 1;
//         println!("{}", b);
//     };
//     move_captures();

//     {
//         let c = HaveDrop { i: 1 };
//     }
//     println!("after drop");

//     {
//         let c = InnerDrop {
//             i: HaveDrop { i: 1 },
//         };
//     }
//     println!("after inner drop");
// }

// mod foo;

// struct A {
//     i: i32,
// }
// impl Iterator for A {
//     type Item = i32;
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.i > 3 {
//             None
//         } else {
//             let value = self.i;
//             self.i += 1;
//             Some(value)
//         }
//     }
// }

// fn main() {
//     let bar = foo::Bar::new(1);
//     let a = [1, 2, 3];
//     println!("array");
//     for i in a {
//         println!("{}", i);
//     }
//     println!("vec");
//     let b = vec![1, 2, 3];
//     for i in b {
//         println!("{}", i);
//     }
//     println!("iterator");
//     let c = A { i: 0 };
//     for i in c {
//         println!("{}", i);
//     }
// }
