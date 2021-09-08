use crate::token::Token;
use std::mem::discriminant;
use crate::debug;

pub fn rules(list: Vec<Token>) -> Vec<Token> {
    let mut k: usize = 0;
    let mut patched_list: Vec<Token> = Vec::new();
    for i in &list {
        if k == list.len() {
            debug!("Returned because index was out of bounds, skipping to next procedure");
            return patched_list;
        }
        if k != 0 {
            if discriminant(&list[k - 1]) == discriminant(&Token::Minus('-')) && discriminant(i) == discriminant(&Token::Arer(String::from(">"))) {
                debug!("Found Arer that would be fatal, skipping, the thing {:#?}", i);
                continue;
            }
        } else {
            debug!("Found unsubtractable, continuing");
        }
        let arer_arrow = String::from(">");
        match i {
            _i if k + 1 == list.len() => break,
            i if matches!(i, &Token::Minus('-')) && matches!(&list[k + 1], Token::Arer(arer_arrow)) => {
                debug!("Found ARER forward... Adding to list, some debug: {0:?}, some more debug: {1:?}", list[k + 1], i);
                patched_list.push(Token::Arer("->".to_string()));
            },
            _ => {
                debug!("Continuing rules iteration, matched underscore _");
                patched_list.push(i.clone());
            }
        }
        k += 1;
    }
    return patched_list;
}