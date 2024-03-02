use crate::task4::stack::Stack;
use std::cmp::Ordering;

pub fn parentheses(string: &str) {
    let mut parentheses_stack: Stack<char> = Stack::new(string.len());
    string
        .chars()
        .filter(|char| matches!(char, '(' | ')'))
        .for_each(|char| {
            if let Some(l) = parentheses_stack.peek() {
                match char.cmp(l) {
                    Ordering::Equal => {
                        parentheses_stack.push(char).unwrap();
                    }
                    _ => {
                        parentheses_stack.pop().unwrap();
                    }
                };
            } else {
                parentheses_stack.push(char).unwrap();
            }
        });
    match parentheses_stack.get_len() {
        0 => println!("Parentheses are balanced"),
        _ => println!(
            "Parentheses are not balanced, extra {} '{}' parentheses found",
            parentheses_stack.get_len(),
            parentheses_stack.peek().unwrap()
        ),
    };
}
