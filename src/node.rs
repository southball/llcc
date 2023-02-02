#[derive(Debug)]
pub enum NodeBinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Lt,
    Le,
    Eq,
    Ne,
    Assign,
}

#[derive(Debug)]
pub enum Node {
    BinaryOp(NodeBinaryOp, Box<Node>, Box<Node>),
    Return(Box<Node>),
    Num(i32),
    LVar(String, i32),
}

