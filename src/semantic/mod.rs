pub mod utils;

use std::collections::HashSet;

use crate::{
    ast::*,
    semantic::utils::*,
    symtbl::{SymTable, SymType},
    utils::{Loc, Span},
};

pub enum SemanticErrorKind {
    Redefinition,
    EnumDuplicate,
    ProtMemberNameConflict,
    StructMemberNameConflict,
    ProtConformance,
    TypeMismatch,
    UnknownType,
    InvalidType,
    FuncParamNameConflict,
}

pub struct SemanticError {
    #[allow(dead_code)]
    pub kind: SemanticErrorKind,
    pub msg: String,
    pub span: Span,
}

/// Check the main program for semantic errors.
pub fn check_program(ast: &Program, sym_table: &mut SymTable) -> Result<(), SemanticError> {
    // First pass: populate symbol table and check for redefinitions
    for constr in ast.constrs.iter() {
        // check for redefinition of constructs
        if let Some(_) = sym_table.insert(
            QualName::from_single(constr.name().clone()),
            constr.to_sym_type(),
        ) {
            return Err(SemanticError {
                kind: SemanticErrorKind::Redefinition,
                msg: "Semantic error: redefinition isn't allowed".into(),
                span: (constr.loc, constr.loc),
            });
        }
    }

    // Second pass: validate protocol graphs and members before any
    // conformance checks
    for constr in ast.constrs.iter() {
        if let ConstructKind::ProtDef(prot_def) = &constr.kind {
            let mut visited: HashSet<QualName> = HashSet::new();
            let mut visiting: HashSet<QualName> = HashSet::new();

            check_protocol_imposition_cycle(
                &prot_def.qualname(),
                sym_table,
                constr.loc,
                &mut visiting,
                &mut visited,
            )?;

            // handle duplicate protocol members, and check function members
            let mut seen_members = HashSet::new();
            for member in prot_def.members.iter() {
                if !seen_members.insert(&member.decl().name) {
                    return Err(SemanticError {
                        kind: SemanticErrorKind::ProtMemberNameConflict,
                        msg: "Semantic error: protocol member name used".into(),
                        span: member.decl().span,
                    });
                }

                match member {
                    ProtMember::Imposition(func_impo) => check_func_decl(func_impo)?,
                    ProtMember::Default(func_def) => check_func_def(func_def, sym_table)?,
                }
            }
        }
    }

    // Third pass: check for semantic errors within non-protocol constructs
    for constr in ast.constrs.iter() {
        match &constr.kind {
            ConstructKind::EnumDef(enum_def) => {
                // handle duplicate enum elements
                let mut seen_elements = HashSet::new();
                for elem in enum_def.elements.iter() {
                    if !seen_elements.insert(&elem.name) {
                        return Err(SemanticError {
                            kind: SemanticErrorKind::EnumDuplicate,
                            msg: "Semantic error: duplicate enum element".into(),
                            span: elem.span,
                        });
                    }
                }
            }
            ConstructKind::StructDef(struct_def) => {
                // handle duplicate struct members and check function members
                let mut seen_members = HashSet::new();
                for member in struct_def.members.iter() {
                    if !seen_members.insert(member.name()) {
                        return Err(SemanticError {
                            kind: SemanticErrorKind::StructMemberNameConflict,
                            msg: "Semantic error: struct member name used".into(),
                            span: member.span,
                        });
                    }

                    match &member.kind {
                        StructMemberKind::VarDecl(var_decl) => check_var_decl(var_decl, sym_table)?,
                        StructMemberKind::FuncDef(func_def) => check_func_def(func_def, sym_table)?,
                    }

                    // check function members
                    if let StructMemberKind::FuncDef(func_def) = &member.kind {
                        check_func_def(func_def, sym_table)?;
                    }
                }

                // check protocol conformance
                let mut prots_queue: Vec<QualName> =
                    collect_unique_impos(&struct_def.impos, constr.loc)?
                        .into_iter()
                        .cloned()
                        .collect();
                let mut visited_prots: HashSet<QualName> = HashSet::new();
                while let Some(prot) = prots_queue.pop() {
                    if !visited_prots.insert(prot.clone()) {
                        continue;
                    }

                    if let Some(SymType::Prot(prot_def)) = sym_table.get(&prot) {
                        if !struct_conforms_to_protocol(&struct_def, &prot_def) {
                            return Err(SemanticError {
                                kind: SemanticErrorKind::ProtConformance,
                                msg: format!(
                                    "Semantic error: struct doesn't conform to protocol '{}'",
                                    prot
                                )
                                .into(),
                                span: (constr.loc, constr.loc),
                            });
                        }

                        // add imposed protocols to the queue
                        prots_queue.extend(
                            collect_unique_impos(&prot_def.impos, constr.loc)?
                                .into_iter()
                                .cloned(),
                        );
                    } else {
                        return Err(SemanticError {
                            kind: SemanticErrorKind::TypeMismatch,
                            msg: format!("Semantic error: no protocol '{}' found in scope", prot),
                            span: (constr.loc, constr.loc),
                        });
                    }
                }
            }
            ConstructKind::FuncDef(func_def) => check_func_def(func_def, sym_table)?,
            ConstructKind::VarDecl(var_decl) => check_var_decl(var_decl, sym_table)?,
            _ => {}
        }
    }

    Ok(())
}

