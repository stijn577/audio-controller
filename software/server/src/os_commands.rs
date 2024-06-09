use windows_volume_control::AudioController;

pub(crate) fn _controller_init() -> AudioController {
    let mut audiocontroller = unsafe { AudioController::init(None) };
    audiocontroller.session_scan();
    audiocontroller
}

pub(crate) async fn _audio_control() {
    let audio_controller = _controller_init();
    let mut sessions = unsafe { audio_controller.get_all_session_names() };

    sessions.sort();

    let cbor = serde_cbor::to_vec(&sessions).unwrap();
    let _ = tokio::fs::write("./out", cbor).await;
    let temp = &tokio::fs::read("./out").await.unwrap();
    let session_names: Vec<String> = serde_cbor::from_slice(temp).unwrap();
    println!("{:?}", session_names);

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

trait _MyAudioController {
    fn session_scan(&mut self);
}

impl _MyAudioController for AudioController {
    fn session_scan(&mut self) {
        unsafe {
            self.GetSessions();
            self.GetDefaultAudioEnpointVolumeControl();
            self.GetAllProcessSessions();
        }
    }
}
