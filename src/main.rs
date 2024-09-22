use iced::widget::{button, column, container, row, text, Column, Container};
use iced::Length;

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
    operation: Operation,
}

// different bases I want the calculator to support
#[derive(Debug, Clone, Copy, Default)]
enum Mode {
    #[default]
    DEC, // base 10 (normal math 10 + 10 = 20)
    OCT, // base 8 (7 + 7 = 16) (actually 14 but base 8)
    HEX, // base 16 (A + A = 14) (actually 20 but base 16)
    BIN, // binary just base 2 (100111 * 11011 = 10000011101) (multiplication because why not) (actually 1,053 in base 10)
}

#[derive(Debug, Clone, Copy, Default)]
enum Operation {
    #[default]
    ADD,
    SUB,
    MUL,
    DIV,
}

// to be used when the supporting ui exists
#[derive(Debug, Clone, Copy)]
enum Message {
    SetValue(u64),
    Calculate(Operation),
    SetMode(Mode),
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
        Container::new((0..=4).rev().fold(
            // columns the rows go into
            Column::new(),
            |a, b| {
                match b {
                    // lowest layer
                    0 => a.push(
                        container(row!(
                            container(button(" - ")).padding(10),
                            container(button(" 0 ")).padding(10),
                            container(button(" . ")).padding(10),
                        ))
                        .center_x(Length::Fill),
                    ),
                    1 => a.push(
                        container(
                            row!(
                                container(button(" 1 "),).padding(10),
                                container(button(" 2 ")).padding(10),
                                container(button(" 3 ")).padding(10),
                            )
                            .padding(20),
                        )
                        .center_x(Length::Fill),
                    ),
                    2 => a.push(
                        container(
                            row!(
                                container(button(" 4 ")).padding(10),
                                container(button(" 5 ")).padding(10),
                                container(button(" 6 ")).padding(10),
                            )
                            .padding(20),
                        )
                        .center_x(Length::Fill),
                    ),
                    3 => a.push(
                        container(
                            row!(
                                container(button(" 7 ")).padding(10),
                                container(button(" 8 ")).padding(10),
                                container(button(" 9 ")).padding(10),
                            )
                            .padding(20),
                        )
                        .center_x(Length::Fill),
                    ),
                    4 => a.push(
                        container(row!(
                            container(button(" A ")).padding(10),
                            container(button(" B ")).padding(10),
                            container(button(" C ")).padding(10),
                            container(button(" D ")).padding(10),
                            container(button(" E ")).padding(10),
                            container(button(" F ")).padding(10),
                        ))
                        .center_x(Length::Fill),
                    ),
                    _ => {
                        column![]
                    }
                }
            },
        ))
        .center_x(Length::Fill)
        .center_y(Length::Fill)
    }
}
