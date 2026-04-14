use crate::token::Token;

pub fn splice<P>(h: P, t: Vec<P>) -> Vec<P> {
    let mut parts: Vec<P> = Vec::new();
    parts.push(h);
    parts.extend(t);
    parts
}

pub fn extract_ids(h: Token, t: Vec<Token>) -> Vec<String> {
    let ids: Vec<Token> = splice(h, t);
    ids.iter().map(|id| id.identifier()).collect()
}
