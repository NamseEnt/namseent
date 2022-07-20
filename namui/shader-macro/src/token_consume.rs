use proc_macro::{token_stream::IntoIter, *};

pub trait TokenConsume {
    fn consume_any_group(&mut self) -> Group;
    fn consume_group(&mut self, expected_group: impl FnOnce(&Group)) -> Group;
    fn consume_any_ident(&mut self) -> Ident;
    fn try_consume_any_ident(&mut self) -> Option<Ident>;
    fn consume_ident(&mut self, expected_ident: &str) -> Ident;
    fn try_consume_ident(&mut self, expected_ident: &str) -> Option<Ident>;
    fn consume_any_punct(&mut self) -> Punct;
    fn consume_punct(&mut self, expected_punct: char) -> Punct;
    fn consume_literal(&mut self) -> Literal;
}

impl TokenConsume for IntoIter {
    fn consume_any_group(&mut self) -> Group {
        let token = self.next().expect("Expected group");
        if let TokenTree::Group(group) = token {
            group
        } else {
            panic!("expected group, but got {:?}", token)
        }
    }

    fn consume_group(&mut self, expected_group: impl FnOnce(&Group)) -> Group {
        let token = self.next().expect("Expected group");
        if let TokenTree::Group(group) = token {
            (expected_group)(&group);
            group
        } else {
            panic!("expected group, but got {:?}", token)
        }
    }
    fn consume_any_ident(&mut self) -> Ident {
        let token = self.next().expect("Expected ident");
        if let TokenTree::Ident(ident) = token {
            ident
        } else {
            panic!("expected ident, but got {:?}", token)
        }
    }
    fn try_consume_any_ident(&mut self) -> Option<Ident> {
        let token = self.next()?;
        if let TokenTree::Ident(ident) = token {
            Some(ident)
        } else {
            None
        }
    }
    fn consume_ident(&mut self, expected_ident: &str) -> Ident {
        let token = self.next().expect("Expected ident");

        if let TokenTree::Ident(ident) = token {
            if ident.to_string().eq(expected_ident) {
                ident
            } else {
                panic!("expected {:?}, but got {:?}", expected_ident, ident)
            }
        } else {
            panic!("expected ident {:?}, but got {:?}", expected_ident, token)
        }
    }
    fn try_consume_ident(&mut self, expected_ident: &str) -> Option<Ident> {
        let token = self.next()?;
        if let TokenTree::Ident(ident) = token {
            if ident.to_string().eq(expected_ident) {
                return Some(ident);
            }
        }

        None
    }

    fn consume_any_punct(&mut self) -> Punct {
        let token = self.next().expect("Expected punct");
        if let TokenTree::Punct(punct) = token {
            punct
        } else {
            panic!("expected punct, but got {:?}", token)
        }
    }

    fn consume_punct(&mut self, expected_punct: char) -> Punct {
        let token = self.next().expect("Expected punct");
        if let TokenTree::Punct(punct) = token {
            if punct == expected_punct {
                punct
            } else {
                panic!("expected {:?}, but got {:?}", expected_punct, punct)
            }
        } else {
            panic!("expected punct {:?}, but got {:?}", expected_punct, token)
        }
    }

    fn consume_literal(&mut self) -> Literal {
        let token = self.next().expect("Expected literal");
        if let TokenTree::Literal(literal) = token {
            literal
        } else {
            panic!("expected literal, but got {:?}", token)
        }
    }
}
