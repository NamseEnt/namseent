struct A<T> {
    a: T,
    b: (T, T),
}

fn foo<T: Copy>(slice: &[T]) {
    let a = A {
        a: slice[0],
        b: (slice[1], slice[2]),
    };
}

fn main() {
    foo(&[1_u8, 2_u8, 3_u8]);
    foo(&[1_u16, 2_u16, 3_u16]);
}

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
