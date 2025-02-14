
use iced::{self, Center, Length, Task};
use iced::widget::{button, column, text, Button};


#[derive(Debug, Clone)]
enum Msg {
    ValueChanged(i32),
}


#[derive(Debug, Clone)]
struct Model {
    value: i32,
}

impl Model {
    fn new() -> Model {
        Model { value: 0 }
    }

    fn update(&mut self, msg: Msg) -> Task<Msg> {
        match msg {
            Msg::ValueChanged(value) => self.value += value,
        }
        Task::none()
    }

    fn view(&self) -> iced::Element<Msg> {
        column![
            my_button("Increment").on_press(Msg::ValueChanged(1)).width(80),
            text(self.value),
            my_button("Decrement").on_press_maybe(
                if self.value > 0 {
                    Some(Msg::ValueChanged(-1))
                } else {
                    None
                }
            ).width(80),
        ]
        .align_x(Center)
        .width(Length::Fill)
        .into()
    }
}


fn my_button(txt: &str) -> Button<Msg> {
    button(text(txt).size(10)
        .width(Length::Fill)
        .align_x(Center)
    )
}


fn main() -> iced::Result {
    iced::application("Icy Web", Model::update, Model::view)
        .run_with(|| (Model::new(), Task::none()))
}
