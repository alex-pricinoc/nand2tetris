use std::fmt;

#[derive(Debug)]
pub struct Ast {
    pub class: Class,
}

#[derive(Debug)]
pub struct Class {
    pub name: String,
    pub vars: Vec<ClassVar>,
    pub subroutines: Vec<Subroutine>,
}

#[derive(Debug)]
pub struct ClassVar {
    pub kind: ClassVarKind,
    pub r#type: Type,
    pub names: Vec<String>,
}

#[derive(Debug, Copy, Clone)]
pub enum ClassVarKind {
    Static,
    Field,
}

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Char,
    Boolean,
    ClassName(String),
}

#[derive(Debug)]
pub struct Subroutine {
    pub kind: SubroutineKind,
    pub name: String,
    pub r#type: ReturnType,
    pub parameters: Vec<Parameter>,
    pub body: SubroutineBody,
}

#[derive(Debug)]
pub enum SubroutineKind {
    Constructor,
    Function,
    Method,
}

#[derive(Debug)]
pub enum ReturnType {
    Void,
    Type(Type),
}

#[derive(Debug)]
pub struct Parameter {
    pub r#type: Type,
    pub name: String,
}

#[derive(Debug)]
pub struct SubroutineBody {
    pub vars: Vec<SubroutineVar>,
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct SubroutineVar {
    pub r#type: Type,
    pub names: Vec<String>,
}

#[derive(Debug)]
pub enum Statement {
    Let {
        var: String,
        accessor: Option<Expression>,
        expression: Expression,
    },
    If {
        cond: Expression,
        statements: Vec<Statement>,
        r#else: Option<Vec<Statement>>,
    },
    While {
        cond: Expression,
        body: Vec<Statement>,
    },
    Do {
        subroutine_call: SubroutineCall,
    },
    Return {
        value: Option<Expression>,
    },
}

#[derive(Debug)]
pub struct Expression(pub Term, pub Vec<(Op, Term)>);

#[derive(Debug)]
pub enum Term {
    Int(u16),
    Str(String),
    Keyword(Keyword),
    Var(String),
    Index(String, Box<Expression>),
    Call(SubroutineCall),
    Paren(Box<Expression>),
    Unary(UnaryOp, Box<Term>),
}

#[derive(Debug)]
pub enum UnaryOp {
    Minus, // -
    Not,   // ~
}

#[derive(Debug)]
pub enum Keyword {
    True,
    False,
    Null,
    This,
}

#[derive(Debug)]
pub enum SubroutineCall {
    Function {
        name: String,
        expressions: Vec<Expression>,
    },
    Method {
        receiver: String,
        name: String,
        expressions: Vec<Expression>,
    },
}

#[derive(Debug)]
pub enum Op {
    Plus,  // +
    Minus, // -
    Star,  // *
    Div,   // /
    And,   // &
    Or,    // |
    Lt,    // <
    Gt,    // >
    Eq,    // =
}

impl fmt::Display for ClassVarKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            ClassVarKind::Static => "static",
            ClassVarKind::Field => "field",
        };

        write!(f, "{s}")
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Type::Int => "int",
            Type::Char => "char",
            Type::Boolean => "boolean",
            Type::ClassName(n) => n,
        };

        write!(f, "{s}")
    }
}

impl fmt::Display for SubroutineKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            SubroutineKind::Constructor => "constructor",
            SubroutineKind::Function => "function",
            SubroutineKind::Method => "method",
        };

        write!(f, "{s}")
    }
}

impl fmt::Display for ReturnType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReturnType::Void => write!(f, "void"),
            ReturnType::Type(t) => write!(f, "{t}"),
        }
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Keyword::True => "true",
            Keyword::False => "false",
            Keyword::Null => "null",
            Keyword::This => "this",
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Op::*;

        let op = match self {
            Plus => "+",
            Minus => "-",
            Star => "*",
            Div => "/",
            Or => "|",
            Eq => "=",

            And => "&amp;",
            Lt => "&lt;",
            Gt => "&gt;",
        };

        write!(f, "{op}")
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let op = match self {
            UnaryOp::Minus => "-",
            UnaryOp::Not => "~",
        };
        write!(f, "{op}")
    }
}
