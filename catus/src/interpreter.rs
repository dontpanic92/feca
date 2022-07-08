use std::collections::HashMap;

use crate::{
    ast::{
        ArgumentList, Declaration, Expression, FunctionDeclaration, HoistableDeclaration, Literal,
        MemberExpression, PrimaryExpression, Script, ScriptBody, Statement, StatementListItem,
    },
    builtins::{make_object, Console, Function, make_new_object},
    symtbl::{JsValue, Symbol},
};

pub struct Interpreter {
    global: HashMap<String, Symbol>,
}

impl Interpreter {
    pub fn new() -> Self {
        let global = builtin_globals();

        Self { global }
    }

    pub fn eval(&mut self, script: &Script) {
        match script {
            Script::ScriptBody(body) => {
                if let Some(body) = body {
                    self.eval_script_body(body)
                }
            }
        }
    }

    fn eval_script_body(&mut self, body: &ScriptBody) {
        match body {
            ScriptBody::StatementList(stmts) => {
                for s in stmts {
                    match s.as_ref() {
                        StatementListItem::Statement(s) => self.eval_statement(s),
                        StatementListItem::Declaration(d) => self.eval_declaration(d),
                    }
                }
            }
        }
    }

    fn eval_declaration(&mut self, decl: &Declaration) {
        match decl {
            Declaration::HoistableDeclaration(d) => self.eval_hoistable_declaration(d),
            Declaration::ClassDeclaration => todo!(),
            Declaration::LexicalDeclaration => todo!(),
        }
    }

    fn eval_hoistable_declaration(&mut self, decl: &HoistableDeclaration) {
        match decl {
            HoistableDeclaration::FunctionDeclaration(f) => self.eval_function_declaration(f),
        }
    }

    fn eval_function_declaration(&mut self, decl: &FunctionDeclaration) {
        let function = self.global.get("Function").unwrap();
        let object = make_new_object(decl.name.unwrap(), function.clone(), );
    }

    fn eval_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::ExpressionStatement(exprs) => {
                for e in exprs {
                    self.eval_expression(e);
                }
            }
        }
    }

    fn eval_expression(&mut self, expr: &Expression) -> JsValue {
        match expr {
            Expression::AssignmentExpression => todo!(),
            Expression::ConditionalExpression => todo!(),
            Expression::ShortCircuitExpression => todo!(),
            Expression::LogicalORExpression => todo!(),
            Expression::LogicalANDExpression => todo!(),
            Expression::BitwiseORExpression => todo!(),
            Expression::BitwiseXORExpression => todo!(),
            Expression::BitwiseANDExpression => todo!(),
            Expression::EqualityExpression => todo!(),
            Expression::RelationalExpression => todo!(),
            Expression::ShiftExpression => todo!(),
            Expression::AdditiveExpression => todo!(),
            Expression::MultiplicativeExpression => todo!(),
            Expression::ExponentiationExpression => todo!(),
            Expression::UnaryExpression => todo!(),
            Expression::UpdateExpression => todo!(),
            Expression::LeftHandSideExpression => todo!(),
            Expression::MemberExpression(m) => self.eval_member_expression(m),
            Expression::NewExpression => todo!(),
            Expression::CallExpression => todo!(),
            Expression::CoverCallExpressionAndAsyncArrowHead(member, arguments) => {
                self.eval_call_expression(member, arguments)
            }
        }
    }

    fn eval_call_expression(&mut self, member: &MemberExpression, arguments: &ArgumentList) -> JsValue {
        let lhs = self.eval_member_expression(member);
        let a = self.eval_argument_list(arguments);

        match lhs {
            JsValue::Undefined => todo!(),
            JsValue::Null => todo!(),
            JsValue::Boolean(_) => todo!(),
            JsValue::String(_) => todo!(),
            JsValue::Number(_) => todo!(),
            JsValue::BigInt => todo!(),
            JsValue::Object(_) => todo!(),
            JsValue::FunctionProxy(mut p) => p.function()(&a),
        }
    }

    fn eval_argument_list(&mut self, arguments: &ArgumentList) -> Vec<JsValue> {
        let mut ret = vec![];
        for a in &arguments.0 {
            ret.push(self.eval_expression(a));
        }

        ret
    }

    fn eval_member_expression(&mut self, member: &MemberExpression) -> JsValue {
        match member {
            MemberExpression::PrimaryExpression(p) => self.eval_primary_expression(p),
            MemberExpression::MemberExpressionDotIdentiferName(member, prop) => {
                let lhs = self.eval_member_expression(&member);
                match lhs {
                    JsValue::Undefined => todo!(),
                    JsValue::Null => todo!(),
                    JsValue::Boolean(_) => todo!(),
                    JsValue::String(_) => todo!(),
                    JsValue::Number(_) => todo!(),
                    JsValue::BigInt => todo!(),
                    JsValue::Object(o) => o.borrow().get_property_value(prop),
                    JsValue::FunctionProxy(_) => todo!(),
                }
            }
        }
    }

    fn eval_primary_expression(&mut self, primary: &PrimaryExpression) -> JsValue {
        match primary {
            PrimaryExpression::This => todo!(),
            PrimaryExpression::IdentifierReference(s) => self
                .global
                .get(s)
                .and_then(|s| Some(s.value()))
                .unwrap_or(JsValue::Undefined),
            PrimaryExpression::Literal(lit) => match lit {
                Literal::NullLiteral => todo!(),
                Literal::BooleanLiteral(_) => todo!(),
                Literal::StringLiteral(s) => JsValue::String(s.clone()),
            },
        }
    }
}

fn builtin_globals() -> HashMap<String, Symbol> {
    let mut globals = HashMap::new();

    let object = make_object();
    let console = Console::make_console(object.clone());
    let function = Function::make_function(object.clone());

    globals.insert("Object".to_string(), object);
    globals.insert("console".to_string(), console);
    globals.insert("Function".to_string(), function);
    globals
}
