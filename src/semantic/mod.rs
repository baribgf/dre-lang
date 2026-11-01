pub mod utils;

use std::{cell::RefCell, collections::HashSet, rc::Rc};

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
    UnknownProtocol,
    InvalidType,
    FuncParamNameConflict,
    InvalidStmt,
}

pub struct SemanticError {
    #[allow(dead_code)]
    pub kind: SemanticErrorKind,
    pub msg: String,
    pub span: Span,
}

/// Check the main program for semantic errors.
pub fn check_program(program: &Program) -> Result<(), SemanticError> {
    // 3rd pass: check for semantic errors within non-protocol constructs
    for constr in program.constrs.iter() {
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
                        StructMemberKind::VarDecl(var_decl) => {
                            check_var_decl(var_decl, program.sym_table.clone())?
                        }
                        StructMemberKind::FuncDef(func_def) => check_func_def(func_def)?,
                        _ => {}
                    }

                    // check function members
                    if let StructMemberKind::FuncDef(func_def) = &member.kind {
                        check_func_def(func_def)?;
                    }
                }

                // check protocol conformance
                let mut prots_queue: Vec<QualName> =
                    collect_unique_impos(struct_def.impos.clone(), constr.loc)?
                        .into_iter()
                        .collect();
                let mut visited_prots: HashSet<QualName> = HashSet::new();
                while let Some(prot) = prots_queue.pop() {
                    if !visited_prots.insert(prot.clone()) {
                        continue;
                    }

                    let prot_sym_type_raw = program.sym_table.borrow().get(&prot);
                    if let Some(prot_sym_type) = prot_sym_type_raw
                        && let SymType::Prot(ref prot_def) = *prot_sym_type.borrow()
                    {
                        if !struct_conforms_to_protocol(&struct_def, prot_def.clone()) {
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
                            collect_unique_impos(prot_def.borrow().impos.clone(), constr.loc)?
                                .into_iter(),
                        );
                    } else {
                        return Err(SemanticError {
                            kind: SemanticErrorKind::UnknownProtocol,
                            msg: format!("Semantic error: no protocol '{}' found in scope", prot),
                            span: (constr.loc, constr.loc),
                        });
                    }
                }
            }
            ConstructKind::FuncDef(func_def) => check_func_def(func_def)?,
            ConstructKind::VarDecl(var_decl) => {
                check_var_decl(var_decl, program.sym_table.clone())?
            }
            ConstructKind::ProtDef(prot_def) => {
                // handle duplicate protocol members, and check function members
                let mut seen_members = HashSet::new();
                for member in prot_def.borrow().members.iter() {
                    if !seen_members.insert(member.decl().borrow().name.clone()) {
                        return Err(SemanticError {
                            kind: SemanticErrorKind::ProtMemberNameConflict,
                            msg: "Semantic error: protocol member name used".into(),
                            span: member.decl().borrow().span,
                        });
                    }

                    match member {
                        ProtMember::Imposition(func_impo) => {
                            check_func_decl(&*func_impo.as_ref().borrow())?
                        }
                        ProtMember::Default(func_def) => {
                            check_func_def(&*func_def.as_ref().borrow())?
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

/// Check if a struct conforms to a protocol by ensuring
/// that all imposed members are implemented.
fn struct_conforms_to_protocol(struct_def: &StructDef, prot_def: Rc<RefCell<ProtDef>>) -> bool {
    let binding = prot_def.borrow();
    let imposition_members = binding
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

        if !func_decls.any(|fd| compare_func_decls(&*fd.borrow(), &*pm.decl().borrow())) {
            return false;
        }
    }
    true
}

/// Check for cycles in protocol imposition graph using DFS.
fn check_protocol_imposition_cycle(
    current: &QualName,
    sym_table: Rc<RefCell<SymTable>>,
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

    let prot_def = {
        if let Some(sym_type) = sym_table.borrow().get(current)
            && let SymType::Prot(ref prot_def) = *sym_type.borrow()
        {
            prot_def.clone()
        } else {
            return Err(SemanticError {
                kind: SemanticErrorKind::UnknownProtocol,
                msg: format!("Semantic error: no protocol '{}' found in scope", current).into(),
                span: (constr_loc, constr_loc),
            });
        }
    };

    visiting.insert(current.clone());

    let impos = collect_unique_impos(prot_def.borrow().impos.clone(), constr_loc)?;
    for imp in impos {
        check_protocol_imposition_cycle(&imp, sym_table.clone(), constr_loc, visiting, visited)?;
    }

    visiting.remove(current);
    visited.insert(current.clone());
    Ok(())
}

/// Check if a type is valid within the given symbol table.
fn check_type_validity(
    type_: &Type,
    sym_table: Rc<RefCell<SymTable>>,
    loc: Loc,
) -> Result<(), SemanticError> {
    match type_ {
        Type::QualName(qn) => {
            if sym_table.borrow().get(qn).is_none() {
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
///
/// Variable declaration type is checked for presence within
/// the symbol table `sym_table`. If an initialization expression
/// is provided, its type is checked for compatibility with the
/// declared type.
fn check_var_decl(
    var_decl: &VarDecl,
    sym_table: Rc<RefCell<SymTable>>,
) -> Result<(), SemanticError> {
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
/// @TODO: implement actual type compatibility logic.
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
fn check_func_def(func: &FuncDef) -> Result<(), SemanticError> {
    check_func_decl(&*func.decl.borrow())?;

    // check function body statements for semantic errors
    func.stmts.iter().try_for_each(|stmt| {
        match &stmt.kind {
            StatementKind::VarDecl(var_decl) => check_var_decl(var_decl, func.sym_table.clone())?,
            StatementKind::FuncDef(func_def) => check_func_def(func_def)?,
            StatementKind::Assignment(assignment) => {}
            StatementKind::UntilLoop(cond_loop) => {}
            StatementKind::WhileLoop(cond_loop) => {}
            StatementKind::ForLoop(for_loop) => {}
            StatementKind::IfElse(if_else) => {}
            StatementKind::FuncCall(func_call) => {}
            StatementKind::Return(items) => {}
            StatementKind::Increment(lvalue) => {}
            StatementKind::Decrement(lvalue) => {}
            StatementKind::Break => Err(SemanticError {
                kind: SemanticErrorKind::InvalidStmt,
                msg: "Semantic error: break statement outside of a loop".into(),
                span: stmt.span,
            })?,
            StatementKind::Skip => Err(SemanticError {
                kind: SemanticErrorKind::InvalidStmt,
                msg: "Semantic error: skip statement outside of a loop".into(),
                span: stmt.span,
            })?,
        }

        Ok(())
    })?;

    Ok(())
}

/// Link inherited members from imposed protocols to the struct definition.
fn link_struct_def_inherited_members(
    struct_def: &mut StructDef,
    global_sym_table: Rc<RefCell<SymTable>>,
    struct_loc: Loc,
) -> Result<(), SemanticError> {
    let mut queue: Vec<QualName> = collect_unique_impos(struct_def.impos.clone(), struct_loc)?
        .into_iter()
        .collect();
    let mut visited: HashSet<QualName> = HashSet::new();

    while let Some(imp) = queue.pop() {
        if !visited.insert(imp.clone()) {
            continue;
        }

        let prot_sym_type_raw = global_sym_table.borrow().get(&imp);
        if let Some(prot_sym_type) = prot_sym_type_raw
            && let SymType::Prot(ref prot_def) = *prot_sym_type.borrow()
        {
            for member in prot_def.borrow().members.iter() {
                if let ProtMember::Default(func_def) = member {
                    struct_def.members.push(StructMember::new(
                        StructMemberKind::ProtMember(func_def.clone()),
                        func_def.borrow().decl.borrow().span,
                    ));
                }
            }

            queue.extend(
                collect_unique_impos(prot_def.borrow().impos.clone(), struct_loc)?.into_iter(),
            );
        } else {
            return Err(SemanticError {
                kind: SemanticErrorKind::UnknownProtocol,
                msg: format!("Semantic error: no protocol '{}' found in scope", imp),
                span: (struct_loc, struct_loc),
            });
        }
    }

    Ok(())
}

/// Fill and link symbol tables for function definitions and
/// their nested constructs.
fn fill_n_link_func_symbol_tables(func_def: &mut FuncDef) -> Result<(), SemanticError> {
    for stmt in func_def.stmts.iter_mut() {
        match &mut stmt.kind {
            StatementKind::FuncDef(inner_func_def) => {
                if let Some(_) = func_def.sym_table.borrow_mut().insert(
                    QualName::from_single(inner_func_def.decl.borrow().name.to_string()),
                    SymType::Func,
                ) {
                    return Err(SemanticError {
                        kind: SemanticErrorKind::Redefinition,
                        msg: format!(
                            "Semantic error: function '{}' redefined",
                            inner_func_def.decl.borrow().name.to_string()
                        )
                        .into(),
                        span: inner_func_def.decl.borrow().span,
                    });
                }

                inner_func_def
                    .sym_table
                    .borrow_mut()
                    .set_parent(func_def.sym_table.clone());

                fill_n_link_func_symbol_tables(inner_func_def)?;
            }
            StatementKind::VarDecl(inner_var_decl) => {
                if let Some(_) = func_def.sym_table.borrow_mut().insert(
                    QualName::from_single(inner_var_decl.name.to_string()),
                    SymType::Var,
                ) {
                    return Err(SemanticError {
                        kind: SemanticErrorKind::Redefinition,
                        msg: format!(
                            "Semantic error: variable '{}' redefined",
                            inner_var_decl.name.to_string()
                        )
                        .into(),
                        span: (inner_var_decl.loc, inner_var_decl.loc),
                    });
                }
            }
            _ => {}
        }
    }

    Ok(())
}

/// Fill and link symbol tables for all constructs in the program.
fn fill_n_link_symbol_tables(program: &mut Program) -> Result<(), SemanticError> {
    for constr in program.constrs.iter_mut() {
        match &mut constr.kind {
            ConstructKind::StructDef(struct_def) => {
                link_struct_def_inherited_members(
                    struct_def,
                    program.sym_table.clone(),
                    constr.loc,
                )?;

                if let Some(_) = program.sym_table.borrow_mut().insert(
                    QualName::from_single(struct_def.name.to_string()),
                    SymType::Struct,
                ) {
                    return Err(SemanticError {
                        kind: SemanticErrorKind::Redefinition,
                        msg: format!(
                            "Semantic error: struct '{}' redefined",
                            struct_def.name.to_string()
                        )
                        .into(),
                        span: (constr.loc, constr.loc),
                    });
                }
                struct_def
                    .sym_table
                    .borrow_mut()
                    .set_parent(program.sym_table.clone());

                for member in struct_def.members.iter_mut() {
                    match &mut member.kind {
                        StructMemberKind::VarDecl(var_decl) => {
                            if let Some(_) = struct_def.sym_table.borrow_mut().insert(
                                QualName::from_single(var_decl.name.to_string()),
                                SymType::Var,
                            ) {
                                return Err(SemanticError {
                                    kind: SemanticErrorKind::Redefinition,
                                    msg: format!(
                                        "Semantic error: variable '{}' redefined",
                                        var_decl.name.to_string()
                                    )
                                    .into(),
                                    span: (constr.loc, constr.loc),
                                });
                            }
                        }
                        StructMemberKind::FuncDef(func_def) => {
                            if let Some(_) = struct_def.sym_table.borrow_mut().insert(
                                QualName::from_single(func_def.decl.borrow().name.to_string()),
                                SymType::Func,
                            ) {
                                return Err(SemanticError {
                                    kind: SemanticErrorKind::Redefinition,
                                    msg: format!(
                                        "Semantic error: function '{}' redefined",
                                        func_def.decl.borrow().name.to_string()
                                    )
                                    .into(),
                                    span: (constr.loc, constr.loc),
                                });
                            }
                            func_def
                                .sym_table
                                .borrow_mut()
                                .set_parent(struct_def.sym_table.clone());
                            fill_n_link_func_symbol_tables(func_def)?;
                        }
                        StructMemberKind::ProtMember(prot_member) => {
                            if let Some(_) = struct_def.sym_table.borrow_mut().insert(
                                QualName::from_single(
                                    prot_member.borrow().decl.borrow().name.to_string(),
                                ),
                                SymType::Func,
                            ) {
                                return Err(SemanticError {
                                    kind: SemanticErrorKind::Redefinition,
                                    msg: format!(
                                        "Semantic error: protocol member '{}' redefined",
                                        prot_member.borrow().decl.borrow().name.to_string()
                                    )
                                    .into(),
                                    span: (constr.loc, constr.loc),
                                });
                            }
                        }
                    }
                }
            }
            ConstructKind::ProtDef(prot_def) => {
                prot_def
                    .borrow()
                    .sym_table
                    .borrow_mut()
                    .set_parent(program.sym_table.clone());

                for member in prot_def.borrow().members.iter() {
                    if let Some(_) = prot_def.borrow().sym_table.borrow_mut().insert(
                        QualName::from_single(member.name().to_string()),
                        SymType::Func,
                    ) {
                        return Err(SemanticError {
                            kind: SemanticErrorKind::Redefinition,
                            msg: format!("Semantic error: function '{}' redefined", member.name())
                                .into(),
                            span: (constr.loc, constr.loc),
                        });
                    }

                    if let ProtMember::Default(func_def) = member {
                        func_def
                            .borrow()
                            .sym_table
                            .borrow_mut()
                            .set_parent(prot_def.borrow().sym_table.clone());

                        fill_n_link_func_symbol_tables(&mut *func_def.borrow_mut())?;
                    }
                }
            }
            ConstructKind::FuncDef(func_def) => {
                if let Some(_) = program.sym_table.borrow_mut().insert(
                    QualName::from_single(func_def.decl.borrow().name.to_string()),
                    SymType::Func,
                ) {
                    return Err(SemanticError {
                        kind: SemanticErrorKind::Redefinition,
                        msg: format!(
                            "Semantic error: function '{}' redefined",
                            func_def.decl.borrow().name
                        )
                        .into(),
                        span: func_def.decl.borrow().span,
                    });
                }
                func_def
                    .sym_table
                    .borrow_mut()
                    .set_parent(program.sym_table.clone());
                fill_n_link_func_symbol_tables(func_def)?;
            }
            ConstructKind::EnumDef(enum_def) => {
                if let Some(_) = program.sym_table.borrow_mut().insert(
                    QualName::from_single(enum_def.name.to_string()),
                    SymType::Enum,
                ) {
                    return Err(SemanticError {
                        kind: SemanticErrorKind::Redefinition,
                        msg: format!("Semantic error: enum '{}' redefined", enum_def.name).into(),
                        span: (constr.loc, constr.loc),
                    });
                }
            }
            ConstructKind::VarDecl(var_decl) => {
                if let Some(_) = program.sym_table.borrow_mut().insert(
                    QualName::from_single(var_decl.name.to_string()),
                    SymType::Var,
                ) {
                    return Err(SemanticError {
                        kind: SemanticErrorKind::Redefinition,
                        msg: format!("Semantic error: variable '{}' redefined", var_decl.name)
                            .into(),
                        span: (var_decl.loc, var_decl.loc),
                    });
                }
            }
        }
    }

    Ok(())
}

/// Prepare the program for semantic analysis
/// by performing two passes:
/// 1. Validate protocol graphs for cycles and redefinitions.
/// 2. Fill and link symbol tables for all constructs in the program.
pub fn prepare_program(program: &mut Program) -> Result<(), SemanticError> {
    // 1st pass: validate protocol graphs before any checks
    let prot_defs_iter = program.constrs.iter().filter_map(|constr| {
        if let ConstructKind::ProtDef(prot_def) = &constr.kind {
            Some((prot_def.clone(), constr.loc))
        } else {
            None
        }
    });

    for (prot_def, loc) in prot_defs_iter.clone() {
        if let Some(_) = program.sym_table.borrow_mut().insert(
            prot_def.borrow().qualname(),
            SymType::Prot(prot_def.clone()),
        ) {
            return Err(SemanticError {
                kind: SemanticErrorKind::Redefinition,
                msg: format!(
                    "Semantic error: protocol '{}' redefined",
                    prot_def.borrow().qualname()
                )
                .into(),
                span: (loc, loc),
            });
        }
    }

    for (prot_def, loc) in prot_defs_iter {
        let mut visited: HashSet<QualName> = HashSet::new();
        let mut visiting: HashSet<QualName> = HashSet::new();

        check_protocol_imposition_cycle(
            &prot_def.borrow().qualname(),
            program.sym_table.clone(),
            loc,
            &mut visiting,
            &mut visited,
        )?;
    }

    // 2nd pass: fill and link symbol tables for all constructs
    fill_n_link_symbol_tables(program)?;

    Ok(())
}
