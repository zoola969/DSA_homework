use crate::common::{ArrayGenerationInputStateMachine, InvalidEnumValueError, Res};
use crate::task2::fibonacci::{delete, fib_search, insert};
use crate::task2::hash::CustomHasSet;
use std::io::Stdin;
use std::time::Instant;

#[derive(Debug)]
enum Task2State {
    AwaitingCommand,
}

#[derive(Debug)]
enum Task2Commands {
    Finder,
    Hasher,
    MainMenu,
    Exit,
}

impl Task2Commands {
    pub fn from_text(value: &str) -> Result<Self, InvalidEnumValueError> {
        match value {
            "finder" => Ok(Task2Commands::Finder),
            "hasher" => Ok(Task2Commands::Hasher),
            "main" => Ok(Task2Commands::MainMenu),
            "exit" => Ok(Task2Commands::Exit),
            _ => Err(InvalidEnumValueError),
        }
    }

    pub fn to_text(&self) -> &'static str {
        match self {
            Task2Commands::Finder => "'finder' - Fibonacci search implementation",
            Task2Commands::Hasher => "'hasher' - Simple rehash implementation",
            Task2Commands::MainMenu => "'main' - Return to main menu",
            Task2Commands::Exit => "'exit' - Exit program",
        }
    }
}

pub struct Task2StateMachine<'a> {
    state: Task2State,
    stdin: &'a Stdin,
}

impl<'a> Task2StateMachine<'a> {
    pub fn new(stdin: &'a Stdin) -> Self {
        Task2StateMachine {
            state: Task2State::AwaitingCommand,
            stdin,
        }
    }

    fn print(&self) {
        match self.state {
            Task2State::AwaitingCommand => {
                println!(
                    "Available commands:\n{}",
                    [
                        Task2Commands::Finder.to_text(),
                        Task2Commands::Hasher.to_text(),
                        Task2Commands::MainMenu.to_text(),
                        Task2Commands::Exit.to_text()
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
            (_, Some(Task2Commands::MainMenu)) => Res::MainMenu,
            (_, Some(Task2Commands::Exit)) => Res::Exit,
            (Task2State::AwaitingCommand, Some(Task2Commands::Finder)) => {
                return FibStateMachine::new(self.stdin).start();
            }
            (Task2State::AwaitingCommand, Some(Task2Commands::Hasher)) => {
                return HashStateMachine::new(self.stdin).start();
            }
            _ => {
                println!("Invalid command");
                Res::Repeat
            }
        }
    }

    fn parse_command(&self, input: &str) -> Option<Task2Commands> {
        match Task2Commands::from_text(input.trim()) {
            Ok(command) => Some(command),
            Err(_) => None,
        }
    }
}

#[derive(Debug)]
enum FibState {
    Start,
    ReadyToWork,
    AwaitingItemToSearch,
    AwaitingItemToInsert,
    AwaitingItemToDelete,
}

#[derive(Debug)]
enum FibCommands {
    Generate,
    Search,
    Insert,
    Delete,
    Print,
    MainMenu,
    Exit,
}

impl FibCommands {
    pub fn from_text(value: &str) -> Result<Self, InvalidEnumValueError> {
        match value {
            "generate" => Ok(FibCommands::Generate),
            "search" => Ok(FibCommands::Search),
            "insert" => Ok(FibCommands::Insert),
            "delete" => Ok(FibCommands::Delete),
            "print" => Ok(FibCommands::Print),
            "main" => Ok(FibCommands::MainMenu),
            "exit" => Ok(FibCommands::Exit),
            _ => Err(InvalidEnumValueError),
        }
    }

    pub fn to_text(&self) -> &'static str {
        match self {
            FibCommands::Generate => "'generate' - Generate array",
            FibCommands::Search => "'search' - Search element",
            FibCommands::Insert => "'insert' - Insert element",
            FibCommands::Delete => "'delete' - Delete element",
            FibCommands::Print => "'print' - Print array",
            FibCommands::MainMenu => "'main' - Return to main menu",
            FibCommands::Exit => "'exit' - Exit program",
        }
    }
}

struct FibStateMachine<'a> {
    state: FibState,
    stdin: &'a Stdin,
    vec: Vec<i32>,
}

impl<'a> FibStateMachine<'a> {
    pub fn new(stdin: &'a Stdin) -> Self {
        FibStateMachine {
            state: FibState::Start,
            stdin,
            vec: vec![],
        }
    }

