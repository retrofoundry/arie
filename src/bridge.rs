use crate::{AudioConfiguration, AudioPlayer};

#[no_mangle]
pub extern "C" fn AudioPlayerCreate(sample_rate: u32, channels: u16) -> Box<AudioPlayer> {
    let config = AudioConfiguration {
        sample_rate,
        channels,
    };

    match AudioPlayer::new(config) {
        Ok(player) => Box::new(player),
        Err(err) => {
            eprintln!("[Audio] failed to create audio player: {err}");
            unsafe { Box::from_raw(std::ptr::null_mut()) }
        }
    }
}

#[no_mangle]
pub extern "C" fn AudioPlayerFree(player: Option<Box<AudioPlayer>>) {
    if let Some(player) = player {
        drop(player);
    }
}

#[no_mangle]
pub extern "C" fn AudioPlayerGetBufferredSampleCount(player: Option<&mut AudioPlayer>) -> usize {
    match player {
        Some(player) => player.bufferred_sample_count(),
        None => {
            eprintln!(
                "[Audio] failed to get buffered audio: was given an invalid instance pointer"
            );
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn AudioPlayerGetBufferSize(player: Option<&mut AudioPlayer>) -> usize {
    match player {
        Some(player) => player.buffer_capacity(),
        None => {
            eprintln!("[Audio] failed to get desired buffered audio: was given an invalid instance pointer");
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn AudioPlayerPlay(player: Option<&mut AudioPlayer>) {
    match player {
        Some(player) => player.play(),
        None => {
            eprintln!("[Audio] failed to play audio: was given an invalid instance pointer")
        }
    }
}

#[no_mangle]
pub extern "C" fn AudioPlayerPause(player: Option<&mut AudioPlayer>) {
    match player {
        Some(player) => player.pause(),
        None => {
            eprintln!("[Audio] failed to pause audio: was given an invalid instance pointer")
        }
    }
}

#[no_mangle]
pub extern "C" fn AudioPlayerQueueBuffer(
    player: Option<&mut AudioPlayer>,
    buf: *const u8,
    len: usize,
) {
    match player {
        Some(player) => {
            let buffer = unsafe { std::slice::from_raw_parts(buf, len) };
            player.queue_buffer_u8(buffer);
        }
        None => {
            eprintln!("[Audio] failed to play audio buffer: was given an invalid instance pointer")
        }
    }
}
