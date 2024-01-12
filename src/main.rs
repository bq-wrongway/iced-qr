use std::path::PathBuf;
use std::time::Duration;

use iced::widget::image::Handle;
use iced::widget::{button, container, horizontal_space, row, svg, text, Image};
use iced::{executor, Alignment, Application, Command, Element, Length, Settings, Size, Theme};
use qrcode_generator::QrCodeEcc;

pub fn main() -> iced::Result {
    Counter::run(Settings {
        window: iced::window::Settings {
            max_size: Some(Size {
                width: 500.0,
                height: 350.0,
            }),
            ..Default::default()
        },
        ..Default::default()
    })
}

struct Counter {
    value: i32,
    path: PathBuf,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Application for Counter {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Counter, Command<Message>) {
        let mut paths = PathBuf::new();
        paths.push("/usr/share/backgrounds/pop/nasa-45068.jpg");
        (
            Counter {
                value: 0,
                path: paths,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                // qrcode_generator::to_svg_to_file(
                //     "hi there you goose!",
                //     QrCodeEcc::Low,
                //     1024,
                //     None::<&str>,
                //     "resources/test.png",
                // )
                // .unwrap();
                // std::thread::sleep(Duration::from_secs(2));
                let mut pat = PathBuf::new();
                pat.push(
                    "/home/melnibone/sandbox/rust/iced_sandbox/iced-qr/resources/file_output.png",
                );

                self.path = pat;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let handle = svg::Handle::from_path(format!(
            "{}/resources/file_output.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        // let handl = Handle::from_path(format!(
        //     "{}/resources/file_output.png",
        //     env!("CARGO_MANIFEST_DIR")
        // ));

        let svasg = svg(handle);
        row![
            horizontal_space(Length::Fill),
            button("Increment").on_press(Message::IncrementPressed),
            text(self.value).size(50),
            container(svasg.content_fit(iced::ContentFit::Fill))
                .width(50)
                .height(50),
            container(
                Image::new(Handle::from_path(&self.path)).content_fit(iced::ContentFit::Fill)
            )
            .width(150)
            .height(150),
            button("Decrement").on_press(Message::DecrementPressed),
            horizontal_space(Length::Fill),
        ]
        .padding(20)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_items(Alignment::Center)
        .into()
    }
}
