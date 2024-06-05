use shared_data::{Button, Message};
use tokio::join;

mod hardware_rx;
mod os_commands;

#[tokio::main]
async fn main() {
    if cfg!(target_os = "windows") {
        // TODO: receive slot messages from controller, instead of hardcoding here
        let thread1 = tokio::spawn(handle_message(Message::Button(Button::Slot0)));
        let thread2 = tokio::spawn(handle_message(Message::Button(Button::Slot1)));
        let thread3 = tokio::spawn(handle_message(Message::Button(Button::Slot2)));
        let thread4 = tokio::spawn(handle_message(Message::Button(Button::Slot3)));
        let thread5 = tokio::spawn(handle_message(Message::Button(Button::Slot4)));
        let _ = join!(thread1, thread2, thread3, thread4, thread5);

        // let x = tokio::spawn(keypress(Button::Slot3));
        // let thread6c = tokio::spawn(handle_message(Message::AudioLevels());

        // let launch_discord_fut = tokio::spawn(handle_message(Message::App(App::Slot0)));
        // let launch_spotify_fut = tokio::spawn(handle_message(Message::App(App::Slot1)));
        // let launch_firefox_fut = tokio::spawn(handle_message(Message::App(App::Slot2)));
    } else {
        todo!("Linux implementation here")
    }
}

async fn handle_message(data: Message) {
    match data {
        Message::Button(app) => match app. {
            Ok(_) => println!("{:?}", app),
            Err(err) => println!("{:?}", err),
        },
        Message::SliderReport(_) => todo!(),
    };
}
