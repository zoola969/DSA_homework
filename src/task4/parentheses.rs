use crate::task4::stack::Stack;

pub fn parentheses(string: &str) {
    let mut parentheses_stack: Stack<char> = Stack::new(string.len());
    for char in string.chars() {
        match (char, parentheses_stack.peek()) {
            ('(', _) => parentheses_stack.push('(').unwrap(),
            (')', Some('(')) => {
                parentheses_stack.pop().unwrap();
            }
            (')', None) => {
                println!("Unmatched ')' parentheses found");
                return;
            }
            _ => (),
        }
    }

    match parentheses_stack.get_len() {
        0 => println!("Parentheses are balanced"),
        _ => println!(
            "Parentheses are not balanced, extra {} '(' parentheses found",
            parentheses_stack.get_len(),
        ),
    };
}
