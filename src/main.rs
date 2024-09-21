use iced::widget::{text, column, Column, Container, Row, row};

fn main() -> iced::Result {
    iced::run("Cool Calc", Calculator::update, Calculator::view)
}

// got a little ahead of my self and have most of the internals of what state the calc needs ready to go
#[derive(Default)]
struct Calculator {
    calc_value: u64,
    first_value: u64,
    first_value_set: bool,
    second_value: u64,
    mode: Mode,
    operation: Operation
}


// different bases I want the calculator to support
#[derive(Debug, Clone, Copy, Default)]
enum Mode {
    #[default]
    DEC, // base 10 (normal math 10 + 10 = 20)
    OCT, // base 8 (7 + 7 = 16) (actually 14 but base 8)
    HEX,// base 16 (A + A = 14) (actually 20 but base 16)
    BIN // binary just base 2 (100111 * 11011 = 10000011101) (multiplication because why not) (actually 1,053 in base 10)
}

#[derive(Debug, Clone, Copy, Default)]
enum Operation {
    #[default]
    ADD,
    SUB,
    MUL,
    DIV
}

// to be used when the supporting ui exists
#[derive(Debug, Clone, Copy)]
enum Message {
    SetValue(u64),
    Calculate(Operation),
    SetMode(Mode)
}

impl Calculator {
    // basic update that will do things eventually when there is ui to support it so it can be tested
    fn update(&mut self, message: Message) {
        match message {
            Message::SetValue(value) => {}
            Message::Calculate(operation) => {}
            Message::SetMode(mode) => {}
        }
    }
    
    // the ui returns a single 
    fn view(&self) -> Container<Message> {
        // add the reverse so it does the row in a bit more of a standard way if you look at a calculator
        Container::new((0..3).rev().fold(
            // columns the rows go into
            Column::new(),
            |a, b| {
                a.push(
                    // will be buttons eventually
                    row!(
                        text(format!(" test a  {} ", b)).size(20),
                        text(format!(" test b {} ", b)).size(20),
                        text(format!(" test c {} ", b)).size(20),
                    ).padding(20)
                )
            },
        ))
    }
}