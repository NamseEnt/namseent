pub trait ToSnakeCase {
    fn to_snake_case(&self) -> String;
}

impl ToSnakeCase for String {
    fn to_snake_case(&self) -> String {
        let mut result = String::new();
        let mut chars = self.chars().peekable();
        while let Some(c) = chars.next() {
            if c.is_uppercase() {
                if let Some(next) = chars.peek() {
                    if next.is_lowercase() {
                        result.push('_');
                    }
                }
                result.push(c.to_lowercase().next().unwrap());
            } else {
                result.push(c);
            }
        }
        result
    }
}
