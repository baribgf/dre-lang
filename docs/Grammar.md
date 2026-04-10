# `Ima` Language Grammar (CFG)

```
Program -> Header Globals Constructs

Header -> (UseStmt <Newline>+)*
UseStmt -> <Use> QualName

QualName -> <Identifier> (<DoubleColon> <Identifier>)*

Globals -> (VarDecl <Newline>+)*

VarDecl -> VarDeclHead VarDeclBody
VarDeclHead -> <Let> <Pub>?
VarDeclBody -> <Identifier> <SpacedColon> Type (<LeftArrow> Expr)?
             | <Const> <Identifier> <SpacedColon> Type <Eq> Expr

Type -> QualName
      | <OpenBracket> <IntegerLtrl>? Type <ClosBracket>
TypeDeclHead -> <Let> <Pub>? <Identifier> <SpacedColon>

Constructs -> (Construct <Newline>+)*
Construct -> EnumDef
           | StructDef
           | ProtDef
           | FuncDef

ProtImpos -> <Is> QualName (<SpacedComma> QualName)*

EnumDef -> TypeDeclHead <Enum> <Newline> <Begin> <Newline> EnumBody <End>
EnumBody -> (<Identifier> (<Eq> <IntegerLtrl>)? <Newline>+)*

StructDef -> TypeDeclHead <Struct> ProtImpos? <Newline> <Begin> <Newline> StructBody <End>
StructBody -> (Member <Newline>+)*

Member -> VarDecl
        | FuncDef

ProtDef -> TypeDeclHead <Prot> ProtImpos? <Newline> <Begin> <Newline> ProtBody <End>
ProtBody -> (FuncDecl (<Newline> FuncBlock)? <Newline>+)*

FuncDef -> FuncDecl <Newline> FuncBlock
FuncDecl -> TypeDeclHead <Func> <OpenParen> FuncParams? <ClosParen> <RightArrow> Type
FuncBlock -> <Begin> <Newline> Statements <End>
FuncParams -> FuncParam (<SpacedComma> FuncParam)*
FuncParam -> <Identifier> <SpacedColon> Type
Statements -> (Statement <Newline>+)*

Statement -> VarDecl
           | FuncDef
           | Assignment
           | UntilLoop
           | WhileLoop
           | ForLoop
           | IfElse
           | FuncCall
           | Return
           | Increment
           | Decrement

FuncCall -> PostfixExpr' <OpenParen> FuncArgs? <ClosParen>

Assignment -> LValue <LeftArrow> RValue
LValue -> (<Self> | QualName) (Index | <Period> <Identifier>)*
RValue -> Expr

UntilLoop -> <Until> Expr DoBlock
WhileLoop -> <While> Expr DoBlock

ForLoop -> <For> <Identifier> <In> Expr DoBlock

DoBlock -> <Do> <Newline> Statements <End>

IfElse -> If ElseIf* Else? <End>
If -> <If> Expr <Then> <Newline> Statements
ElseIf -> <Else> If
Else -> <Else> <Newline> Statements

FuncArgs -> FuncArg (<SpacedComma> FuncArg)*
FuncArg -> Expr

Return -> <Return> Expr
Increment -> <Inc> LValue
Decrement -> <Dec> LValue
Call -> <OpenParen> FuncArgs? <ClosParen>
Index -> <LeftBracket> Expr <RightBracket>

Expr -> OrExpr
OrExpr -> AndExpr (<Or> AndExpr)*
AndExpr -> BitOr (<And> BitOr)*
BitOr -> BitXor (<Pipe> BitXor)*
BitXor -> BitAnd (<Hat> BitAnd)*
BitAnd -> RelExpr (<Ampersand> RelExpr)*
RelExpr -> ShiftExpr (RelOp ShiftExpr)*
ShiftExpr -> RngExpr (ShiftOp RngExpr)*
RngExpr -> AddExpr (<DoublePeriod> AddExpr)?
AddExpr -> MulExpr ((<Add> | <Sub>) MulExpr)*
MulExpr -> UnaryExpr ((<Mul> | <Div> | <Mod>) UnaryExpr)*
UnaryExpr -> (<Add> | <Sub> | <Not> | <Tilde>) UnaryExpr | PostfixExpr
PostfixExpr -> PrimaryExpr PostfixOp*
PostfixExpr' -> PrimaryExpr' PostfixOp*
PostfixOp -> Call
           | Index
           | <Period> <Identifier>
PrimaryExpr -> <StringLtrl>
             | <CharLtrl>
             | <IntegerLtrl>
             | <FloatLtrl>
             | <TrueLtrl>
             | <FalseLtrl>
             | <OpenParen> Expr <ClosParen>
             | PrimaryExpr'

PrimaryExpr' -> <Self>
              | QualName

ShiftOp -> <ShiftL>
         | <ShiftR>

RelOp -> <Eq>
       | <Diamond>
       | <Lt>
       | <Lte>
       | <Gt>
       | <Gte>
```
