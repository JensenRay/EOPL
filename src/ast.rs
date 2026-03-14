#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArithOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Number(i32),
    Variable(String),
    Let {
        name: String,
        value: Box<Expr>,
        body: Box<Expr>,
    },
    If {
        test: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Box<Expr>,
    },
    Arithmetic {
        op: ArithOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Zero(Box<Expr>),
    Proc {
        param: String,
        body: Box<Expr>,
    },
    Call {
        operator: Box<Expr>,
        operand: Box<Expr>,
    },
    LetRec {
        name: String,
        param: String,
        func_body: Box<Expr>,
        body: Box<Expr>,
    },
}
