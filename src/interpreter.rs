use crate::ast::ast::{Expression, Statement};
use crate::environment::{Environment, JsValue};
use crate::js_object::JsObject;
use std::cell::RefCell;
use std::rc::Rc;


pub struct Interpreter {
    env: Rc<RefCell<Environment>>,
    pub object_prototype: Rc<RefCell<JsObject>>,
}

impl Interpreter {
    pub(crate) fn new() -> Self {
        let proto = Rc::new(RefCell::new(JsObject::new(JsValue::Null)));

        proto.borrow_mut().set_property(
            "toString".to_string(),
            // Just for test
            JsValue::String("[object Object]".to_string()),
        );


        Self {
            env: Rc::new(RefCell::new(Environment::new())),
            object_prototype: proto,
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
            Statement::EmptyStatement => {
                self.evaluate_expression(Expression::Literal(JsValue::Undefined));
            },
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
            }
            Statement::While { condition, body } => {
                while self.evaluate_expression(condition.clone()).is_truthy() {
                    self.evaluate_statement(*body.clone())
                }
            }
        }
    }

    fn evaluate_expression(&self, expr: Expression) -> JsValue {
        match expr {
            Expression::ObjectLiteral(props) => {
                let obj = Rc::new(RefCell::new(JsObject::new(JsValue::Object(self.object_prototype.clone()))));
                for (key, val_expr) in props {
                    let value = self.evaluate_expression(val_expr);
                    obj.borrow_mut().set_property(key, value);
                }
                JsValue::Object(obj)
            }
            Expression::MemberAccess { object, property } => {
                let obj_value = self.evaluate_expression(*object);
                if let JsValue::Object(obj_ptr) = obj_value {
                    obj_ptr.borrow().get_property(&property)
                } else {
                    panic!(
                        "TypeError: Cannot read property '{}' of {:?}",
                        property, obj_value
                    );
                }
            }

            Expression::PropertyAssignment { object, property, value } => {
                let obj_value = self.evaluate_expression(*object);

                let new_val = self.evaluate_expression(*value);

                if let JsValue::Object(obj_ptr) = obj_value {
                    obj_ptr.borrow_mut().set_property(property, new_val.clone());
                    new_val
                } else {
                    panic!(
                        "TypeError: Cannot assign to property '{}' of {:?}",
                        property, obj_value
                    );
                }
            }

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
                    "+" | "Operator(Plus)" => left_value.add(right_value),
                    "-" | "Operator(Minus)" => left_value.substract(right_value),
                    "*" | "Operator(Star)" => left_value.binary_op(right_value, "Star"),
                    "/" | "Operator(Slash)" => left_value.binary_op(right_value, "Slash"),
                    "%" | "Operator(Percent)" => left_value.binary_op(right_value, "Percent"),
                    "Operator(Greater)" => left_value.compare(&right_value, "Greater"),
                    "Operator(GreaterOrEqual)" => {
                        left_value.compare(&right_value, "GreaterOrEqual")
                    }
                    "Operator(Less)" => left_value.compare(&right_value, "Less"),
                    "Operator(LessOrEqual)" => left_value.compare(&right_value, "LessOrEqual"),
                    "Operator(Equal)" => left_value.compare(&right_value, "Equal"),
                    "Operator(NotEqual)" => left_value.compare(&right_value, "NotEqual"),
                    _ => panic!("Unknown operator: {}", operator),
                }
            }

            Expression::Unary { operator, operand } => {
                let right_val = self.evaluate_expression(*operand);

                match operator.as_str() {
                    "Operator(Minus)" => match right_val {
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
