use std::collections::HashMap;

use crate::{
    ast::{
        ArgumentList, CallExpression, Declaration, Expression, FunctionDeclaration,
        HoistableDeclaration, LeftHandSideExpression, Literal, MemberExpression, PrimaryExpression,
        Script, ScriptBody, Statement, StatementListItem,
    },
    builtins::{make_object, Console, Function},
    symtbl::{JsObject, JsValue, Symbol},
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
                    self.eval_statement_list_item(s);
                }
            }
        }
    }

    fn eval_statement_list_item(&mut self, item: &StatementListItem) -> JsValue {
        match item {
            StatementListItem::Statement(s) => self.eval_statement(s),
            StatementListItem::Declaration(d) => self.eval_declaration(d),
        }

        JsValue::Undefined
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
        let function_proto = self.global.get("Function").unwrap();
        let f = Function::new_js_function(function_proto.clone(), decl);
        self.global.insert(f.name().clone(), f);
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
            Expression::AssignmentExpression(a) => todo!(),
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
            Expression::LeftHandSideExpression(lhs_expr) => self.eval_lhs_expression(lhs_expr),
        }
    }

    fn eval_lhs_expression(&mut self, lhs_expr: &LeftHandSideExpression) -> JsValue {
        match lhs_expr {
            LeftHandSideExpression::MemberExpression(m) => self.eval_member_expression(m),
            LeftHandSideExpression::NewExpression => todo!(),
            LeftHandSideExpression::CallExpression(c) => self.eval_call_expression(c),
        }
    }

    fn eval_call_expression(&mut self, call_expr: &CallExpression) -> JsValue {
        match call_expr {
            CallExpression::CoverCallExpressionAndAsyncArrowHead(m, a) => self.eval_call(m, a),
        }
    }

    fn eval_call(&mut self, member: &MemberExpression, arguments: &ArgumentList) -> JsValue {
        let lhs = self.eval_member_expression(member);
        let a = self.eval_argument_list(arguments);

        match lhs {
            JsValue::Undefined => todo!(),
            JsValue::Null => todo!(),
            JsValue::Boolean(_) => todo!(),
            JsValue::String(_) => todo!(),
            JsValue::Number(_) => todo!(),
            JsValue::BigInt => todo!(),
            JsValue::Object(f) => {
                let f = f.borrow();
                self.eval_js_function_call(&f)
            }
            JsValue::NativeFunctionProxy(mut p) => p.function()(&a),
            JsValue::FunctionDeclaration(_) => todo!(),
        }
    }

    fn eval_js_function_call(&mut self, obj: &JsObject) -> JsValue {
        if !obj.is_function() {
            return JsValue::Undefined;
        }

        if let JsValue::FunctionDeclaration(decl) = obj.get_property_value("__function__") {
            if let Some(items) = decl.body.0.as_ref() {
                for item in items {
                    self.eval_statement_list_item(item);
                }
            }
        }

        JsValue::Undefined
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
                    JsValue::NativeFunctionProxy(_) => todo!(),
                    JsValue::FunctionDeclaration(_) => todo!(),
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
