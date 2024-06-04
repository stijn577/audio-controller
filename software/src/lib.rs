// has to be no_std so it can be used on the controller
#![no_std]

use serde::{Deserialize, Serialize};

#[macro_use]
mod macros;

create_constants!(Discord Spotify Firefox);

#[derive(Serialize, Deserialize, Debug)]
pub struct AudioLevels {
    // array of AUDIO levels, (range 0-1024)
    data: [u16; N_AUDIO_LEVELS],
}

pub enum Message {
    AppLaunch(AppLaunch),
    AudioLevels(AudioLevels),
}
