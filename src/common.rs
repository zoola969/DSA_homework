use rand::Rng;
use std::io::Stdin;

pub fn find_average(slice: &[f64]) -> f64 {
    if slice.is_empty() {
        return 0.0;
    }
    return slice.iter().sum::<f64>() / (slice.len() as f64);
}

#[derive(Debug)]
pub struct InvalidEnumValueError;

pub enum Res<T> {
    // exit program
    Exit,
    // go back to previous state
    Back,
    // repeat input
    Repeat,
    // go to main menu
    MainMenu,
    // return correct result
    Result(T),
}

#[derive(Debug)]
enum ArrayManualInputState {
    AwaitingInputSize,
    AwaitingInputLeft,
    AwaitingInputRight,
}

pub struct ArrayGenerationInputStateMachine<'a> {
    state: ArrayManualInputState,
    stdin: &'a Stdin,
    rng: rand::rngs::ThreadRng,
    size: usize,
    left: i32,
    right: i32,
}

impl<'a> ArrayGenerationInputStateMachine<'a> {
    pub fn new(stdin: &'a Stdin, rng: rand::rngs::ThreadRng) -> Self {
        ArrayGenerationInputStateMachine {
            state: ArrayManualInputState::AwaitingInputSize,
            stdin,
            rng,
            size: 0,
            left: 0,
            right: 0,
        }
    }

    fn print(&self) {
        match self.state {
            ArrayManualInputState::AwaitingInputSize => println!(
                "Input array size or 'main' to return to main menu, or 'exit' to exit program:"
            ),
            ArrayManualInputState::AwaitingInputLeft => println!("Input left bound or 'back' to input size again or 'main' to return to main menu, or 'exit' to exit program:"),
            ArrayManualInputState::AwaitingInputRight => println!("Input right bound or 'back' to input left bound again or 'main' to return to main menu, or 'exit' to exit program:"),
        }
    }

    pub fn start(&mut self) -> Res<Vec<i32>> {
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

    fn handle_input(&mut self, input: &str) -> Res<Vec<i32>> {
        match input.trim() {
            "main" => return Res::MainMenu,
            "exit" => return Res::Exit,
            "back" => match self.state {
                ArrayManualInputState::AwaitingInputSize => return Res::Back,
                ArrayManualInputState::AwaitingInputLeft => {
                    self.state = ArrayManualInputState::AwaitingInputSize;
                    return Res::Repeat;
                }
                ArrayManualInputState::AwaitingInputRight => {
                    self.state = ArrayManualInputState::AwaitingInputLeft;
                    return Res::Repeat;
                }
            },
            _ => (),
        }
        match self.state {
            ArrayManualInputState::AwaitingInputSize => match input.trim().parse::<usize>() {
                Ok(size) => {
                    self.size = size;
                    self.state = ArrayManualInputState::AwaitingInputLeft;
                    Res::Repeat
                }
                Err(_) => {
                    println!(
                        "Cannot parse input. Size must be a number between {} and {}",
                        usize::MIN,
                        usize::MAX,
                    );
                    Res::Repeat
                }
            },
            ArrayManualInputState::AwaitingInputLeft => match input.trim().parse::<i32>() {
                Ok(left) => {
                    self.left = left;
                    self.state = ArrayManualInputState::AwaitingInputRight;
                    Res::Repeat
                }
                Err(_) => {
                    println!(
                        "Cannot parse input. Left must be a number between {} and {}",
                        i32::MIN,
                        i32::MAX
                    );
                    Res::Repeat
                }
            },
            ArrayManualInputState::AwaitingInputRight => match input.trim().parse::<i32>() {
                Ok(right) => {
                    if right < self.left {
                        println!("Right must be greater than left");
                        return Res::Repeat;
                    };
                    self.right = right;
                    Res::Result(self.gen())
                }
                Err(_) => {
                    println!(
                        "Cannot parse input. Right must be a number between {} and {}",
                        i32::MIN,
                        i32::MAX
                    );
                    Res::Repeat
                }
            },
        }
    }
    fn gen(&mut self) -> Vec<i32> {
        (0..self.size)
            .map(|_| self.rng.gen_range(self.left..self.right))
            .collect()
    }
}
