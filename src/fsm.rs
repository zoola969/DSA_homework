use crate::common::{InvalidEnumValueError, Res};
use crate::task1::fsm::SortStateMachine;
use crate::task2::fsm::Task2StateMachine;
use crate::task3::fsm::SimplifiedBoyerMoorStateMachine;
use crate::task4::fsm::CheckParenthesesStateMachine;
use crate::task5::fsm::KochCurveDrawerStateMachine;
use std::io::Stdin;

#[derive(Debug)]
enum MainState {
    AwaitingTaskNumber,
}

#[derive(Debug)]
enum MainCommands {
    Exit,
}

impl MainCommands {
    pub fn from_text(value: &str) -> Result<Self, InvalidEnumValueError> {
        match value {
            "exit" => Ok(MainCommands::Exit),
            _ => Err(InvalidEnumValueError),
        }
    }

    pub fn to_text(&self) -> &'static str {
        match self {
            MainCommands::Exit => "'exit' - Exit program",
        }
    }
}

pub struct MainStateMachine<'a> {
    state: MainState,
    stdin: &'a Stdin,
}

impl<'a> MainStateMachine<'a> {
    pub fn new(stdin: &'a Stdin) -> Self {
        MainStateMachine {
            state: MainState::AwaitingTaskNumber,
            stdin,
        }
    }

    fn print(&self) {
        match self.state {
            MainState::AwaitingTaskNumber => {
                println!(
                    "Available tasks:\n\
                1 - Sort\n\
                2 - Fibonacci search and simple rehash\n\
                3 - Simplified Boyer-Moor search\n\
                4 - Check parentheses balance\n\
                5 - Draw Koch curve"
                );
                println!(
                    "Enter task number (1-5) for running or\n{}",
                    [MainCommands::Exit.to_text()].join("\n")
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
                    Res::MainMenu => (),
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
            (MainState::AwaitingTaskNumber, _) => self.parse_task_number(input),
        }
    }

    fn parse_task_number(&self, input: &str) -> Res<()> {
        match input.trim().parse::<usize>() {
            Ok(number) => match number {
                1 => SortStateMachine::new(self.stdin).start(),
                2 => Task2StateMachine::new(self.stdin).start(),
                3 => SimplifiedBoyerMoorStateMachine::new(self.stdin).start(),
                4 => CheckParenthesesStateMachine::new(self.stdin).start(),
                5 => KochCurveDrawerStateMachine::new(self.stdin).start(),
                _ => {
                    println!("Invalid task number. Available tasks: 1-5");
                    Res::Repeat
                }
            },
            Err(_) => {
                println!("Invalid task number. Available tasks: 1-5");

                Res::Repeat
            }
        }
    }

    fn parse_command(&self, input: &str) -> Option<MainCommands> {
        match MainCommands::from_text(input.trim()) {
            Ok(command) => Some(command),
            Err(_) => None,
        }
    }
}
