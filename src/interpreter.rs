use crate::environment::{Environment, JsValue};
use crate::parser::ast::ast::{Expression, Statement};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    env: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub(crate) fn new() -> Self {
        Self {
            env: Rc::new(RefCell::new(Environment::new())),
        }
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
                println!("Binding variable '{}' with value {:?}", id, value);
                self.env.borrow_mut().define(id, value);
            }
            Statement::Block(statements) => self.execute_block(statements),
            Statement::Expression(expr) => {
                let value = self.evaluate_expression(expr);
                println!("Evaluated expression to {:?}", value);
            }
        }
    }

    fn evaluate_expression(&self, expr: Expression) -> JsValue {
        match expr {
            Expression::Literal(token) => JsValue::from(token),
            Expression::Identifier(name) => self.env.borrow().get(&name),
            Expression::Assignment { name, value } => {
                let value = self.evaluate_expression(*value);
                let success = self.env.borrow_mut().assign(name.clone(), value.clone());

                if !success {
                    panic!("ReferenceError: {} is not defined", name.to_string() + " =")
                }

                value
            }
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left_value = self.evaluate_expression(*left);
                let right_value = self.evaluate_expression(*right);

                if operator == "+" {
                    left_value.add(right_value)
                } else {
                    JsValue::Undefined
                }
            }
        }
    }

    fn execute_block(&mut self, statements: Vec<Statement>) {
        let previous_env = Rc::clone(&self.env);
        let new_env = Rc::new(RefCell::new(Environment::new_enclosed(Rc::clone(
            &self.env,
        ))));

        self.env = new_env;

        for statement in statements {
            self.evaluate_statement(statement);
        }

        self.env = previous_env;
    }
}
