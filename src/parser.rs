use crate::{
    node::{Node, NodeBinaryOp},
    token::{Token, TokenData},
};

pub struct ParseError(pub usize, pub String);
type ParseResult<T> = Result<T, ParseError>;

pub struct Parser {
    tokens: Vec<Token>,
    head: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, head: 0 }
    }

    fn eof(&self) -> bool {
        self.tokens[self.head].data == TokenData::Eof
    }

    pub fn consume_reserved(&mut self, token: &str) -> Option<Token> {
        if let TokenData::Reserved(reserved) = self.tokens[self.head].data.clone() {
            if reserved == token {
                self.head += 1;
                return Some(self.tokens[self.head - 1].clone());
            }
        }

        None
    }

    pub fn consume_ident(&mut self) -> Option<Token> {
        if let TokenData::Ident(_ident) = self.tokens[self.head].data.clone() {
            self.head += 1;
            return Some(self.tokens[self.head - 1].clone());
        }

        None
    }

    pub fn consume_number(&mut self) -> Option<Token> {
        if let TokenData::Num(_num) = self.tokens[self.head].data.clone() {
            self.head += 1;
            return Some(self.tokens[self.head - 1].clone());
        }

        None
    }

    pub fn expect(&self, token: Option<Token>, message: String) -> ParseResult<Token> {
        match token {
            Some(token) => Ok(token),
            None => Err(ParseError(self.head, message)),
        }
    }

    pub fn parse_primary(&mut self) -> ParseResult<Node> {
        if let Some(_) = self.consume_reserved("(") {
            let node = self.parse_expr();
            let token = self.consume_reserved(")");
            self.expect(token, "Expected close parenthesis.".to_string())?;
            return node;
        }

        if let Some(token) = self.consume_ident() {
            let TokenData::Ident(ident) = token.data else {panic!()};
            return Ok(Node::LVar(
                (ident.chars().next().unwrap() as i32) - ('a' as i32),
            ));
        }

        let token = self.consume_number();
        let TokenData::Num(number) = self
            .expect(token, "Expect number.".to_string())?
            .data else {unreachable!()};
        Ok(Node::Num(number))
    }

    pub fn parse_unary(&mut self) -> ParseResult<Node> {
        if let Some(_) = self.consume_reserved("+") {
            self.parse_primary()
        } else if let Some(_) = self.consume_reserved("-") {
            Ok(Node::BinaryOp(
                NodeBinaryOp::Sub,
                Box::new(Node::Num(0)),
                Box::new(self.parse_primary()?),
            ))
        } else {
            self.parse_primary()
        }
    }

    pub fn parse_mul(&mut self) -> ParseResult<Node> {
        let mut node = self.parse_unary()?;
        loop {
            if let Some(_) = self.consume_reserved("*") {
                node = Node::BinaryOp(
                    NodeBinaryOp::Mul,
                    Box::new(node),
                    Box::new(self.parse_unary()?),
                );
            } else if let Some(_) = self.consume_reserved("/") {
                node = Node::BinaryOp(
                    NodeBinaryOp::Div,
                    Box::new(node),
                    Box::new(self.parse_unary()?),
                );
            } else {
                return Ok(node);
            }
        }
    }

    pub fn parse_add(&mut self) -> ParseResult<Node> {
        let mut node = self.parse_mul()?;
        loop {
            if let Some(_) = self.consume_reserved("+") {
                node = Node::BinaryOp(
                    NodeBinaryOp::Add,
                    Box::new(node),
                    Box::new(self.parse_mul()?),
                );
            } else if let Some(_) = self.consume_reserved("-") {
                node = Node::BinaryOp(
                    NodeBinaryOp::Sub,
                    Box::new(node),
                    Box::new(self.parse_mul()?),
                );
            } else {
                return Ok(node);
            }
        }
    }

    pub fn parse_relational(&mut self) -> ParseResult<Node> {
        let mut node = self.parse_add()?;
        loop {
            if let Some(_) = self.consume_reserved("<") {
                node = Node::BinaryOp(
                    NodeBinaryOp::Lt,
                    Box::new(node),
                    Box::new(self.parse_add()?),
                );
            } else if let Some(_) = self.consume_reserved("<=") {
                node = Node::BinaryOp(
                    NodeBinaryOp::Le,
                    Box::new(node),
                    Box::new(self.parse_add()?),
                );
            } else if let Some(_) = self.consume_reserved(">") {
                node = Node::BinaryOp(
                    NodeBinaryOp::Lt,
                    Box::new(self.parse_add()?),
                    Box::new(node),
                );
            } else if let Some(_) = self.consume_reserved(">=") {
                node = Node::BinaryOp(
                    NodeBinaryOp::Le,
                    Box::new(self.parse_add()?),
                    Box::new(node),
                );
            } else {
                return Ok(node);
            }
        }
    }

    pub fn parse_equality(&mut self) -> ParseResult<Node> {
        let mut node = self.parse_relational()?;
        loop {
            if let Some(_) = self.consume_reserved("==") {
                node = Node::BinaryOp(
                    NodeBinaryOp::Eq,
                    Box::new(node),
                    Box::new(self.parse_relational()?),
                );
            } else if let Some(_) = self.consume_reserved("!=") {
                node = Node::BinaryOp(
                    NodeBinaryOp::Ne,
                    Box::new(node),
                    Box::new(self.parse_relational()?),
                );
            } else {
                return Ok(node);
            }
        }
    }

    pub fn parse_assign(&mut self) -> ParseResult<Node> {
        let mut node = self.parse_equality()?;
        if let Some(_) = self.consume_reserved("=") {
            node = Node::BinaryOp(
                NodeBinaryOp::Assign,
                Box::new(node),
                Box::new(self.parse_assign()?),
            );
        }
        Ok(node)
    }

    pub fn parse_expr(&mut self) -> ParseResult<Node> {
        self.parse_assign()
    }

    pub fn parse_stmt(&mut self) -> ParseResult<Node> {
        let node = self.parse_expr()?;
        let token = self.consume_reserved(";");
        self.expect(token, "Semicolon expected.".to_string())?;
        Ok(node)
    }

    pub fn parse_program(&mut self) -> ParseResult<Vec<Node>> {
        let mut stmt = vec![];
        while !self.eof() {
            stmt.push(self.parse_stmt()?);
        }
        Ok(stmt)
    }
}
