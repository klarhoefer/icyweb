
use iced::{self, Center, Length, Task, Rectangle, Renderer, Theme, mouse, Color};
use iced::widget::{button, column, text, vertical_space, Button, canvas};

use reqwest::Client;
use serde::Deserialize;


#[derive(Debug, Clone)]
enum Msg {
    ValueChanged(i32),
    Fetched(Result<IpAddress, String>),
}


#[derive(Debug, Clone, Deserialize)]
struct IpAddress {
    origin: String,
}

#[derive(Debug, Clone)]
struct Circle {
    x: f32,
    y: f32,
    radius: f32,
}

impl<Msg> canvas::Program<Msg> for Circle {
    type State = ();
    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        // We prepare a new `Frame`
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        // We create a `Path` representing a simple circle
        let mut point = frame.center();
        point.x += self.x;
        point.y += self.y;
        let circle = canvas::Path::circle(point, self.radius);

        // And fill it with some color
        frame.fill(&circle, Color::BLACK);

        match cursor {
            mouse::Cursor::Available(point) => {
                let circle2 = canvas::Path::circle(point, self.radius);
                frame.fill(&circle2, Color::from([0.0, 0.0, 1.0]));
            },
            mouse::Cursor::Unavailable => (),
        }
        
        // Then, we produce the geometry
        vec![frame.into_geometry()]
    }
}

#[derive(Debug, Clone)]
struct Model {
    value: i32,
    ip_address: Option<IpAddress>,
    error: Option<String>,
    cirle: Circle,
}

impl Model {
    fn new() -> Model {
        Model {
            value: 0,
            ip_address: None,
            error: None,
            cirle: Circle {
                x: 100.0,
                y: 100.0,
                radius: 50.0,
            },
        }
    }

    fn update(&mut self, msg: Msg) -> Task<Msg> {
        match msg {
            Msg::ValueChanged(value) => self.value += value,
            Msg::Fetched(Ok(s)) => self.ip_address = Some(s),
            Msg::Fetched(Err(e)) => self.error = Some(e),
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
            vertical_space().height(20),
            text(self.ip_address.as_ref().map_or("Fetching...".to_string(), |ip| ip.origin.clone())),
            vertical_space().height(20),
            canvas(&self.cirle).width(Length::Fill).height(Length::Fill),
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
        .run_with(|| (Model::new(), Task::perform(fetch(), Msg::Fetched)))
}

async fn fetch() -> Result<IpAddress, String> {
    let client = Client::new();
    client.get("https://httpbin.org/ip")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}
