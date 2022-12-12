pub use iced::widget::Column;
pub use iced::widget::{
    button, checkbox, column, container, row, text, text_input, vertical_space,
};
pub use iced::{theme, Alignment, Color, Element, Length, Sandbox, Settings};

pub(crate) struct HotelUI {
    hotel: crate::Hotel,
    operation: ReserveOperations,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToReservation,
    ToCancel,
    ToCheckin,
    ToCheckout,
    Submit,
    OperationMessage(OperationMessage),
}

impl Sandbox for HotelUI {
    type Message = Message;

    fn new() -> Self {
        Self {
            hotel: crate::Hotel::new().expect("Error during initialization!"),
            operation: ReserveOperations::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Hotel Reservation UI")
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::ToReservation => {
                self.operation.current_tab = Tab::Reserve {
                    input: String::new(),
                    output: String::new(),
                };
            }
            Message::ToCancel => {
                self.operation.current_tab = Tab::Cancel {
                    input: String::new(),
                    output: String::new(),
                };
            }
            Message::ToCheckin => {
                self.operation.current_tab = Tab::Checkin {
                    input: String::new(),
                    output: String::new(),
                };
            }
            Message::ToCheckout => {
                self.operation.current_tab = Tab::Checkout {
                    input: String::new(),
                    output: String::new(),
                };
            }
            Message::Submit => {
                self.operation.submit(&self.hotel);
            }
            Message::OperationMessage(msg) => {
                self.operation.update(msg);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let menu_bar = row![
            button("Reservation")
                .on_press(Message::ToReservation)
                .style(if let Tab::Reserve { .. } = self.operation.current_tab {
                    theme::Button::Primary
                } else {
                    theme::Button::Secondary
                }),
            button("Cancel").on_press(Message::ToCancel).style(
                if let Tab::Cancel { .. } = self.operation.current_tab {
                    theme::Button::Primary
                } else {
                    theme::Button::Secondary
                }
            ),
            button("Checkin").on_press(Message::ToCheckin).style(
                if let Tab::Checkin { .. } = self.operation.current_tab {
                    theme::Button::Primary
                } else {
                    theme::Button::Secondary
                }
            ),
            button("Checkout").on_press(Message::ToCheckout).style(
                if let Tab::Checkout { .. } = self.operation.current_tab {
                    theme::Button::Primary
                } else {
                    theme::Button::Secondary
                }
            ),
        ];

        let content: Element<_> = column![
            menu_bar,
            self.operation.view().map(Message::OperationMessage),
            button("Submit").on_press(Message::Submit),
        ]
        .max_width(540)
        .spacing(20)
        .padding(20)
        .into();

        container(content).into()
    }
}

enum Tab {
    Reserve { input: String, output: String },
    Cancel { input: String, output: String },
    Checkin { input: String, output: String },
    Checkout { input: String, output: String },
}

#[derive(Debug, Clone)]
pub enum OperationMessage {
    InputDateChanged(String),
    InputUUIDChanged(String),
}

struct ReserveOperations {
    current_tab: Tab,
}

impl<'a> ReserveOperations {
    fn new() -> Self {
        Self {
            current_tab: Tab::Reserve {
                input: String::new(),
                output: String::new(),
            },
        }
    }

    fn submit(&mut self, hotel: &crate::Hotel) {
        match &mut self.current_tab {
            Tab::Reserve { input, output } => {
                let result = hotel.reserve(input);
                match result {
                    Ok(s) => *output = s,
                    Err(s) => *output = s,
                };
            }
            Tab::Cancel { input, output } => {
                let result = hotel.cancel(input);
                match result {
                    Ok(s) => *output = s,
                    Err(s) => *output = s,
                };
            }
            Tab::Checkin { input, output } => {
                let result = hotel.checkin(input);
                match result {
                    Ok(s) => *output = s,
                    Err(s) => *output = s,
                };
            }
            Tab::Checkout { input, output } => {
                let result = hotel.checkout(input, "0");
                match result {
                    Ok(s) => *output = s,
                    Err(s) => *output = s,
                };
            }
        }
    }

    fn update(&mut self, msg: OperationMessage) {
        let tab = &mut self.current_tab;
        match msg {
            OperationMessage::InputDateChanged(new_value) => {
                if let Tab::Reserve { input, .. } = tab {
                    *input = new_value;
                }
            }
            OperationMessage::InputUUIDChanged(new_value) => match tab {
                Tab::Cancel { input, .. } => *input = new_value,
                Tab::Checkin { input, .. } => *input = new_value,
                Tab::Checkout { input, .. } => *input = new_value,
                _ => (),
            },
        }
    }
    /*
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
            },
            Operation::Cancel(state) => match event {
                Message::Submit => {
                    let result = self.hotel.cancel(state.input_value.clone());
                    match result {
                        Ok(s) => state.result_value = Ok(s),
                        Err(s) => state.result_value = Err(s),
                    };
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
            },
        }
    }
    */

    fn view(&self) -> Element<OperationMessage> {
        match &self.current_tab {
            Tab::Reserve { input, output } => Self::reserve(input, output),
            Tab::Cancel { input, output } => Self::cancel(input, output),
            Tab::Checkin { input, output } => Self::checkin(input, output),
            Tab::Checkout { input, output } => Self::checkout(input, output),
        }
        .into()
    }

    fn reserve(input: &str, output: &str) -> Column<'a, OperationMessage> {
        column![
            text_input("yyyy-mm-dd", input, OperationMessage::InputDateChanged),
            text(output),
        ]
        .into()
    }

    fn cancel(input: &str, output: &str) -> Column<'a, OperationMessage> {
        column![
            text_input(
                "xxxxxxxx-xxxx-Mxxx-Nxxx-xxxxxxxxxxxx",
                input,
                OperationMessage::InputUUIDChanged
            ),
            text(output),
        ]
        .into()
    }

    fn checkin(input: &str, output: &str) -> Column<'a, OperationMessage> {
        column![
            text_input(
                "xxxxxxxx-xxxx-Mxxx-Nxxx-xxxxxxxxxxxx",
                input,
                OperationMessage::InputUUIDChanged
            ),
            text(output),
        ]
        .into()
    }

    fn checkout(input: &str, output: &str) -> Column<'a, OperationMessage> {
        column![
            text_input(
                "xxxxxxxx-xxxx-Mxxx-Nxxx-xxxxxxxxxxxx",
                input,
                OperationMessage::InputUUIDChanged
            ),
            text(output),
        ]
        .into()
    }
}
