use std::{collections::HashMap, rc::Rc};

use crate::{
    ast::{ProtDef, QualName},
    core::BuiltinType,
};

#[derive(Debug)]
pub enum SymType {
    Builtin(BuiltinType),
    Enum,
    Struct,
    Prot(Rc<ProtDef>),
    Func,
    Var,
    Mod,
}

pub type SymTable = HashMap<QualName, SymType>;

pub fn new_with_builtins() -> SymTable {
    let mut symtbl = HashMap::new();

    // add built-in types to the symbol table
    symtbl.insert(
        QualName::from_single("None".to_string()),
        SymType::Builtin(BuiltinType::None),
    );
    symtbl.insert(
        QualName::from_single("Int".to_string()),
        SymType::Builtin(BuiltinType::Int),
    );
    symtbl.insert(
        QualName::from_single("Int8".to_string()),
        SymType::Builtin(BuiltinType::Int8),
    );
    symtbl.insert(
        QualName::from_single("Int16".to_string()),
        SymType::Builtin(BuiltinType::Int16),
    );
    symtbl.insert(
        QualName::from_single("Int32".to_string()),
        SymType::Builtin(BuiltinType::Int32),
    );
    symtbl.insert(
        QualName::from_single("Int64".to_string()),
        SymType::Builtin(BuiltinType::Int64),
    );
    symtbl.insert(
        QualName::from_single("Uint".to_string()),
        SymType::Builtin(BuiltinType::Uint),
    );
    symtbl.insert(
        QualName::from_single("Uint8".to_string()),
        SymType::Builtin(BuiltinType::Uint8),
    );
    symtbl.insert(
        QualName::from_single("Uint16".to_string()),
        SymType::Builtin(BuiltinType::Uint16),
    );
    symtbl.insert(
        QualName::from_single("Uint32".to_string()),
        SymType::Builtin(BuiltinType::Uint32),
    );
    symtbl.insert(
        QualName::from_single("Uint64".to_string()),
        SymType::Builtin(BuiltinType::Uint64),
    );
    symtbl.insert(
        QualName::from_single("Float".to_string()),
        SymType::Builtin(BuiltinType::Float),
    );
    symtbl.insert(
        QualName::from_single("Float16".to_string()),
        SymType::Builtin(BuiltinType::Float16),
    );
    symtbl.insert(
        QualName::from_single("Float32".to_string()),
        SymType::Builtin(BuiltinType::Float32),
    );
    symtbl.insert(
        QualName::from_single("Float64".to_string()),
        SymType::Builtin(BuiltinType::Float64),
    );
    symtbl.insert(
        QualName::from_single("Bool".to_string()),
        SymType::Builtin(BuiltinType::Bool),
    );
    symtbl.insert(
        QualName::from_single("String".to_string()),
        SymType::Builtin(BuiltinType::String),
    );

    symtbl
}
