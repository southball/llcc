use crate::node::{Node, NodeBinaryOp};

pub struct Codegen;

impl Codegen {
    fn gen_lval(lval: &Node) -> String {
        let mut code = String::new();

        match lval {
            Node::LVar(offset) => {
                code += "  mov rax, rbp\n";
                code += &format!("  sub rax, {}\n", offset);
                code += "  push rax\n";
            }
            _ => unreachable!(),
        }

        code
    }

    fn gen_stmt(stmt: &Node) -> String {
        let mut code = String::new();

        match stmt {
            Node::Num(num) => {
                code += &format!("  push {}\n", num);
                return code;
            }
            Node::LVar(_offset) => {
                code += &Codegen::gen_lval(stmt);
                code += "  pop rax\n";
                code += "  mov rax, [rax]\n";
                code += "  push rax\n";
                return code;
            }
            Node::BinaryOp(NodeBinaryOp::Assign, lhs, rhs) => {
                code += &Codegen::gen_lval(lhs);
                code += &Codegen::gen_stmt(rhs);
                code += "  pop rdi\n";
                code += "  pop rax\n";
                code += "  mov [rax], rdi\n";
                code += "  push rdi\n";
                return code;
            }
            Node::BinaryOp(op, lhs, rhs) => {
                code += &Codegen::gen_stmt(lhs);
                code += &Codegen::gen_stmt(rhs);
                code += "  pop rdi\n";
                code += "  pop rax\n";

                match op {
                    NodeBinaryOp::Add => {
                        code += "  add rax, rdi\n";
                    }
                    NodeBinaryOp::Sub => {
                        code += "  sub rax, rdi\n";
                    }
                    NodeBinaryOp::Mul => {
                        code += "  imul rax, rdi\n";
                    }
                    NodeBinaryOp::Div => {
                        code += "  cqo\n";
                        code += "  idiv rdi\n";
                    }
                    NodeBinaryOp::Eq | NodeBinaryOp::Ne | NodeBinaryOp::Le | NodeBinaryOp::Lt => {
                        code += "  cmp rax, rdi\n";
                        code += &format!(
                            "  {} al\n",
                            match op {
                                NodeBinaryOp::Eq => "sete",
                                NodeBinaryOp::Ne => "setne",
                                NodeBinaryOp::Le => "setle",
                                NodeBinaryOp::Lt => "setl",
                                _ => unreachable!(),
                            }
                        );
                        code += "  movzb rax, al\n";
                    }
                    _ => unreachable!()
                }

                code += "  push rax\n";
            }
        }

        code
    }

    pub fn gen(stmts: Vec<Node>) -> String {
        let mut code = String::new();

        code += ".intel_syntax noprefix\n";
        code += ".globl main\n";
        code += "main:\n";

        code += "  push rbp\n";
        code += "  mov rbp, rsp\n";
        code += "  sub rsp, 208\n";

        for stmt in stmts.iter() {
            code += &Codegen::gen_stmt(stmt);
            code += "  pop rax\n";
        }

        code += "  mov rsp, rbp\n";
        code += "  pop rbp\n";
        code += "  ret\n";

        code
    }
}
