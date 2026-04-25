use crate::{ast::QualName, token::Token};

pub fn splice<P>(h: P, t: Vec<P>) -> Vec<P> {
    let mut parts: Vec<P> = Vec::new();
    parts.push(h);
    parts.extend(t);
    parts
}

pub fn extract_ids(h: Token, t: Vec<Token>) -> QualName {
    let ids: Vec<Token> = splice(h, t);
    QualName::new(ids.iter().map(|id| id.identifier()).collect())
}
