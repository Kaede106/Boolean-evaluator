use std::collections::HashMap;
use std::ops::Not;
use tree_sitter::Node;

unsafe extern "C" {
    fn tree_sitter_boolean_algebra() -> tree_sitter::Language;
}

pub fn language() -> tree_sitter::Language {
    unsafe { tree_sitter_boolean_algebra() }
}

pub fn evaluate(node: Node, source: &[u8], env: &HashMap<String, bool>) -> bool {
    match node.kind() {
        "source_file" | "expression" => {
            let child: Node = node.child(0).expect("Expected child expression");
            evaluate(child, source, env)
        }
        "parenthesized_expression" => {
            let expr = node.child_by_field_name("expression").unwrap();
            evaluate(expr, source, env)
        }
        "variable" => {
            let var_name = node.utf8_text(source).unwrap();
            *env.get(var_name)
                .unwrap_or_else(|| panic!("Undefined variable in environment: {}", var_name))
        }
        "not_expression" => {
            let arg = node.child_by_field_name("argument").unwrap();
            evaluate(arg, source, env).not()
        }
        "binary_expression" => {
            let left_node = node.child_by_field_name("left").unwrap();
            let right_node = node.child_by_field_name("right").unwrap();
            let op_node = node.child_by_field_name("operator").unwrap();

            let op_text = op_node.utf8_text(source).unwrap();
            let left_val = evaluate(left_node, source, env);
            let right_val = evaluate(right_node, source, env);

            match op_text {
                "&" | "\\land" | "AND" | "and" => left_val && right_val,
                "|" | "\\lor" | "OR" | "or" => left_val || right_val,
                _ => panic!("Unknown binary operator: {}", op_text),
            }
        }
        _ => panic!("Unexpected AST node kind: {}", node.kind()),
    }
}