    fn print(&self) {
        match self.state {
            FibState::Start => {
                println!(
                    "Available commands:\n{}",
                    [
                        FibCommands::Generate.to_text(),
                        FibCommands::MainMenu.to_text(),
                        FibCommands::Exit.to_text()
                    ]
                    .join("\n")
                )
            }
            FibState::ReadyToWork => {
                println!("Array with size {} is ready.", self.vec.len(),);
                println!(
                    "Available commands:\n{}",
                    [
                        FibCommands::Search.to_text(),
                        FibCommands::Insert.to_text(),
                        FibCommands::Delete.to_text(),
                        FibCommands::Print.to_text(),
                        FibCommands::MainMenu.to_text(),
                        FibCommands::Exit.to_text()
                    ]
                    .join("\n")
                );
            }
            _ => (),
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
            (_, Some(FibCommands::MainMenu)) => Res::MainMenu,
            (_, Some(FibCommands::Exit)) => Res::Exit,
            (FibState::ReadyToWork, Some(FibCommands::Print)) => {
                println!("{:?}", self.vec);
                Res::Repeat
            }
            (FibState::Start, Some(command)) => self.handle_start(command),
            (FibState::ReadyToWork, Some(command)) => self.handle_work(command),
            (FibState::AwaitingItemToSearch, _) => self.handle_search(input),
            (FibState::AwaitingItemToInsert, _) => self.handle_insert(input),
            (FibState::AwaitingItemToDelete, _) => self.handle_delete(input),
            _ => {
                println!("Invalid input");
                Res::Repeat
            }
        }
    }

    fn handle_delete(&mut self, input: &str) -> Res<()> {
        match self.parse_item(input) {
            Some(num) => {
                match delete(&mut self.vec, &num) {
                    Some(idx) => println!("Element {} deleted from position {}", num, idx),
                    None => println!("Element {} not found", num),
                };
                self.state = FibState::ReadyToWork;
                Res::Repeat
            }
            None => {
                println!(
                    "Invalid item for delete. Item must be a number between {} and {}",
                    i32::MIN,
                    i32::MAX
                );
                Res::Repeat
            }
        }
    }

    fn handle_insert(&mut self, input: &str) -> Res<()> {
        match self.parse_item(input) {
            Some(num) => {
                let idx = insert(&mut self.vec, num);
                self.state = FibState::ReadyToWork;
                println!("Element {} inserted in position {}", num, idx);
                Res::Repeat
            }
            None => {
                println!(
                    "Invalid item for insert. Item must be a number between {} and {}",
                    i32::MIN,
                    i32::MAX
                );
                Res::Repeat
            }
        }
    }

    fn handle_search(&mut self, input: &str) -> Res<()> {
        match self.parse_item(input) {
            Some(num) => {
                let mut now: Instant;

                now = Instant::now();
                let (res, _) = fib_search(&self.vec, &num);
                println!("Fibonacci search took {:.10}", now.elapsed().as_secs_f64());
                match res {
                    Some(idx) => println!("Fibonacci: Element {} found at index {}", num, idx),
                    None => println!("Fibonacci: Element {} not found", num),
                };

                now = Instant::now();
                let res = self.vec.binary_search(&num);
                println!("Binary search took {:.10}", now.elapsed().as_secs_f64());
                match res {
                    Ok(idx) => {
                        println!("Std binary search: Element {} found at index {}", num, idx)
                    }
                    Err(_) => println!("Std binary search: Element {} not found", num),
                };

                self.state = FibState::ReadyToWork;
                Res::Repeat
            }
            None => {
                println!(
                    "Invalid item for search. Item must be a number between {} and {}",
                    i32::MIN,
                    i32::MAX
                );
                Res::Repeat
            }
        }
    }

    fn parse_item(&self, input: &str) -> Option<i32> {
        match input.trim().parse::<i32>() {
            Ok(num) => Some(num),
            Err(_) => None,
        }
    }

    fn parse_command(&self, input: &str) -> Option<FibCommands> {
        match FibCommands::from_text(input.trim()) {
            Ok(command) => Some(command),
            Err(_) => None,
        }
    }

    fn handle_work(&mut self, command: FibCommands) -> Res<()> {
        match command {
            FibCommands::Search => {
                println!("Input number to search or 'main' to return to main menu, or 'exit' to exit program:");
                self.state = FibState::AwaitingItemToSearch;
            }
            FibCommands::Insert => {
                println!("Input number to insert or 'main' to return to main menu, or 'exit' to exit program:");
                self.state = FibState::AwaitingItemToInsert;
            }
            FibCommands::Delete => {
                println!("Input number to delete or 'main' to return to main menu, or 'exit' to exit program:");
                self.state = FibState::AwaitingItemToDelete;
            }
            _ => {
                println!("Invalid command")
            }
        }
        Res::Repeat
    }

    fn handle_start(&mut self, command: FibCommands) -> Res<()> {
        match command {
            FibCommands::Generate => self.handle_generation(),
            FibCommands::Insert | FibCommands::Search | FibCommands::Delete => {
                println!("You must generate array first");
                Res::Repeat
            }
            _ => {
                println!("Invalid command");
                Res::Repeat
            }
        }
    }

    fn handle_generation(&mut self) -> Res<()> {
        match ArrayGenerationInputStateMachine::new(self.stdin, rand::thread_rng()).start() {
            Res::Result(mut vec) => {
                vec.sort_unstable();
                self.vec = vec;
                self.state = FibState::ReadyToWork;
                Res::Repeat
            }
            Res::MainMenu => Res::MainMenu,
            Res::Back => Res::Back,
            Res::Exit => Res::Exit,
            Res::Repeat => Res::Repeat,
        }
    }
}

#[derive(Debug)]
enum HashCommands {
    Add,
    Contains,
    MainMenu,
    Exit,
}

impl HashCommands {
    pub fn from_str(value: &str) -> Result<Self, InvalidEnumValueError> {
        match value {
            "add" => Ok(HashCommands::Add),
            "search" => Ok(HashCommands::Contains),
            "main" => Ok(HashCommands::MainMenu),
            "exit" => Ok(HashCommands::Exit),
            _ => Err(InvalidEnumValueError),
        }
    }

