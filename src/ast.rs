use crate::lexer::tokens::Operator;

#[derive(Clone)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Clone)]
pub struct Item {
    tags: Vec<Tag>,
    item_variant: ItemVariant,
}

#[derive(Clone)]
pub enum ItemVariant {
    Globals {
        t: Type,
        name: String,
        val: Exp,
    },
    TypeDef {
        name: String,
        typedef: TypeDef,
    },
    Fns {
        reciever: Option<Type>,
        name: String,
        params: Vec<Parameter>,
        return_type: Type,
        body: Block,
    },
    Imports {
        path: String,
    },
}

#[derive(Clone)]
pub enum TypeDef {
    Sum { fields: Vec<(Type, String)> },
    Struct { fields: Vec<(Type, String)> },
    Alias { true_type: Type },
    Trait {},
}

#[derive(Clone)]
struct Block {
    statements: Vec<Statement>,
}

#[derive(Clone)]
pub enum Statement {
    If {
        cond: Exp,
        block: Block,
    },
    Declare {
        name: String,
        ty: Option<Type>,
        val: Exp,
    },
    Assign {
        name: String,
        eq: Option<DualOperator>,
        val: Exp,
    },
    Return {
        val: Exp,
    },
    Call {
        name: String,
        params: Vec<Parameter>,
    },
    Block(Block),
    Goto {
        cmd: GotoVariant,
        label: Option<String>,
    },
    For {
        vars: Vec<String>,
        stream: Exp,
        block: Block,
    },
}

#[derive(Clone)]
pub enum GotoVariant {
    Cont,
    BreakOut,
}

#[derive(Clone)]
pub struct Parameter {}

#[derive(Clone)]
pub enum Type {
    Int(i32),
    Bool(bool),
    Custom(String),
    Ref(Box<Type>),
}

#[derive(Clone)]
pub enum Exp {
    Id(String),
    Literal(Literal),
    DoubleOp {
        op: Operator,
        left: Box<Exp>,
        right: Box<Exp>,
    },
    SoloOp {
        op: Operator,
        exp: Box<Exp>,
    },
    Brackets {
        exp: Box<Exp>,
    },
    Index {
        var: Box<Exp>,
        val: Box<Exp>,
    },
    Call {
        name: String,
        args: Vec<Exp>,
    },
    FieldAccess {
        exp: Box<Exp>,
        field: String,
    },
    Method {
        exp: Box<Exp>,
        method: String,
        args: Vec<Exp>,
    },
}

#[derive(Clone)]
pub enum Literal {
    String(String),
    Int(u64),
    Float(f64),
    Array { len: u64, content: Vec<Exp> },
    Tupel { content: Vec<Exp> },
}

#[derive(Clone)]
pub enum DualOperator {
    And,
    Or,
    Xor,
    Plus,
    Minus,
    
}

#[derive(Clone)]
pub enum SoloOperator {
    Neg,
    Minus,
    Deref,
}

#[derive(Clone)]
pub enum Tag {
    Impure,
    Is(Vec<Type>),
    Type(Vec<Type>),
    Test,
    Override,
    Custom(String),
}
