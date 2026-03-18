use crate::environment::Environment;
use crate::parser::ast::ast::{Expression, Statement};
use crate::Token;

pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub(crate) fn new() -> Self {
        Self { env: Environment::new() }
    }

    pub(crate) fn execute(&mut self, program: Vec<Statement>) {
        for statement in program {
            self.evaluate_statement(statement);
        }
    }

    fn evaluate_statement(&mut self, stmt: Statement) {
        match stmt {
            Statement::VariableDeclaration { id, init } => {
                let value = self.evaluate_expression(init);
                println!("Linking variable '{}' with value {:?}", id, value);
                self.env.define(id, value);
            }
        }
    }


    fn evaluate_expression(&self, expr: Expression) -> Token {
        match expr {
            Expression::Literal(token) => token,
            Expression::Identifier(name) => {
                panic!("Variable lookup not implemented yet! :(");
            }
        }
    }
}
