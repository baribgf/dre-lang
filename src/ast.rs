#![allow(dead_code)]

pub type QualName = Vec<String>;
pub type FuncArg = Expr;
pub type Call = Vec<FuncArg>;
pub type Index = Expr;

#[derive(Debug)]
pub enum Type {
    QualName(QualName),
    ArrayOf(Option<i32>, Box<Type>),
}

#[derive(Debug)]
pub struct VarDecl {
    is_public: bool,
    is_const: bool,
    name: String,
    type_: Type,
    value: Option<Expr>,
}

impl VarDecl {
    pub fn new(
        is_public: bool,
        is_const: bool,
        name: String,
        type_: Type,
        value: Option<Expr>,
    ) -> Self {
        Self {
            is_public,
            is_const,
            name,
            type_,
            value,
        }
    }
}

#[derive(Debug)]
pub enum Construct {
    EnumDef(EnumDef),
    StructDef(StructDef),
    ProtDef(ProtDef),
    FuncDef(FuncDef),
    VarDecl(VarDecl),
}

#[derive(Debug)]
pub struct EnumElem {
    name: String,
    value: Option<i32>,
}

impl EnumElem {
    pub fn new(name: String, value: Option<i32>) -> Self {
        Self { name, value }
    }
}

#[derive(Debug)]
pub struct EnumDef {
    is_public: bool,
    name: String,
    elements: Vec<EnumElem>,
}

impl EnumDef {
    pub fn new(is_public: bool, name: String, elements: Vec<EnumElem>) -> Self {
        Self {
            is_public,
            name,
            elements,
        }
    }
}

#[derive(Debug)]
pub enum StructMember {
    VarDecl(VarDecl),
    FuncDef(FuncDef),
}

#[derive(Debug)]
pub struct StructDef {
    is_public: bool,
    name: String,
    impos: Vec<QualName>,
    members: Vec<StructMember>,
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
            name,
            impos,
            members,
        }
    }
}

#[derive(Debug)]
pub struct FuncParam {
    name: String,
    type_: Type,
}

impl FuncParam {
    pub fn new(name: String, type_: Type) -> Self {
        Self { name, type_ }
    }
}

#[derive(Debug)]
pub struct FuncDecl {
    is_public: bool,
    name: String,
    params: Vec<FuncParam>,
    ret_type: Type,
}

impl FuncDecl {
    pub fn new(is_public: bool, name: String, params: Vec<FuncParam>, ret_type: Type) -> Self {
        Self {
            is_public,
            name,
            params,
            ret_type,
        }
    }
}

#[derive(Debug)]
pub struct FuncDef {
    decl: FuncDecl,
    stmts: Vec<Statement>,
}

impl FuncDef {
    pub fn new(decl: FuncDecl, stmts: Vec<Statement>) -> Self {
        Self { decl, stmts }
    }
}

#[derive(Debug)]
pub struct ProtMember {
    decl: FuncDecl,
    body: Option<Vec<Statement>>,
}

impl ProtMember {
    pub fn new(decl: FuncDecl, body: Option<Vec<Statement>>) -> Self {
        Self { decl, body }
    }
}

#[derive(Debug)]
pub struct ProtDef {
    is_public: bool,
    name: String,
    impos: Vec<QualName>,
    members: Vec<ProtMember>,
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
            name,
            impos,
            members,
        }
    }
}

#[derive(Debug)]
pub enum Statement {
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
    positive: bool,
    condition: Expr,
    body: Vec<Statement>,
}

impl CondLoop {
    pub fn new(positive: bool, condition: Expr, body: Vec<Statement>) -> Self {
        Self {
            positive,
            condition,
            body,
        }
    }
}

#[derive(Debug)]
pub struct ForLoop {
    iter_var: String,
    range: Expr,
    body: Vec<Statement>,
}

impl ForLoop {
    pub fn new(iter_var: String, range: Expr, body: Vec<Statement>) -> Self {
        Self {
            iter_var,
            range,
            body,
        }
    }
}

#[derive(Debug)]
pub struct If {
    condition: Expr,
    body: Vec<Statement>,
}

impl If {
    pub fn new(condition: Expr, body: Vec<Statement>) -> Self {
        Self { condition, body }
    }
}

#[derive(Debug)]
pub struct IfElse {
    if_part: If,
    else_if_parts: Vec<If>,
    else_part: Vec<Statement>,
}

impl IfElse {
    pub fn new(if_part: If, else_if_parts: Vec<If>, else_part: Vec<Statement>) -> Self {
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
    local: bool,
    path: Vec<String>,
}

impl UseStmt {
    pub fn new(local: bool, path: Vec<String>) -> Self {
        Self { local, path }
    }
}

pub type Header = Vec<UseStmt>;

#[derive(Debug)]
pub struct Program {
    header: Header,
    constrs: Vec<Construct>,
}

impl Program {
    pub fn new(header: Header, constrs: Vec<Construct>) -> Self {
        Self { header, constrs }
    }
}
