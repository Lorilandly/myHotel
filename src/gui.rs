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
                self.operation.current_tab = Tab::Reserve(Default::default());
            }
            Message::ToCancel => {
                self.operation.current_tab = Tab::Cancel(Default::default());
            }
            Message::ToCheckin => {
                self.operation.current_tab = Tab::Checkin(Default::default());
            }
            Message::ToCheckout => {
                self.operation.current_tab = Tab::Checkout(Default::default());
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
                .style(if let Tab::Reserve(_) = self.operation.current_tab {
                    theme::Button::Primary
                } else {
                    theme::Button::Secondary
                }),
            button("Cancel").on_press(Message::ToCancel).style(
                if let Tab::Cancel(_) = self.operation.current_tab {
                    theme::Button::Primary
                } else {
                    theme::Button::Secondary
                }
            ),
            button("Checkin").on_press(Message::ToCheckin).style(
                if let Tab::Checkin(_) = self.operation.current_tab {
                    theme::Button::Primary
                } else {
                    theme::Button::Secondary
                }
            ),
            button("Checkout").on_press(Message::ToCheckout).style(
                if let Tab::Checkout(_) = self.operation.current_tab {
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

        container(content.explain(Color::BLACK)).into()
    }
}

#[derive(Default)]
struct IO {
    input: String,
    output: String,
}

enum Tab {
    Reserve(IO),
    Cancel(IO),
    Checkin(IO),
    Checkout(IO),
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
            current_tab: Tab::Reserve(Default::default()),
        }
    }

    fn submit(&mut self, hotel: &crate::Hotel) {
        match &mut self.current_tab {
            Tab::Reserve(IO { input, output }) => {
                let result = hotel.reserve(input);
                match result {
                    Ok(s) => *output = s,
                    Err(s) => *output = s,
                };
            }
            Tab::Cancel(IO { input, output }) => {
                let result = hotel.cancel(input);
                match result {
                    Ok(s) => *output = s,
                    Err(s) => *output = s,
                };
            }
            Tab::Checkin(IO { input, output }) => {
                let result = hotel.checkin(input);
                match result {
                    Ok(s) => *output = s,
                    Err(s) => *output = s,
                };
            }
            Tab::Checkout(IO { input, output }) => {
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
                if let Tab::Reserve(IO { input, .. }) = tab {
                    *input = new_value;
                }
            }
            OperationMessage::InputUUIDChanged(new_value) => match tab {
                Tab::Cancel(IO { input, .. }) => *input = new_value,
                Tab::Checkin(IO { input, .. }) => *input = new_value,
                Tab::Checkout(IO { input, .. }) => *input = new_value,
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
            Tab::Reserve(IO { input, output }) => Self::reserve(input, output),
            Tab::Cancel(IO { input, output })
            | Tab::Checkin(IO { input, output })
            | Tab::Checkout(IO { input, output }) => Self::cancel_checkin_checkout(input, output),
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

    fn cancel_checkin_checkout(input: &str, output: &str) -> Column<'a, OperationMessage> {
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
