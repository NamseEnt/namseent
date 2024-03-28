mod gem_td;
mod path_finding;

pub fn main() {
    namui::start(|| gem_td::Game {})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
        panic!();
    }
}
