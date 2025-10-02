use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

static VALUE: AtomicUsize = AtomicUsize::new(0);

#[unsafe(no_mangle)]
pub extern "C" fn callable_from_c() -> usize {
    println!("before call thread spawn");
    std::thread::spawn(|| {
        println!("hello on thread");
        let value = VALUE.fetch_add(1, Ordering::SeqCst);
        println!("value: {}", value);
    });
    println!("after call thread spawn");
    VALUE.load(Ordering::SeqCst)
}

fn main() {}
