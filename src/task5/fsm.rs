use crate::common::{InvalidEnumValueError, Res};
use crate::task5::fractal::draw_koch_curve;
use std::io::Stdin;
use std::time::Instant;

#[derive(Debug)]
enum FractalDrawerState {
    Start,
    AwaitingDepth,
}

#[derive(Debug)]
enum DrawerCommands {
    EnterDepth,
    MainMenu,
    Exit,
}

impl DrawerCommands {
    pub fn from_text(value: &str) -> Result<Self, InvalidEnumValueError> {
        match value {
            "depth" => Ok(DrawerCommands::EnterDepth),
            "main" => Ok(DrawerCommands::MainMenu),
            "exit" => Ok(DrawerCommands::Exit),
            _ => Err(InvalidEnumValueError),
        }
    }

    pub fn to_text(&self) -> &'static str {
        match self {
            DrawerCommands::EnterDepth => "'depth' - Enter fractal depth",
            DrawerCommands::MainMenu => "'main' - Return to main menu",
            DrawerCommands::Exit => "'exit' - Exit program",
        }
    }
}

pub struct KochCurveDrawerStateMachine<'a> {
    state: FractalDrawerState,
    stdin: &'a Stdin,
}

impl<'a> KochCurveDrawerStateMachine<'a> {
    pub fn new(stdin: &'a Stdin) -> Self {
        KochCurveDrawerStateMachine {
            state: FractalDrawerState::Start,
            stdin,
        }
    }

    fn print(&self) {
        match self.state {
            FractalDrawerState::Start => {
                println!(
                    "Available commands:\n{}",
                    [
                        DrawerCommands::EnterDepth.to_text(),
                        DrawerCommands::MainMenu.to_text(),
                        DrawerCommands::Exit.to_text(),
                    ]
                    .join("\n")
                );
            }
            FractalDrawerState::AwaitingDepth => {
                println!(
                    "Enter fractal depth for drawing or\n{}",
                    [
                        DrawerCommands::MainMenu.to_text(),
                        DrawerCommands::Exit.to_text(),
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
            (_, Some(DrawerCommands::MainMenu)) => Res::MainMenu,
            (_, Some(DrawerCommands::Exit)) => Res::Exit,
            (FractalDrawerState::Start, Some(DrawerCommands::EnterDepth)) => {
                self.state = FractalDrawerState::AwaitingDepth;
                Res::Repeat
            }
            (FractalDrawerState::AwaitingDepth, _) => self.draw(input),
            _ => {
                println!("Invalid command");
                Res::Repeat
            }
        }
    }

    fn draw(&mut self, input: &str) -> Res<()> {
        match self.parse_depth(input) {
            Some(depth) => {
                let start = Instant::now();
                draw_koch_curve(depth);
                let duration = start.elapsed();
                println!("Time elapsed in drawing fractal: {:?}", duration);
                println!("You can find the result in koch.svg");
                Res::Repeat
            }
            None => Res::Repeat,
        }
    }

    fn parse_depth(&self, input: &str) -> Option<usize> {
        match input.trim().parse::<usize>() {
            Ok(depth) => Some(depth),
            Err(_) => {
                println!(
                    "Invalid input. Depth must be a number between {} and {}",
                    usize::MIN,
                    usize::MAX
                );
                None
            }
        }
    }

    fn parse_command(&self, input: &str) -> Option<DrawerCommands> {
        match DrawerCommands::from_text(input.trim()) {
            Ok(command) => Some(command),
            Err(_) => None,
        }
    }
}
