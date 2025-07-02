use proc_macro::{token_stream::IntoIter, *};

pub trait TokenConsume {
    fn consume_any_group(&mut self) -> Group;
    fn consume_group(&mut self, expected_group: impl FnOnce(&Group)) -> Group;
    fn consume_any_ident(&mut self) -> Ident;
    fn try_consume_any_ident(&mut self) -> Option<Ident>;
    fn consume_punct(&mut self, expected_punct: char) -> Punct;
}

impl TokenConsume for IntoIter {
    fn consume_any_group(&mut self) -> Group {
        let token = self.next().expect("Expected group");
        match token {
            TokenTree::Group(group) => group,
            _ => {
                panic!("expected group, but got {token:?}")
            }
        }
    }

    fn consume_group(&mut self, expected_group: impl FnOnce(&Group)) -> Group {
        let token = self.next().expect("Expected group");
        match token {
            TokenTree::Group(group) => {
                (expected_group)(&group);
                group
            }
            _ => {
                panic!("expected group, but got {token:?}")
            }
        }
    }
    fn consume_any_ident(&mut self) -> Ident {
        let token = self.next().expect("Expected ident");
        match token {
            TokenTree::Ident(ident) => ident,
            _ => {
                panic!("expected ident, but got {token:?}")
            }
        }
    }
    fn try_consume_any_ident(&mut self) -> Option<Ident> {
        let token = self.next()?;
        match token {
            TokenTree::Ident(ident) => Some(ident),
            _ => None,
        }
    }

    fn consume_punct(&mut self, expected_punct: char) -> Punct {
        let token = self.next().expect("Expected punct");
        match token {
            TokenTree::Punct(punct) => {
                if punct == expected_punct {
                    punct
                } else {
                    panic!("expected {expected_punct:?}, but got {punct:?}")
                }
            }
            _ => {
                panic!("expected punct {expected_punct:?}, but got {token:?}")
            }
        }
    }
}
