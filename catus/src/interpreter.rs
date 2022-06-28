use crate::ast::{Script, ScriptBody, Statement, Expression, MemberExpression, ArgumentList};


pub fn interpret(script: &Script) {

}


fn visit_script_body(body: &ScriptBody) {
    match body {
        ScriptBody::StatementList(stmts) => {
            for s in stmts {
                visit_statement(s);
            }
        },
    }
}

fn visit_statement(stmt: &Statement) {
    match stmt {
        Statement::ExpressionStatement(exprs) => {
            for e in exprs {

            }
        },
    }
}

fn eval_expression(expr: &Expression) {
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
        Expression::MemberExpression(_) => todo!(),
        Expression::NewExpression => todo!(),
        Expression::CallExpression => todo!(),
        Expression::CoverCallExpressionAndAsyncArrowHead(member, arguments) => {

        },
    }
}

fn eval_call_expression(member: &MemberExpression, arguments: &ArgumentList) {
    
}
