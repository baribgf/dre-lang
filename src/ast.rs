#![allow(dead_code)]

use std::rc::Rc;
use std::{cell::RefCell, fmt::Display};

use crate::{
    symtbl::{SymTable, SymType},
    utils::{Loc, Span},
};

pub type FuncArg = Expr;
pub type Call = Vec<FuncArg>;
pub type Index = Expr;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct QualName(Vec<String>);

impl QualName {
    pub fn new(parts: Vec<String>) -> Self {
        Self(parts)
    }

    pub fn from_single(name: String) -> Self {
        Self(vec![name])
    }
}

impl Display for QualName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.join("::"))
    }
}

#[derive(Debug)]
pub enum Type {
    QualName(QualName),
    ArrayOf(Option<i32>, Box<Type>),
}

impl Type {
    pub fn is_none(&self) -> bool {
        matches!(self, Type::QualName(qn) if qn.0 == ["None"])
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::QualName(n1), Type::QualName(n2)) => n1 == n2,
            (Type::ArrayOf(s1, t1), Type::ArrayOf(s2, t2)) => s1 == s2 && t1 == t2,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct VarDecl {
    pub is_public: bool,
    pub is_const: bool,
    pub name: Rc<String>,
    pub type_: Type,
    pub value: Option<Expr>,
    pub loc: Loc,
}

impl VarDecl {
    pub fn new(
        is_public: bool,
        is_const: bool,
        name: String,
        type_: Type,
        value: Option<Expr>,
        loc: Loc,
    ) -> Self {
        Self {
            is_public,
            is_const,
            name: name.into(),
            type_,
            value,
            loc,
        }
    }
}

#[derive(Debug)]
pub enum ConstructKind {
    EnumDef(EnumDef),
    StructDef(StructDef),
    ProtDef(Rc<RefCell<ProtDef>>),
    FuncDef(FuncDef),
    VarDecl(VarDecl),
}

#[derive(Debug)]
pub struct Construct {
    pub kind: ConstructKind,
    pub loc: Loc,
}

impl Construct {
    pub fn new(kind: ConstructKind, loc: Loc) -> Self {
        Self { kind, loc }
    }

    pub fn name(&self) -> Rc<String> {
        match &self.kind {
            ConstructKind::EnumDef(def) => def.name.clone(),
            ConstructKind::StructDef(def) => def.name.clone(),
            ConstructKind::ProtDef(def) => def.borrow().name.clone(),
            ConstructKind::FuncDef(def) => def.decl.borrow().name.clone(),
            ConstructKind::VarDecl(decl) => decl.name.clone(),
        }
    }

    pub fn to_sym_type(&self) -> SymType {
        match &self.kind {
            ConstructKind::EnumDef(_) => SymType::Enum,
            ConstructKind::StructDef(_) => SymType::Struct,
            ConstructKind::ProtDef(prot_def) => SymType::Prot(prot_def.clone()),
            ConstructKind::FuncDef(_) => SymType::Func,
            ConstructKind::VarDecl(_) => SymType::Var,
        }
    }
}

#[derive(Debug)]
pub struct EnumElem {
    pub name: String,
    pub span: Span,
}

impl EnumElem {
    pub fn new(name: String, span: Span) -> Self {
        Self { name, span }
    }
}

#[derive(Debug)]
pub struct EnumDef {
    pub is_public: bool,
    pub name: Rc<String>,
    pub elements: Vec<EnumElem>,
}

impl EnumDef {
    pub fn new(is_public: bool, name: String, elements: Vec<EnumElem>) -> Self {
        Self {
            is_public,
            name: name.into(),
            elements,
        }
    }
}

#[derive(Debug)]
pub enum StructMemberKind {
    VarDecl(VarDecl),
    FuncDef(FuncDef),
    ProtMember(Rc<RefCell<FuncDef>>),
}

#[derive(Debug)]
pub struct StructMember {
    pub kind: StructMemberKind,
    pub span: Span,
}

impl StructMember {
    pub fn new(kind: StructMemberKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn name(&self) -> Rc<String> {
        match &self.kind {
            StructMemberKind::VarDecl(decl) => decl.name.clone(),
            StructMemberKind::FuncDef(def) => def.decl.borrow().name.clone(),
            StructMemberKind::ProtMember(def) => def.borrow().decl.borrow().name.clone(),
        }
    }
}

#[derive(Debug)]
pub struct StructDef {
    pub is_public: bool,
    pub name: Rc<String>,
    pub impos: Rc<Vec<QualName>>,
    pub members: Vec<StructMember>,
    pub sym_table: Rc<RefCell<SymTable>>,
}

impl StructDef {
    pub fn new(
        is_public: bool,
        name: String,
        impos: Vec<QualName>,
        members: Vec<StructMember>,
    ) -> Self {
        Self {
            is_public,
            name: name.into(),
            impos: impos.into(),
            members,
            sym_table: RefCell::new(SymTable::new()).into(),
        }
    }
}

#[derive(Debug)]
pub struct FuncParam {
    pub name: String,
    pub type_: Type,
}

impl FuncParam {
    pub fn new(name: String, type_: Type) -> Self {
        Self { name, type_ }
    }
}

#[derive(Debug)]
pub struct FuncDecl {
    pub is_public: bool,
    pub name: Rc<String>,
    pub params: Vec<FuncParam>,
    pub ret_type: Type,
    pub span: Span,
}

impl FuncDecl {
    pub fn new(
        is_public: bool,
        name: String,
        params: Vec<FuncParam>,
        ret_type: Type,
        span: Span,
    ) -> Self {
        Self {
            is_public,
            name: name.into(),
            params,
            ret_type,
            span,
        }
    }
}

#[derive(Debug)]
pub struct FuncDef {
    pub decl: Rc<RefCell<FuncDecl>>,
    pub stmts: Vec<Statement>,
    pub sym_table: Rc<RefCell<SymTable>>,
}

impl FuncDef {
    pub fn new(decl: FuncDecl, stmts: Vec<Statement>) -> Self {
        Self {
            decl: RefCell::new(decl).into(),
            stmts,
            sym_table: RefCell::new(SymTable::new()).into(),
        }
    }
}

#[derive(Debug)]
pub enum ProtMember {
    Imposition(Rc<RefCell<FuncDecl>>),
    Default(Rc<RefCell<FuncDef>>),
}

impl ProtMember {
    pub fn name(&self) -> Rc<String> {
        match self {
            ProtMember::Imposition(imp) => imp.borrow().name.clone(),
            ProtMember::Default(def) => def.borrow().decl.borrow().name.clone(),
        }
    }

