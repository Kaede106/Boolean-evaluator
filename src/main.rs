use std::collections::HashMap;
use std::io::{self, Write};
use std::ops::Not;
use tree_sitter::{Node, Parser};

unsafe extern "C" {
    fn tree_sitter_boolean_algebra() -> tree_sitter::Language;
}

pub fn language() -> tree_sitter::Language {
    unsafe { tree_sitter_boolean_algebra() }
}

/// Recursively evaluates a given source by going down each node until primitive ones occur
///
/// # Examples
/// evaluate(node.root_node, source, env) begins first recursion by starting at root node and
/// going down further on
/// ```
fn evaluate(node: Node, source: &[u8], env: &HashMap<String, bool>) -> bool {
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

fn main() {
    // Get user variable Input
    println!("Enter variables separated by spaces (e.g., P Q R):");
    io::stdout().flush().unwrap();

    let mut var_line = String::new();
    io::stdin().read_line(&mut var_line).unwrap();
    let variables: Vec<String> = var_line.split_whitespace().map(|s| s.to_string()).collect();

    // Get user bit value input
    println!(
        "Enter truth values (0 or 1) for {:?} separated by spaces (e.g., 1 0 1):",
        variables
    );
    io::stdout().flush().unwrap();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();

    let values: Vec<bool> = input_line
        .trim()
        .chars()
        .map(|s| match s {
            '0' => false,
            '1' => true,
            _ => panic!("Truth values must be 0 or 1"),
        })
        .collect();

    assert_eq!(
        variables.len(),
        values.len(),
        "Number of inputs must equal length of bits"
    );

    // Map variables to their truth values
    let env: HashMap<String, bool> = variables
        .iter()
        .zip(values.iter())
        .map(|(v, &val)| (v.clone(), val))
        .collect();

    // Get user sentence to evaluate
    println!("Enter the Boolean statement to evaluate (e.g., P \\land (Q \\lor R)):");
    io::stdout().flush().unwrap();

    let mut statement_line = String::new();
    io::stdin().read_line(&mut statement_line).unwrap();
    let source_code = statement_line.trim();

    // Create parser
    let mut parser: Parser = Parser::new();
    parser
        .set_language(&language())
        .expect("Error loading Boolean grammar");

    let source_bytes = source_code.as_bytes();
    let tree = parser.parse(source_bytes, None).unwrap();

    // Recursively evaluate user input
    let result = evaluate(tree.root_node(), source_bytes, &env);
    println!("Result: {}", if result { 1 } else { 0 });
}
