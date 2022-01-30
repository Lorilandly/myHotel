pub use iced::{
    button,
    text_input::{self, TextInput},
    Button, Checkbox, Column, Command, Container, Element, Font, Length, Row, Sandbox, Settings,
    Text,
};

pub(crate) struct HotelUI {
    hotel: crate::Hotel,
    operation: Operation,
}

#[derive(Debug, Clone)]
pub(crate) enum Operation {
    Reserve(State),
    Cancel(State),
    Checkin(State),
    Checkout(State),
}

#[derive(Debug, Clone)]
pub struct State {
    input: text_input::State,
    input_value: String,
    result_value: Result<String, String>,
    submit: button::State,
    reservation: button::State,
    cancel: button::State,
    checkin: button::State,
    checkout: button::State,
}

pub fn ini() -> State {
    State {
        input: text_input::State::new(),
        input_value: String::new(),
        result_value: Err(String::new()),
        submit: button::State::new(),
        reservation: button::State::new(),
        cancel: button::State::new(),
        checkin: button::State::new(),
        checkout: button::State::new(),
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    ToReservation,
    ToCancel,
    ToCheckin,
    ToCheckout,
    Submit,
}

impl Sandbox for HotelUI {
    type Message = Message;

    fn new() -> Self {
        Self {
            hotel: crate::Hotel::new().expect("Error during initialization!"),
            operation: Operation::Reserve(ini()),
        }
    }

    fn title(&self) -> String {
        String::from("Hotel Reservation UI")
    }

    fn update(&mut self, event: Message) {
        match &mut self.operation {
            Operation::Reserve(state) => match event {
                Message::Submit => {
                    let result = self.hotel.reserve(state.input_value.clone());
                    match result {
                        Ok(s) => state.result_value = Ok(s),
                        Err(s) => state.result_value = Err(s),
                    };
                }
                Message::InputChanged(value) => {
                    state.input_value = value;
                }
                Message::ToReservation => {
                    state.input_value = String::new();
                    state.result_value = Err(String::new());
                }
                Message::ToCancel => {
                    state.input_value = String::new();
                    state.result_value = Ok(String::new());
                    self.operation = Operation::Cancel(state.clone());
                }
                Message::ToCheckin => {
                    state.input_value = String::new();
                    state.result_value = Ok(String::new());
                    self.operation = Operation::Checkin(state.clone());
                }
                Message::ToCheckout => {
                    state.input_value = String::new();
                    state.result_value = Ok(String::new());
                    self.operation = Operation::Checkout(state.clone());
                }
            },
            Operation::Cancel(state) => match event {
                Message::Submit => {
                    let result = self.hotel.cancel(state.input_value.clone());
                    match result {
                        Ok(s) => state.result_value = Ok(s),
                        Err(s) => state.result_value = Err(s),
                    };
                }
                Message::InputChanged(value) => {
                    state.input_value = value;
                }
                Message::ToReservation => {
                    state.input_value = String::new();
                    state.result_value = Err(String::new());
                    self.operation = Operation::Reserve(state.clone());
                }
                Message::ToCancel => {
                    state.input_value = String::new();
                    state.result_value = Ok(String::new());
                }
                Message::ToCheckin => {
                    state.input_value = String::new();
                    state.result_value = Ok(String::new());
                    self.operation = Operation::Checkin(state.clone());
                }
                Message::ToCheckout => {
                    state.input_value = String::new();
                    state.result_value = Ok(String::new());
                    self.operation = Operation::Checkout(state.clone());
                }
            },
            Operation::Checkin(state) => match event {
                Message::Submit => {
                    let result = self.hotel.checkin(state.input_value.clone());
                    match result {
                        Ok(s) => state.result_value = Ok(s),
                        Err(s) => state.result_value = Err(s),
                    };
                }
                Message::InputChanged(value) => {
                    state.input_value = value;
                }
                Message::ToReservation => {
                    state.input_value = String::new();
                    state.result_value = Err(String::new());
                    self.operation = Operation::Reserve(state.clone());
                }
                Message::ToCancel => {
                    state.input_value = String::new();
                    state.result_value = Ok(String::new());
                    self.operation = Operation::Cancel(state.clone());
                }
                Message::ToCheckin => {
                    state.input_value = String::new();
                    state.result_value = Ok(String::new());
                }
                Message::ToCheckout => {
                    state.input_value = String::new();
                    state.result_value = Ok(String::new());
                    self.operation = Operation::Checkout(state.clone());
                }
            },
            Operation::Checkout(state) => match event {
                Message::Submit => {
                    let result = self
                        .hotel
                        .checkout(state.input_value.clone(), String::from("0"));
                    match result {
                        Ok(s) => state.result_value = Ok(s),
                        Err(s) => state.result_value = Err(s),
                    };
                }
                Message::InputChanged(value) => {
                    state.input_value = value;
                }
                Message::ToReservation => {
                    state.input_value = String::new();
                    state.result_value = Err(String::new());
                    self.operation = Operation::Reserve(state.clone());
                }
                Message::ToCancel => {
                    state.input_value = String::new();
                    state.result_value = Ok(String::new());
                    self.operation = Operation::Cancel(state.clone());
                }
                Message::ToCheckin => {
                    state.input_value = String::new();
                    state.result_value = Ok(String::new());
                    self.operation = Operation::Checkin(state.clone());
                }
                Message::ToCheckout => {
                    state.input_value = String::new();
                    state.result_value = Ok(String::new());
                }
            },
        }
    }

    fn view(&mut self) -> Element<Message> {
        match &mut self.operation {
            Operation::Reserve(State {
                input,
                input_value,
                result_value,
                submit,
                reservation,
                cancel,
                checkin,
                checkout,
            }) => {
                let elements = Column::new();

                let switch = Row::new()
                    .push(button(reservation, "Reservation").on_press(Message::ToReservation))
                    .push(button(cancel, "Cancel").on_press(Message::ToCancel))
                    .push(button(checkin, "Checkin").on_press(Message::ToCheckin))
                    .push(button(checkout, "Checkout").on_press(Message::ToCheckout));

                let title = Text::new("Reserve a room")
                    .width(Length::Fill)
                    .size(100)
                    .color([0.5, 0.5, 0.5])
                    .horizontal_alignment(iced::HorizontalAlignment::Center);

                let button = button(submit, "submit").on_press(Message::Submit);
                let input = TextInput::new(input, "yyyy-mm-dd", input_value, Message::InputChanged)
                    .on_submit(Message::Submit);
                let result = match result_value {
                    Ok(s) => Text::new(format!("Your reservation id is: {}", s.to_string(),)),
                    Err(s) => Text::new(s.to_string()).color([1.0, 0.0, 0.0]),
                };

                elements
                    .max_width(800)
                    .spacing(20)
                    .push(switch)
                    .push(title)
                    .push(input)
                    .push(result)
                    .push(button)
                    .into()
            }
            Operation::Cancel(State {
                input,
                input_value,
                result_value,
                submit,
                reservation,
                cancel,
                checkin,
                checkout,
            }) => {
                let elements = Column::new();

                let switch = Row::new()
                    .push(button(reservation, "Reservation").on_press(Message::ToReservation))
                    .push(button(cancel, "Cancel").on_press(Message::ToCancel))
                    .push(button(checkin, "Checkin").on_press(Message::ToCheckin))
                    .push(button(checkout, "Checkout").on_press(Message::ToCheckout));

                let title = Text::new("Cancel a reservation")
                    .width(Length::Fill)
                    .size(100)
                    .color([0.5, 0.5, 0.5])
                    .horizontal_alignment(iced::HorizontalAlignment::Center);

                let button = button(submit, "submit").on_press(Message::Submit);
                let input = TextInput::new(
                    input,
                    "xxxxxxxx-xxxx-Mxxx-Nxxx-xxxxxxxxxxxx",
                    input_value,
                    Message::InputChanged,
                )
                .on_submit(Message::Submit);
                let result = match result_value {
                    Ok(s) => Text::new(format!("{}", s.to_string(),)),
                    Err(s) => Text::new(s.to_string()).color([1.0, 0.0, 0.0]),
                };

                elements
                    .max_width(800)
                    .spacing(20)
                    .push(switch)
                    .push(title)
                    .push(input)
                    .push(result)
                    .push(button)
                    .into()
            }
            Operation::Checkin(State {
                input,
                input_value,
                result_value,
                submit,
                reservation,
                cancel,
                checkin,
                checkout,
            }) => {
                let elements = Column::new();

                let switch = Row::new()
                    .push(button(reservation, "Reservation").on_press(Message::ToReservation))
                    .push(button(cancel, "Cancel").on_press(Message::ToCancel))
                    .push(button(checkin, "Checkin").on_press(Message::ToCheckin))
                    .push(button(checkout, "Checkout").on_press(Message::ToCheckout));

                let title = Text::new("Checkin to a room")
                    .width(Length::Fill)
                    .size(100)
                    .color([0.5, 0.5, 0.5])
                    .horizontal_alignment(iced::HorizontalAlignment::Center);

                let button = button(submit, "submit").on_press(Message::Submit);
                let input = TextInput::new(
                    input,
                    "xxxxxxxx-xxxx-Mxxx-Nxxx-xxxxxxxxxxxx",
                    input_value,
                    Message::InputChanged,
                )
                .on_submit(Message::Submit);
                let result = match result_value {
                    Ok(s) => Text::new(format!("{}", s.to_string(),)),
                    Err(s) => Text::new(s.to_string()).color([1.0, 0.0, 0.0]),
                };

                elements
                    .max_width(800)
                    .spacing(20)
                    .push(switch)
                    .push(title)
                    .push(input)
                    .push(result)
                    .push(button)
                    .into()
            }
            Operation::Checkout(State {
                input,
                input_value,
                result_value,
                submit,
                reservation,
                cancel,
                checkin,
                checkout,
            }) => {
                let elements = Column::new();

                let switch = Row::new()
                    .push(button(reservation, "Reservation").on_press(Message::ToReservation))
                    .push(button(cancel, "Cancel").on_press(Message::ToCancel))
                    .push(button(checkin, "Checkin").on_press(Message::ToCheckin))
                    .push(button(checkout, "Checkout").on_press(Message::ToCheckout));

                let title = Text::new("Checkout from Hotel")
                    .width(Length::Fill)
                    .size(100)
                    .color([0.5, 0.5, 0.5])
                    .horizontal_alignment(iced::HorizontalAlignment::Center);

                let button = button(submit, "submit").on_press(Message::Submit);
                let input = TextInput::new(
                    input,
                    "xxxxxxxx-xxxx-Mxxx-Nxxx-xxxxxxxxxxxx",
                    input_value,
                    Message::InputChanged,
                )
                .on_submit(Message::Submit);
                let result = match result_value {
                    Ok(s) => Text::new(format!("{}", s.to_string(),)),
                    Err(s) => Text::new(s.to_string()).color([1.0, 0.0, 0.0]),
                };

                elements
                    .max_width(800)
                    .spacing(20)
                    .push(switch)
                    .push(title)
                    .push(input)
                    .push(result)
                    .push(button)
                    .into()
            }
        }
    }
}
fn button<'a, Message: Clone>(state: &'a mut button::State, label: &str) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label).horizontal_alignment(iced::HorizontalAlignment::Center),
    )
    .padding(12)
    .min_width(100)
}
