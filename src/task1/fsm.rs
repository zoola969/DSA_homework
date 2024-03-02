use crate::common;
use crate::common::{ArrayGenerationInputStateMachine, InvalidEnumValueError, Res};
use crate::task1::quicksort::quicksort;
use crate::task1::selection_sort::selection_sort;
use std::io::Stdin;
use std::time::Instant;

#[derive(Debug)]
enum SortState {
    Start,
    AwaitingArray,
    ReadyToWork,
}

#[derive(Debug)]
enum SortCommands {
    Generate,
    Sort,
    Stat,
    MainMenu,
    Exit,
}

impl SortCommands {
    pub fn from_str(value: &str) -> Result<Self, InvalidEnumValueError> {
        match value {
            "generate" => Ok(SortCommands::Generate),
            "sort" => Ok(SortCommands::Sort),
            "stat" => Ok(SortCommands::Stat),
            "main" => Ok(SortCommands::MainMenu),
            "exit" => Ok(SortCommands::Exit),
            _ => Err(InvalidEnumValueError),
        }
    }

    pub fn to_text(&self) -> &'static str {
        match self {
            SortCommands::Generate => "'generate' - Generate array",
            SortCommands::Sort => "'sort' - Sort array",
            SortCommands::Stat => "'stat' - Print statistics",
            SortCommands::MainMenu => "'main' - Return to main menu",
            SortCommands::Exit => "'exit' - Exit program",
        }
    }
}

pub struct SortStateMachine<'a> {
    state: SortState,
    stdin: &'a Stdin,
    vec: Vec<i32>,
    iter_number: usize,
    result: Vec<[f64; 3]>,
}

impl<'a> SortStateMachine<'a> {
    pub fn new(stdin: &'a Stdin) -> Self {
        SortStateMachine {
            state: SortState::Start,
            stdin,
            vec: vec![],
            iter_number: 0,
            result: vec![],
        }
    }

    fn print(&self) {
        match self.state {
            SortState::Start => {
                println!(
                    "Available commands:\n{}",
                    [
                        SortCommands::Generate.to_text(),
                        SortCommands::MainMenu.to_text(),
                        SortCommands::Exit.to_text()
                    ]
                    .join("\n")
                );
            }
            SortState::ReadyToWork => {
                println!(
                    "Array with size {} is ready. Current iteration: {}",
                    self.vec.len(),
                    self.iter_number
                );
                println!(
                    "Available commands:\n{}",
                    [
                        SortCommands::Sort.to_text(),
                        SortCommands::Stat.to_text(),
                        SortCommands::MainMenu.to_text(),
                        SortCommands::Exit.to_text()
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
            (_, Some(SortCommands::MainMenu)) => Res::MainMenu,
            (_, Some(SortCommands::Exit)) => Res::Exit,
            (SortState::Start, Some(SortCommands::Generate)) => {
                self.state = SortState::AwaitingArray;
                self.handle_generate()
            }
            (SortState::Start, Some(SortCommands::Sort) | Some(SortCommands::Stat)) => {
                println!("You must create array first");
                Res::Repeat
            }
            (SortState::ReadyToWork, Some(SortCommands::Sort)) => {
                self.work();
                Res::Repeat
            }
            (SortState::ReadyToWork, Some(SortCommands::Stat)) => {
                self.print_stat();
                Res::Repeat
            }
            _ => {
                println!("Invalid command");
                Res::Repeat
            }
        }
    }

    fn parse_command(&self, input: &str) -> Option<SortCommands> {
        match SortCommands::from_str(input.trim()) {
            Ok(command) => Some(command),
            Err(_) => None,
        }
    }

    fn work(&mut self) {
        let mut now: Instant;
        let mut data_for_std = self.vec.clone();
        let mut data_for_sel = self.vec.clone();
        let mut data_for_quick = self.vec.clone();
        let mut times: [f64; 3] = [0.0; 3];

        now = Instant::now();
        data_for_std.sort_unstable();
        let std_duration = now.elapsed().as_secs_f64();
        times[0] = std_duration;

        now = Instant::now();
        quicksort(&mut data_for_quick);
        let quick_duration = now.elapsed().as_secs_f64();
        times[1] = quick_duration;
        assert_eq!(data_for_quick, data_for_std);

        now = Instant::now();
        selection_sort(&mut data_for_sel);
        let select_duration = now.elapsed().as_secs_f64();
        times[2] = select_duration;
        assert_eq!(data_for_sel, data_for_std);

        println!("Iteration, Std, quick, select");
        println!(
            "{}, {:.10}, {:.10}, {:.10}",
            self.iter_number, std_duration, quick_duration, select_duration
        );
        self.iter_number += 1;
        self.result.push(times);
    }

    fn handle_generate(&mut self) -> Res<()> {
        match ArrayGenerationInputStateMachine::new(self.stdin, rand::thread_rng()).start() {
            Res::Result(vec) => {
                self.vec = vec;
                self.state = SortState::ReadyToWork;
                Res::Repeat
            }
            Res::MainMenu => Res::MainMenu,
            Res::Back => Res::Back,
            Res::Exit => Res::Exit,
            Res::Repeat => Res::Repeat,
        }
    }

    fn print_stat(&self) {
        println!(
            "Average times after {} iterations: Std sort: {:.10}, Quicksort: {:.10}, Selection sort: {:.10}",
            self.iter_number,
            common::find_average(&self.result.iter().map(|x| x[0]).collect::<Vec<f64>>()),
            common::find_average(&self.result.iter().map(|x| x[1]).collect::<Vec<f64>>()),
            common::find_average(&self.result.iter().map(|x| x[2]).collect::<Vec<f64>>()),
        )
    }
}
