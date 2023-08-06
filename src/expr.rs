use crate::token::Token;

#[derive(Debug)]
pub enum LiteralValue {
    String(String),
    Number(f32),
    True,
    False,
    Nil,
}

pub enum Expr {
    /// <expr> <op> <expr>
    Binary(Box<Expr>, Token, Box<Expr>),

    /// <expr>
    Grouping(Box<Expr>),

    /// literal value
    Literal(LiteralValue),

    /// <op> <expr>
    Unary(Token, Box<Expr>),
}

impl Expr {
    pub fn print(&self) {
        println!("{}", self.to_string());
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Binary(left, op, right) => format!(
                "({} {} {})",
                op.to_string(),
                left.to_string(),
                right.to_string()
            ),
            Self::Grouping(expr) => format!("(group {})", expr.to_string()),
            Self::Literal(l) => {
                format!("{l:?}")
            }
            Self::Unary(op, expr) => format!("({} {})", op.to_string(), expr.to_string()),
        }
    }
}
