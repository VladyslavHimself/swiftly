use crate::ast::ast::{Expression, Statement};
use crate::environment::{Environment, JsValue};
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
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let check = self.evaluate_expression(condition);
                if check.is_truthy() {
                   self.evaluate_statement(*then_branch)
                } else if let Some(else_branch) = else_branch {
                    self.evaluate_statement(*else_branch)
                }
            },
            Statement::While { condition, body } => {
                while self.evaluate_expression(condition.clone()).is_truthy() {
                    self.evaluate_statement(*body.clone())
                }
            }
        }
    }

    fn evaluate_expression(&self, expr: Expression) -> JsValue {
        match expr {
            // Expression::Unary => JsValue::Number(0 as f64),
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

                match operator.as_str() {
                    "+" | "Plus" => left_value.add(right_value),
                    "-" | "Minus" => left_value.substract(right_value),
                    "*" | "Star" => left_value.binary_op(right_value, "Star"),
                    "/" | "Slash" => left_value.binary_op(right_value, "Slash"),
                    "%" | "Percent" => left_value.binary_op(right_value, "Percent"),
                    "RelationOpMore" => left_value.compare(&right_value, "RelationOpMore"),
                    "RelationOpMoreOrEqual" => left_value.compare(&right_value, "RelationOpMoreOrEqual"),
                    "RelationOpLess" => left_value.compare(&right_value, "RelationOpLess"),
                    "RelationOpLessOrEqual" => left_value.compare(&right_value, "RelationOpLessOrEqual"),
                    "EqualityOpEqual" => left_value.compare(&right_value, "EqualityOpEqual"),
                    "EqualityOpNotEqual" => left_value.compare(&right_value, "EqualityOpNotEqual"),
                    _ => panic!("Unknown operator: {}", operator),
                }

            }

            Expression::Unary { operator, operand } => {
                let right_val = self.evaluate_expression(*operand);

                match operator.as_str() {
                    "Minus" => match right_val {
                        JsValue::Number(n) => JsValue::Number(-n),
                        _ => JsValue::Undefined, // TODO: Add NaN (As referenced in Ecma Spec)
                    },
                    "Not" => JsValue::Boolean(!right_val.is_truthy()),
                    _ => panic!("Unknown unary operator: {}", operator),
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
