#[macro_export]
macro_rules! simple_error_impl {
    ($error_struct: ident) => {
        impl std::fmt::Display for $error_struct {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
        impl std::error::Error for $error_struct {}
    };
}

#[macro_export]
macro_rules! append_slash {
    ($($x:expr),+ $(,)?) => {{
        let mut result = String::new();
        $(
            let x = $x.to_string();
            if result.is_empty() {
                result = x;
            } else if result.ends_with('/') {
                if x.starts_with('/') {
                    result.push_str(&x[1..]);
                } else {
                    result.push_str(&x);
                }
            } else {
                if x.starts_with('/') {
                    result.push_str(&x);
                } else {
                    result.push('/');
                    result.push_str(&x);
                }
            }
        )+
        result
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_append_slash() {
        assert_eq!(append_slash!["a", "b"], "a/b");
        assert_eq!(append_slash!["a/", "b"], "a/b");
        assert_eq!(append_slash!["a", "/b"], "a/b");
        assert_eq!(append_slash!["a/", "/b"], "a/b");
        assert_eq!(append_slash!["a/", "/b/"], "a/b/");
        assert_eq!(append_slash!["a/", "b/"], "a/b/");
    }
}
