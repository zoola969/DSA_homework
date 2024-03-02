use crate::common::{InvalidEnumValueError, Res};
use crate::task4::parentheses::parentheses;
use std::io::Stdin;

#[derive(Debug)]
enum ParenthesesState {
    Start,
    AwaitingSting,
}

#[derive(Debug)]
enum ParenthesesCommands {
    EnterString,
    MainMenu,
    Exit,
}

impl ParenthesesCommands {
    pub fn from_text(value: &str) -> Result<Self, InvalidEnumValueError> {
        match value {
            "string" => Ok(ParenthesesCommands::EnterString),
            "main" => Ok(ParenthesesCommands::MainMenu),
            "exit" => Ok(ParenthesesCommands::Exit),
            _ => Err(InvalidEnumValueError),
        }
    }

    pub fn to_text(&self) -> &'static str {
        match self {
            ParenthesesCommands::EnterString => "'string' - Enter string",
            ParenthesesCommands::MainMenu => "'main' - Return to main menu",
            ParenthesesCommands::Exit => "'exit' - Exit program",
        }
    }
}

pub struct CheckParenthesesStateMachine<'a> {
    state: ParenthesesState,
    stdin: &'a Stdin,
}

impl<'a> CheckParenthesesStateMachine<'a> {
    pub fn new(stdin: &'a Stdin) -> Self {
        CheckParenthesesStateMachine {
            state: ParenthesesState::Start,
            stdin,
        }
    }

    fn print(&self) {
        match self.state {
            ParenthesesState::Start => {
                println!(
                    "Available commands:\n{}",
                    [
                        ParenthesesCommands::EnterString.to_text(),
                        ParenthesesCommands::MainMenu.to_text(),
                        ParenthesesCommands::Exit.to_text()
                    ]
                    .join("\n")
                );
            }
            ParenthesesState::AwaitingSting => {
                println!(
                    "Enter string for searching in or\n{}",
                    [
                        ParenthesesCommands::MainMenu.to_text(),
                        ParenthesesCommands::Exit.to_text()
                    ]
                    .join("\n")
                );
            }
        }
    }

    pub fn start(&mut self) -> Res<()> {
        let mut buffer = String::new();

        loop {
            self.print();
            if self.stdin.read_line(&mut buffer).is_ok() {
                match self.handle_input(&buffer) {
                    Res::Repeat => (),
                    res => return res,
                }
            } else {
                println!("Error reading input");
            }
            buffer.clear();
        }
    }

    fn handle_input(&mut self, input: &str) -> Res<()> {
        match (&self.state, self.parse_command(input)) {
            (_, Some(ParenthesesCommands::MainMenu)) => Res::MainMenu,
            (_, Some(ParenthesesCommands::Exit)) => Res::Exit,
            (ParenthesesState::Start, Some(ParenthesesCommands::EnterString)) => {
                self.state = ParenthesesState::AwaitingSting;
                Res::Repeat
            }
            (ParenthesesState::AwaitingSting, _) => self.handle_string(input),
            _ => {
                println!("Invalid command");
                Res::Repeat
            }
        }
    }

    fn handle_string(&mut self, input: &str) -> Res<()> {
        parentheses(&self.parse_string(input));
        Res::Repeat
    }

    fn parse_string(&self, input: &str) -> String {
        return input.trim().to_string();
    }

    fn parse_command(&self, input: &str) -> Option<ParenthesesCommands> {
        match ParenthesesCommands::from_text(input.trim()) {
            Ok(command) => Some(command),
            Err(_) => None,
        }
    }
}
