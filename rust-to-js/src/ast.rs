/// Abstract Syntax Tree for MIR
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Function(Function),
    Const(Const),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub return_type: Type,
    pub locals: Vec<Local>,
    pub blocks: Vec<BasicBlock>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Const {
    pub name: String,
    pub ty: Type,
    pub locals: Vec<Local>,
    pub blocks: Vec<BasicBlock>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Local {
    pub mutable: bool,
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BasicBlock {
    pub label: String,
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Assign {
        target: String,
        value: Expr,
    },
    Return,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Const(String),
    Copy(String),
    Move(String),
    Call {
        function: Box<Expr>,
        args: Vec<Expr>,
        target: Option<String>, // return bb label
    },
    Array(Vec<Expr>),
    Ref(Box<Expr>),
    Path(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Unit,
    Array {
        element: Box<Type>,
        size: Option<usize>,
    },
    Ref(Box<Type>),
    Path(String),
}