    pub fn decl(&self) -> Rc<RefCell<FuncDecl>> {
        match self {
            ProtMember::Imposition(imp) => imp.clone(),
            ProtMember::Default(def) => def.borrow().decl.clone(),
        }
    }
}

#[derive(Debug)]
pub struct ProtDef {
    pub is_public: bool,
    pub name: Rc<String>,
    pub impos: Rc<Vec<QualName>>,
    pub members: Vec<ProtMember>,
    pub sym_table: Rc<RefCell<SymTable>>,
}

impl ProtDef {
    pub fn new(
        is_public: bool,
        name: String,
        impos: Vec<QualName>,
        members: Vec<ProtMember>,
    ) -> Self {
        Self {
            is_public,
            name: name.into(),
            impos: impos.into(),
            members,
            sym_table: RefCell::new(SymTable::new()).into(),
        }
    }

    pub fn qualname(&self) -> QualName {
        // @TODO: expand the name to a fully qualified name based
        // on the current module path
        QualName::from_single(self.name.clone().to_string())
    }
}

#[derive(Debug)]
pub enum StatementKind {
    VarDecl(VarDecl),
    FuncDef(FuncDef),
    Assignment(Assignment),
    UntilLoop(CondLoop),
    WhileLoop(CondLoop),
    ForLoop(ForLoop),
    IfElse(IfElse),
    FuncCall(FuncCall),
    Return(Expr),
    Break,
    Skip,
    Increment(LValue),
    Decrement(LValue),
}

#[derive(Debug)]
pub struct Statement {
    pub kind: StatementKind,
    pub span: Span,
}

impl Statement {
    pub fn new(kind: StatementKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug)]
pub enum PrimaryExprPrime {
    Self_,
    QualName(QualName),
}

#[derive(Debug)]
pub enum PostfixOp {
    Call(Call),
    PostfixOpPrime(PostfixOpPrime),
}

#[derive(Debug)]
pub enum PostfixOpPrime {
    Index(Index),
    Access(String),
}

#[derive(Debug)]
pub struct Assignment {
    lvalue: LValue,
    rvalue: Expr,
}

impl Assignment {
    pub fn new(lvalue: LValue, rvalue: Expr) -> Self {
        Self { lvalue, rvalue }
    }
}

#[derive(Debug)]
pub struct LValue {
    head: PrimaryExprPrime,
    tail: Option<(Vec<PostfixOp>, PostfixOpPrime)>,
}

impl LValue {
    pub fn new(head: PrimaryExprPrime, tail: Option<(Vec<PostfixOp>, PostfixOpPrime)>) -> Self {
        Self { head, tail }
    }
}

#[derive(Debug)]
pub struct CondLoop {
    pub positive: bool,
    pub condition: Expr,
    pub body: Vec<Statement>,
    pub sym_table: SymTable,
}

impl CondLoop {
    pub fn new(positive: bool, condition: Expr, body: Vec<Statement>) -> Self {
        Self {
            positive,
            condition,
            body,
            sym_table: SymTable::new(),
        }
    }
}

#[derive(Debug)]
pub struct ForLoop {
    pub iter_var: String,
    pub range: Expr,
    pub body: Vec<Statement>,
    pub sym_table: SymTable,
}

impl ForLoop {
    pub fn new(iter_var: String, range: Expr, body: Vec<Statement>) -> Self {
        Self {
            iter_var,
            range,
            body,
            sym_table: SymTable::new(),
        }
    }
}

#[derive(Debug)]
pub struct If {
    pub condition: Expr,
    pub body: Vec<Statement>,
    pub sym_table: SymTable,
}

impl If {
    pub fn new(condition: Expr, body: Vec<Statement>) -> Self {
        Self {
            condition,
            body,
            sym_table: SymTable::new(),
        }
    }
}

#[derive(Debug)]
pub struct Else {
    pub body: Vec<Statement>,
    pub sym_table: SymTable,
}

impl Else {
    pub fn new(body: Vec<Statement>) -> Self {
        Self {
            body,
            sym_table: SymTable::new(),
        }
    }
}

#[derive(Debug)]
pub struct IfElse {
    pub if_part: If,
    pub else_if_parts: Vec<If>,
    pub else_part: Option<Else>,
}

impl IfElse {
    pub fn new(if_part: If, else_if_parts: Vec<If>, else_part: Option<Else>) -> Self {
        Self {
            if_part,
            else_if_parts,
            else_part,
        }
    }
}

#[derive(Debug)]
pub struct FuncCall {
    head: PrimaryExprPrime,
    tail: Vec<PostfixOp>,
    call: Call,
}

impl FuncCall {
    pub fn new(head: PrimaryExprPrime, tail: Vec<PostfixOp>, call: Call) -> Self {
        Self { head, tail, call }
    }
}

#[derive(Debug)]
pub enum AddOp {
    Add,
    Sub,
}

#[derive(Debug)]
pub enum MulOp {
    Mul,
    Div,
    Mod,
}

#[derive(Debug)]
pub enum ShiftOp {
    Left,
    Right,
}

#[derive(Debug)]
pub enum RelOp {
    Eq,
    Diamond,
    Lt,
    Lte,
    Gt,
    Gte,
    Id, // the identity operator!
}

#[derive(Debug)]
pub enum UnaryOp {
    Add,
    Sub,
    Not,
    Tilde,
}

#[derive(Debug)]
pub enum UnaryExpr {
    Term(UnaryOp, Box<UnaryExpr>),
    Postfix(PostfixExpr),
}

#[derive(Debug)]
pub enum PrimaryExpr {
    StringLtrl(String),
    CharLtrl(char),
    IntegerLtrl(i32),
    FloatLtrl(f64),
    BoolLtrl(bool),
    Expr(Expr),
    Prime(PrimaryExprPrime),
}

#[derive(Debug)]
pub struct PostfixExpr {
    head: PrimaryExpr,
    tail: Vec<PostfixOp>,
}

impl PostfixExpr {
    pub fn new(head: PrimaryExpr, tail: Vec<PostfixOp>) -> Self {
        Self { head, tail }
    }
}

pub type Expr = OrExpr;
pub type OrExpr = Vec<AndExpr>;
pub type AndExpr = Vec<BitOr>;
pub type BitOr = Vec<BitXor>;
pub type BitXor = Vec<BitAnd>;
pub type BitAnd = Vec<RelExpr>;

pub type RelExpr = Vec<RelTerm>;
pub type RelTerm = (RelOp, ShiftExpr);

pub type ShiftExpr = Vec<ShiftTerm>;
pub type ShiftTerm = (ShiftOp, RngExpr);

pub type RngExpr = (AddExpr, Option<AddExpr>);

pub type AddExpr = Vec<AddTerm>;
pub type AddTerm = (AddOp, MulExpr);

pub type MulExpr = Vec<MulFact>;
pub type MulFact = (MulOp, UnaryExpr);

#[derive(Debug)]
pub struct UseStmt {
    pub local: bool,
    pub path: QualName,
}

impl UseStmt {
    pub fn new(local: bool, path: QualName) -> Self {
        Self { local, path }
    }
}

pub type Header = Vec<UseStmt>;

#[derive(Debug)]
pub struct Program {
    pub header: Header,
    pub constrs: Vec<Construct>,
    pub sym_table: Rc<RefCell<SymTable>>,
}

impl Program {
    pub fn new(header: Header, constrs: Vec<Construct>) -> Self {
        Self {
            header,
            constrs,
            sym_table: RefCell::new(SymTable::new_with_builtins()).into(),
        }
    }
}
