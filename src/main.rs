use crate::fsm::MainStateMachine;

mod common;
mod fsm;
mod task1;
mod task2;
mod task3;
mod task4;
mod task5;

fn main() {
    // task3::main();
    let stdin = std::io::stdin();
    MainStateMachine::new(&stdin).start();
}
