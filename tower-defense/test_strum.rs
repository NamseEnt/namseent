use strum_macros::EnumIter;

#[derive(EnumIter)]
enum TestEnum {
    A(u32),
    B(String),
}

fn main() {}
