use iced::theme::{Theme};
use iced::widget::{
    button, column, container,
    row, 
};
use iced::{Element, Length, Sandbox, window::Icon};

static ICON: &[u8] = include_bytes!("../assets/icon.png");
const ICON_HEIGHT: u32 = 64;
const ICON_WIDTH: u32 = 64;
pub fn main() {
    let image = image::load_from_memory(ICON).unwrap();
    let icon = iced::window::Icon::from_rgba(image.as_bytes().to_vec(), ICON_HEIGHT, ICON_WIDTH).unwrap();
    
    let settings = iced::settings::Settings {
        window: iced::window::Settings {
            size: (300, 300),
            min_size: Some((300, 300)),
            icon: Some(icon),
            ..Default::default()
        },
        ..Default::default()
    };
    // let settings = Settings::default();
    Button::run(settings);
}

#[derive(Default)]
struct Button {
    theme: Theme,
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
}

impl Sandbox for Button {
    type Message = Message;

    fn new() -> Self {
        Button::default()
    }

    fn title(&self) -> String {
        String::from("Button")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => {}
        }
    }

    fn view(&self) -> Element<Message> {

        let button = button("Click Me")
            .padding(10)
            .on_press(Message::ButtonPressed);

        let content = column![
            row![button]
        ];

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
