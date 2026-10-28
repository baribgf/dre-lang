use std::collections::HashSet;

use crate::{
    ast::{FuncDecl, QualName},
    semantic::{SemanticError, SemanticErrorKind},
    utils::Loc,
};

pub fn compare_func_decls(fd1: &FuncDecl, fd2: &FuncDecl) -> bool {
    fd1.name == fd2.name
        && fd1.params.len() == fd2.params.len()
        && fd1.ret_type == fd2.ret_type
        && fd1
            .params
            .iter()
            .zip(fd2.params.iter())
            .all(|(p1, p2)| p1.type_ == p2.type_)
}

/// Collects unique `QualName`s of protocol impositions from a vector,
/// returning a `SemanticError` if duplicates are found.
/// # Arguments
/// * `quals` - A vector of `QualName`s to be checked for uniqueness.
/// * `constr_loc` - The location of the construct being checked, used
/// for error reporting if duplicates are found.
/// # Returns
/// * `Ok(HashSet<&QualName>)` - A set of unique `QualName`s if
/// no duplicates are found.
/// * `Err(SemanticError)` - An error indicating that a duplicate `QualName`
/// was found, including the location of the construct.
pub fn collect_unique_impos(
    quals: &Vec<QualName>,
    constr_loc: Loc,
) -> Result<HashSet<&QualName>, SemanticError> {
    let mut impos = HashSet::new();
    for imp in quals.iter() {
        if !impos.insert(imp) {
            return Err(SemanticError {
                kind: SemanticErrorKind::ProtConformance,
                msg: "Semantic error: protocol conformance name used".into(),
                span: (constr_loc, constr_loc),
            });
        }
    }
    Ok(impos)
}
