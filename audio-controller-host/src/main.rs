use std::time::Duration;

use windows_volume_control::{AudioController, CoinitMode};

fn main() {
    loop {
        std::thread::sleep(Duration::from_millis(1000));

        let audio_controller = controller_init();
        let sessions = unsafe { audio_controller.get_all_session_names() };

        let sessions = sessions
            .into_iter()
            .map(|name| unsafe { audio_controller.get_session_by_name(name) })
            .collect::<Vec<_>>();

        for session in sessions {
            // TODO: read data from external hardware
            match session {
                Some(session) => unsafe { println!("{:?}", session.getName()) },
                None => todo!(),
            }
        }

        println!("🦀All processes listed! 🦀\n")
    }
}

fn controller_init() -> AudioController {
    let mut audiocontroller = unsafe { AudioController::init(None) };
    audiocontroller.session_scan();
    audiocontroller
}

trait MyAudioController {
    fn session_scan(&mut self);
}

impl MyAudioController for AudioController {
    fn session_scan(&mut self) {
        unsafe {
            self.GetSessions();
            self.GetDefaultAudioEnpointVolumeControl();
            self.GetAllProcessSessions();
        }
    }
}
