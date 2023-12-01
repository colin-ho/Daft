use std::collections::HashMap;

use common_treenode::{Transformed, TreeNode, VisitRecursion};

use super::expr::Expr;

pub fn get_required_columns(e: &Expr) -> Vec<String> {
    let mut cols = vec![];
    e.apply(&mut |expr| {
        if let Expr::Column(name) = expr {
            cols.push(name.as_ref().into());
        }
        Ok(VisitRecursion::Continue)
    })
    .expect("Error occurred when visiting for required columns");
    cols
}

pub fn requires_computation(e: &Expr) -> bool {
    // Returns whether or not this expression runs any computation on the underlying data
    match e {
        Expr::Alias(child, _) => requires_computation(child),
        Expr::Column(..) | Expr::Literal(_) => false,
        Expr::Agg(..)
        | Expr::BinaryOp { .. }
        | Expr::Cast(..)
        | Expr::Function { .. }
        | Expr::Not(..)
        | Expr::IsNull(..)
        | Expr::IfElse { .. } => true,
    }
}

pub fn replace_columns_with_expressions(expr: &Expr, replace_map: &HashMap<String, Expr>) -> Expr {
    expr.clone()
        .transform(&|e| {
            if let Expr::Column(ref name) = e && let Some(tgt) = replace_map.get(name.as_ref()) {
                Ok(Transformed::Yes(tgt.clone()))
            } else {
                Ok(Transformed::No(e))
            }
        })
        .expect("Error occurred when rewriting column expressions")
}
