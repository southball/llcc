#[derive(Debug)]
pub enum NodeBinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
    Assign,
}

#[derive(Debug)]
pub enum Node {
    BinaryOp(NodeBinaryOp, Box<Node>, Box<Node>),
    Num(i32),
    LVar(i32),
}