/// Check if a struct conforms to a protocol by
/// ensuring that all imposed members are implemented.
fn struct_conforms_to_protocol(struct_def: &StructDef, prot_def: &ProtDef) -> bool {
    let imposition_members = prot_def
        .members
        .iter()
        .filter(|pm| matches!(pm, ProtMember::Imposition(_)));
    for pm in imposition_members {
        let mut func_decls = struct_def.members.iter().filter_map(|sm| {
            if let StructMemberKind::FuncDef(func_def) = &sm.kind {
                Some(&func_def.decl)
            } else {
                None
            }
        });

        if !func_decls.any(|fd| compare_func_decls(fd, &pm.decl())) {
            return false;
        }
    }
    true
}

/// Check for cycles in protocol imposition graph using DFS.
fn check_protocol_imposition_cycle(
    current: &QualName,
    sym_table: &SymTable,
    constr_loc: Loc,
    visiting: &mut HashSet<QualName>,
    visited: &mut HashSet<QualName>,
) -> Result<(), SemanticError> {
    if visiting.contains(current) {
        return Err(SemanticError {
            kind: SemanticErrorKind::ProtConformance,
            msg: "Semantic error: protocol imposition cycle detected".into(),
            span: (constr_loc, constr_loc),
        });
    }

    if visited.contains(current) {
        return Ok(());
    }

    let prot_def = if let Some(SymType::Prot(prot_def)) = sym_table.get(current) {
        prot_def
    } else {
        return Err(SemanticError {
            kind: SemanticErrorKind::TypeMismatch,
            msg: format!("Semantic error: no protocol '{}' found in scope", current).into(),
            span: (constr_loc, constr_loc),
        });
    };

    visiting.insert(current.clone());

    let impos = collect_unique_impos(&prot_def.impos, constr_loc)?;
    for imp in impos {
        check_protocol_imposition_cycle(imp, sym_table, constr_loc, visiting, visited)?;
    }

    visiting.remove(current);
    visited.insert(current.clone());
    Ok(())
}

/// Check if a type is valid within the given symbol table.
fn check_type_validity(type_: &Type, sym_table: &SymTable, loc: Loc) -> Result<(), SemanticError> {
    match type_ {
        Type::QualName(qn) => {
            if sym_table.get(qn).is_none() {
                return Err(SemanticError {
                    kind: SemanticErrorKind::UnknownType,
                    msg: format!("Semantic error: unknown type '{}'", qn).into(),
                    span: (loc, loc),
                });
            }
        }
        Type::ArrayOf(opt_num, sub_type) => {
            if let Some(num) = opt_num {
                if num <= &0 {
                    return Err(SemanticError {
                        kind: SemanticErrorKind::InvalidType,
                        msg: "Semantic error: array size must be greater than zero".into(),
                        span: (loc, loc),
                    });
                }
            } else {
                return Err(SemanticError {
                    kind: SemanticErrorKind::InvalidType,
                    msg: "Semantic error: array type must have a specific size".into(),
                    span: (loc, loc),
                });
            }

            check_type_validity(sub_type, sym_table, loc)?;
        }
    }

    Ok(())
}

/// Check a variable declaration for semantic errors.
fn check_var_decl(var_decl: &VarDecl, sym_table: &SymTable) -> Result<(), SemanticError> {
    // check variable type none-nullability
    if var_decl.type_.is_none() {
        return Err(SemanticError {
            kind: SemanticErrorKind::InvalidType,
            msg: "Semantic error: `None` type is not allowed for variable declarations".into(),
            span: (var_decl.loc, var_decl.loc),
        });
    }

    // check variable type validity
    check_type_validity(&var_decl.type_, sym_table, var_decl.loc)?;

    // check variable initialization
    if let Some(expr) = &var_decl.value {
        check_type_expr_compatibility(&var_decl.type_, expr)?;
    }

    Ok(())
}

/// Check if a type is compatible with an expression.
///
/// @TODO: implement actual type compatibility logic
fn check_type_expr_compatibility(type_: &Type, expr: &Expr) -> Result<(), SemanticError> {
    /*
    return Err(SemanticError {
        kind: SemanticErrorKind::TypeMismatch,
        msg: "Semantic error: type mismatch".into(),
        span: (var_decl.loc, var_decl.loc),
    });
     */
    Ok(())
}

/// Check a function declaration for semantic errors.
fn check_func_decl(func_decl: &FuncDecl) -> Result<(), SemanticError> {
    let mut seen_params = HashSet::new();
    func_decl.params.iter().try_for_each(|param| {
        if !seen_params.insert(&param.name) {
            return Err(SemanticError {
                kind: SemanticErrorKind::FuncParamNameConflict,
                msg: "Semantic error: duplicate parameter name in function declaration".into(),
                span: func_decl.span,
            });
        }

        if param.type_.is_none() {
            return Err(SemanticError {
                kind: SemanticErrorKind::InvalidType,
                msg: "Semantic error: `None` type is not allowed for function parameters".into(),
                span: func_decl.span,
            });
        }
        Ok(())
    })?;

    Ok(())
}

/// Check a function definition for semantic errors.
///
/// @TODO: Implement function body checks.
fn check_func_def(func: &FuncDef, sym_table: &SymTable) -> Result<(), SemanticError> {
    Ok(())
}
