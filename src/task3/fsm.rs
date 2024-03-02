use crate::common::{InvalidEnumValueError, Res};

use crate::task3::substr_search::boyer_moor_search;
use std::io::Stdin;
use std::time::Instant;

#[derive(Debug)]
enum FinderState {
    Start,
    AwaitingSting,
    AwaitingPattern,
    AwaitingCommand,
}

#[derive(Debug)]
enum FinderCommands {
    EnterString,
    EnterPattern,
    Search,
    SearchCaseInsensitive,
    MainMenu,
    Exit,
}

impl FinderCommands {
    pub fn from_text(value: &str) -> Result<Self, InvalidEnumValueError> {
        match value {
            "string" => Ok(FinderCommands::EnterString),
            "pattern" => Ok(FinderCommands::EnterPattern),
            "search" => Ok(FinderCommands::Search),
            "searchi" => Ok(FinderCommands::SearchCaseInsensitive),
            "main" => Ok(FinderCommands::MainMenu),
            "exit" => Ok(FinderCommands::Exit),
            _ => Err(InvalidEnumValueError),
        }
    }

    pub fn to_text(&self) -> &'static str {
        match self {
            FinderCommands::EnterString => "'string' - Enter string",
            FinderCommands::EnterPattern => "'pattern' - Enter pattern",
            FinderCommands::Search => "'search' - Search",
            FinderCommands::SearchCaseInsensitive => "'searchi' - Search case insensitive",
            FinderCommands::MainMenu => "'main' - Return to main menu",
            FinderCommands::Exit => "'exit' - Exit program",
        }
    }
}

pub struct SimplifiedBoyerMoorStateMachine<'a> {
    state: FinderState,
    stdin: &'a Stdin,
    string: String,
    pattern: String,
}

impl<'a> SimplifiedBoyerMoorStateMachine<'a> {
    pub fn new(stdin: &'a Stdin) -> Self {
        SimplifiedBoyerMoorStateMachine {
            state: FinderState::Start,
            stdin,
            string: String::default(),
            pattern: String::default(),
        }
    }

    fn print(&self) {
        match self.state {
            FinderState::Start => {
                println!(
                    "Available commands:\n{}",
                    [
                        FinderCommands::EnterString.to_text(),
                        FinderCommands::MainMenu.to_text(),
                        FinderCommands::Exit.to_text()
                    ]
                    .join("\n")
                );
            }
            FinderState::AwaitingSting => {
                println!(
                    "Enter string for searching in or\n{}",
                    [
                        FinderCommands::MainMenu.to_text(),
                        FinderCommands::Exit.to_text()
                    ]
                    .join("\n")
                );
            }
            FinderState::AwaitingPattern => {
                println!(
                    "Enter pattern to search for or\n{}",
                    [
                        FinderCommands::EnterString.to_text(),
                        FinderCommands::MainMenu.to_text(),
                        FinderCommands::Exit.to_text()
                    ]
                    .join("\n")
                );
            }
            FinderState::AwaitingCommand => {
                println!("String: {}", self.string);
                println!("Pattern: {}", self.pattern);
                println!(
                    "Available commands:\n{}",
                    [
                        FinderCommands::EnterString.to_text(),
                        FinderCommands::EnterPattern.to_text(),
                        FinderCommands::Search.to_text(),
                        FinderCommands::SearchCaseInsensitive.to_text(),
                        FinderCommands::MainMenu.to_text(),
                        FinderCommands::Exit.to_text()
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
            (_, Some(FinderCommands::MainMenu)) => Res::MainMenu,
            (_, Some(FinderCommands::Exit)) => Res::Exit,
            (
                FinderState::Start | FinderState::AwaitingCommand | FinderState::AwaitingPattern,
                Some(FinderCommands::EnterString),
            ) => {
                self.state = FinderState::AwaitingSting;
                Res::Repeat
            }
            (
                FinderState::Start | FinderState::AwaitingCommand | FinderState::AwaitingSting,
                Some(FinderCommands::EnterPattern),
            ) => {
                self.state = FinderState::AwaitingPattern;
                Res::Repeat
            }
            (FinderState::AwaitingSting, _) => self.handle_string(input),
            (FinderState::AwaitingPattern, _) => self.handle_pattern(input),
            (FinderState::AwaitingCommand, Some(FinderCommands::Search)) => {
                self.search(false);
                Res::Repeat
            }
            (FinderState::AwaitingCommand, Some(FinderCommands::SearchCaseInsensitive)) => {
                self.search(true);
                Res::Repeat
            }
            _ => {
                println!("Invalid command");
                Res::Repeat
            }
        }
    }

    fn search(&self, case_insensitive: bool) -> Vec<usize> {
        let mut now = Instant::now();
        let result = boyer_moor_search(&self.string, &self.pattern, !case_insensitive);
        println!(
            "SBM: Pattern found at positions: {:?}. Microseconds elapsed: {}",
            result,
            now.elapsed().as_micros()
        );

        now = Instant::now();
        let std_result = if case_insensitive {
            self.string
                .to_lowercase()
                .match_indices(&self.pattern.to_lowercase())
                .map(|(i, _)| self.string[..i].chars().count())
                .collect::<Vec<_>>()
        } else {
            self.string
                .match_indices(&self.pattern)
                .map(|(i, _)| self.string[..i].chars().count())
                .collect::<Vec<_>>()
        };
        println!(
            "STD: Pattern found at positions: {:?}. Microseconds elapsed: {}",
            std_result,
            now.elapsed().as_micros()
        );
        result
    }

    fn handle_pattern(&mut self, input: &str) -> Res<()> {
        self.pattern = self.parse_string(input);
        self.state = FinderState::AwaitingCommand;
        Res::Repeat
    }

    fn handle_string(&mut self, input: &str) -> Res<()> {
        self.string = self.parse_string(input);
        self.state = FinderState::AwaitingPattern;
        Res::Repeat
    }

    fn parse_string(&self, input: &str) -> String {
        return input.trim().to_string();
    }

    fn parse_command(&self, input: &str) -> Option<FinderCommands> {
        match FinderCommands::from_text(input.trim()) {
            Ok(command) => Some(command),
            Err(_) => None,
        }
    }
}
