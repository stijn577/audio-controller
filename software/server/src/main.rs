use shared_data::{AudioLevels, ButtonError, Message};
use tokio::join;

mod hardware_rx;
mod os_commands;

#[tokio::main]
async fn main() {
    if cfg!(target_os = "windows") {
        // TODO: receive slot messages from controller, instead of hardcoding here
        let message0 = Message::Slot0;
        let message1 = Message::Slot1;
        let message2 = Message::Slot2;
        let message3 = Message::Slot3;
        let message4 = Message::Slot4;
        let message5 = Message::Slot5;
        let thread0 = tokio::spawn(process(message0));
        let thread1 = tokio::spawn(process(message1));
        let thread2 = tokio::spawn(process(message2));
        let thread3 = tokio::spawn(process(message3));
        let thread4 = tokio::spawn(process(message4));
        let thread5 = tokio::spawn(process(message5));
        let _ = join!(thread0, thread1, thread2, thread3, thread4, thread5);

        // let x = tokio::spawn(keypress(Button::Slot3));
        // let thread6c = tokio::spawn(handle_message(Message::AudioLevels());

        // let launch_discord_fut = tokio::spawn(handle_message(Message::App(App::Slot0)));
        // let launch_spotify_fut = tokio::spawn(handle_message(Message::App(App::Slot1)));
        // let launch_firefox_fut = tokio::spawn(handle_message(Message::App(App::Slot2)));
    } else {
        todo!("Linux implementation here")
    }
}

async fn process(message: Message) {
    match message.process_message().await {
        Ok(_) => {}
        Err(e) => println!("{:?}", e),
    };
}

// trait ProcessMessage {
//     async fn process_message(&self) -> Result<(), MessageParseError>;
//     async fn process_slot(&self) -> Result<(), MessageParseError>;

//     async fn command(&self, args: &[&str]) -> Result<(), MessageParseError>;

//     fn as_usize(&self) -> usize;
// }
