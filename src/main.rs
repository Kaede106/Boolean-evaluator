use std::collections::HashMap;
use std::io::{self, Write};
use tree_sitter::Parser;

mod evaluator;

fn optimize() {
    println!("Not implemented yet");
}

fn optimize_and_evaluate() {
    println!("Not implemented yet");
}

fn evaluation() {
    loop {
        // Get user variable Input

        println!("Enter variables separated by spaces (e.g., P Q R):");
        io::stdout().flush().unwrap();
        let mut var_line = String::new();
        io::stdin().read_line(&mut var_line).unwrap();
        let trimmed = var_line.trim();
        if trimmed.eq_ignore_ascii_case("q!") {
            break;
        }

        let variables: Vec<String> = var_line.split_whitespace().map(|s| s.to_string()).collect();

        // Get user bit value input
        println!(
            "Enter truth values (0 or 1) for {:?} separated by spaces (e.g., 1 0 1):",
            variables
        );
        io::stdout().flush().unwrap();
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();

        let trimmed = input_line.trim();
        if trimmed.eq_ignore_ascii_case("q!") {
            break;
        }
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

        let check: Vec<String> = statement_line
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        // Now check if they typed "Q !" safely using the vector:
        if check.len() == 2 && check[0].to_uppercase() == "Q" && check[1] == "!" {
            break;
        }

        // Create parser
        let mut parser: Parser = Parser::new();
        parser
            .set_language(&evaluator::language())
            .expect("Error loading Boolean grammar");

        let source_bytes = source_code.as_bytes();
        let tree = parser.parse(source_bytes, None).unwrap();

        // Recursively evaluate user input
        let result = evaluator::evaluate(tree.root_node(), source_bytes, &env);
        println!("Result: {}", if result { 1 } else { 0 });
    }
}

fn main() {
    println!(
        "Welcome to Boolean Evaluator! You can always quit the program by inputing 'q!' case insensitive.\n"
    );

    loop {
        println!("What would you like to do today?\n");
        println!("E - Evaluate");
        println!("O - Optimize");
        println!("F - Evaluate and Optimize [Unimplemented currently]");
        println!("Q! - QUIT\n");

        let mut user_input: String = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line.");

        match user_input.trim().chars().next() {
            Some(c) => match c {
                'E' => evaluation(),
                'O' => optimize(),
                'F' => optimize_and_evaluate(),
                'Q' | 'q' => break,
                _ => println!("Improper input add input from list!"),
            },
            None => println!("Error no char inputted"),
        }
        continue;
    }
}
