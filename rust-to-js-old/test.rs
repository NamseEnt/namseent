trait First {
    fn print(&self);
}
trait Second<T> {
    fn print(&self, arg: T);
}
trait Thrid {
    fn print(&self);
}

struct StructA;
impl First for StructA {
    fn print(&self) {
        println!("StructA");
    }
}
impl Second<i32> for StructA {
    fn print(&self, arg: i32) {
        println!("StructA: {}", arg);
    }
}
impl<T> Thrid for T {
    fn print(&self) {
        println!("Thrid");
    }
}

struct StructB {
    q: i32,
}

fn unreached() {
    println!("yeah");
}

fn main() {
    let a = StructA;
    First::print(&a);
    Second::print(&a, 1);
    Thrid::print(&a);

    let b = StructB { q: 1 };
}