    pub fn to_text(&self) -> &'static str {
        match self {
            HashCommands::Add => "'add' - Add an element to the set",
            HashCommands::Contains => "'search' - Search an element in the set",
            HashCommands::MainMenu => "'main' - Return to main menu",
            HashCommands::Exit => "'exit' - Exit program",
        }
    }
}

#[derive(Debug)]
enum HashState {
    Start,
    ReadyToWork,
    AwaitingItemToAdd,
    AwaitingItemToSearch,
}

struct HashStateMachine<'a> {
    state: HashState,
    stdin: &'a Stdin,
    set: CustomHasSet,
}

impl<'a> HashStateMachine<'a> {
    pub fn new(stdin: &'a Stdin) -> Self {
        HashStateMachine {
            state: HashState::Start,
            stdin,
            set: CustomHasSet::new(0),
        }
    }

    fn print(&self) {
        match self.state {
            HashState::Start => println!(
                "Enter size of the set or one of available commands:\n{}:",
                [
                    HashCommands::MainMenu.to_text(),
                    HashCommands::Exit.to_text()
                ]
                .join("\n"),
            ),
            HashState::ReadyToWork => println!(
                "Set with size {} is ready.\nAvailable commands:\n{}",
                self.set.get_size(),
                [
                    HashCommands::Add.to_text(),
                    HashCommands::Contains.to_text(),
                    HashCommands::MainMenu.to_text(),
                    HashCommands::Exit.to_text()
                ]
                .join("\n"),
            ),
            HashState::AwaitingItemToAdd => println!(
                "Enter number to add one of available commands:\n{}:",
                [
                    HashCommands::MainMenu.to_text(),
                    HashCommands::Exit.to_text()
                ]
                .join("\n")
            ),
            HashState::AwaitingItemToSearch => println!(
                "Enter number to search one of available commands:\n{}:",
                [
                    HashCommands::MainMenu.to_text(),
                    HashCommands::Exit.to_text()
                ]
                .join("\n")
            ),
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
            (_, Some(HashCommands::MainMenu)) => Res::MainMenu,
            (_, Some(HashCommands::Exit)) => Res::Exit,
            (HashState::Start, _) => self.handle_start(input),
            (HashState::ReadyToWork, Some(command)) => self.handle_work(command),
            (HashState::AwaitingItemToAdd, _) => self.handle_add(input),
            (HashState::AwaitingItemToSearch, _) => self.handle_search(input),
            _ => {
                println!("Invalid input");
                Res::Repeat
            }
        }
    }

    fn handle_start(&mut self, input: &str) -> Res<()> {
        match input.trim().parse::<usize>() {
            Ok(size) => {
                self.set = CustomHasSet::new(size);
                println!("Set with size {} created", size);
                self.state = HashState::ReadyToWork;
            }
            Err(_) => {
                println!(
                    "Invalid size. Size must be a number between {} and {}",
                    usize::MIN,
                    usize::MAX
                );
            }
        };
        Res::Repeat
    }

    fn handle_work(&mut self, command: HashCommands) -> Res<()> {
        match command {
            HashCommands::Add => {
                self.state = HashState::AwaitingItemToAdd;
            }
            HashCommands::Contains => {
                self.state = HashState::AwaitingItemToSearch;
            }
            _ => {
                println!("Invalid command");
            }
        }
        Res::Repeat
    }

    fn handle_add(&mut self, input: &str) -> Res<()> {
        match input.trim().parse::<i32>() {
            Ok(num) => {
                let res = self.set.add(num);
                match res {
                    Ok(true) => println!("Element {} added", num),
                    Ok(false) => println!("Element {} already exists", num),
                    Err(_) => println!("Set is full"),
                };
                self.state = HashState::ReadyToWork;
            }
            Err(_) => {
                println!(
                    "Invalid item for add. Item must be a number between {} and {}",
                    i32::MIN,
                    i32::MAX
                );
            }
        };
        Res::Repeat
    }

    fn handle_search(&mut self, input: &str) -> Res<()> {
        match input.trim().parse::<i32>() {
            Ok(num) => {
                let res = self.set.contains(num);
                match res {
                    true => println!("Element {} found", num),
                    false => println!("Element {} not found", num),
                };
                self.state = HashState::ReadyToWork;
            }
            Err(_) => {
                println!(
                    "Invalid item for search. Item must be a number between {} and {}",
                    i32::MIN,
                    i32::MAX
                );
            }
        };
        Res::Repeat
    }

    fn parse_command(&self, input: &str) -> Option<HashCommands> {
        match HashCommands::from_str(input.trim()) {
            Ok(command) => Some(command),
            Err(_) => None,
        }
    }
}
