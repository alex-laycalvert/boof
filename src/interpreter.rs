use crate::{
    expr::{Expr, LiteralValue},
    token::{Token, TokenType},
};
pub struct Interpreter {}

impl Interpreter {
    pub fn evaluate(expr: Expr) -> LiteralValue {
        match expr {
            Expr::Binary(left, op, right) => Self::evaluate_binary(*left, op, *right),
            Expr::Unary(op, expr) => Self::evaluate_unary(op, *expr),
            Expr::Grouping(expr) => Self::evaluate(*expr),
            Expr::Literal(l) => l,
            _ => LiteralValue::Nil,
        }
    }

    fn evaluate_binary(left: Expr, op: Token, right: Expr) -> LiteralValue {
        let left = Self::evaluate(left);
        let right = Self::evaluate(right);

        match op.token_type {
            TokenType::Minus => {
                if let LiteralValue::Number(left) = left {
                    if let LiteralValue::Number(right) = right {
                        return LiteralValue::Number(left - right);
                    }
                }
                LiteralValue::Nil
            }
            TokenType::Slash => {
                if let LiteralValue::Number(left) = left {
                    if let LiteralValue::Number(right) = right {
                        return LiteralValue::Number(left / right);
                    }
                }
                LiteralValue::Nil
            }
            TokenType::Star => {
                if let LiteralValue::Number(left) = left {
                    if let LiteralValue::Number(right) = right {
                        return LiteralValue::Number(left * right);
                    }
                }
                LiteralValue::Nil
            }
            TokenType::Plus => {
                if let LiteralValue::String(left) = left {
                    if let LiteralValue::String(right) = right {
                        return LiteralValue::String(left + &right.clone());
                    }
                } else if let LiteralValue::Number(left) = left {
                    if let LiteralValue::Number(right) = right {
                        return LiteralValue::Number(left + right);
                    }
                }
                LiteralValue::Nil
            }
            TokenType::Greater => {
                if let LiteralValue::Number(left) = left {
                    if let LiteralValue::Number(right) = right {
                        if left > right {
                            return LiteralValue::True;
                        } else {
                            return LiteralValue::False;
                        }
                    }
                }
                LiteralValue::Nil
            }
            TokenType::GreaterEqual => {
                if let LiteralValue::Number(left) = left {
                    if let LiteralValue::Number(right) = right {
                        if left >= right {
                            return LiteralValue::True;
                        } else {
                            return LiteralValue::False;
                        }
                    }
                }
                LiteralValue::Nil
            }
            TokenType::Less => {
                if let LiteralValue::Number(left) = left {
                    if let LiteralValue::Number(right) = right {
                        if left < right {
                            return LiteralValue::True;
                        } else {
                            return LiteralValue::False;
                        }
                    }
                }
                LiteralValue::Nil
            }
            TokenType::LessEqual => {
                if let LiteralValue::Number(left) = left {
                    if let LiteralValue::Number(right) = right {
                        if left <= right {
                            return LiteralValue::True;
                        } else {
                            return LiteralValue::False;
                        }
                    }
                }
                LiteralValue::Nil
            }
            TokenType::EqualEqual => Self::equals(left, right),
            TokenType::BangEqual => Self::negate(Self::equals(left, right)),
            _ => LiteralValue::Nil,
        }
    }

    fn evaluate_unary(op: Token, expr: Expr) -> LiteralValue {
        let val = Self::evaluate(expr);
        match op.token_type {
            TokenType::Bang => Self::is_falsy(val),
            TokenType::Minus => {
                if let LiteralValue::Number(val) = val {
                    return LiteralValue::Number(-val);
                }
                LiteralValue::Nil
            }
            _ => LiteralValue::Nil,
        }
    }

    fn equals(left: LiteralValue, right: LiteralValue) -> LiteralValue {
        match (left, right) {
            (LiteralValue::Nil, LiteralValue::Nil) => LiteralValue::True,
            (LiteralValue::True, LiteralValue::True) => LiteralValue::True,
            (LiteralValue::False, LiteralValue::False) => LiteralValue::True,
            (LiteralValue::Number(left), LiteralValue::Number(right)) => {
                if left == right {
                    return LiteralValue::True;
                }
                LiteralValue::False
            }
            (LiteralValue::String(left), LiteralValue::String(right)) => {
                if left == right {
                    return LiteralValue::True;
                }
                LiteralValue::False
            }
            _ => LiteralValue::False,
        }
    }

    fn negate(value: LiteralValue) -> LiteralValue {
        match value {
            LiteralValue::False | LiteralValue::Nil => LiteralValue::True,
            _ => LiteralValue::False,
        }
    }

    fn is_truthy(value: LiteralValue) -> LiteralValue {
        match value {
            LiteralValue::False | LiteralValue::Nil => LiteralValue::False,
            _ => LiteralValue::True,
        }
    }

    fn is_falsy(value: LiteralValue) -> LiteralValue {
        Self::negate(Self::is_truthy(value))
    }
}
