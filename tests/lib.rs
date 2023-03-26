use arie::{AudioConfiguration, AudioPlayer};

#[test]
fn demo_test() {
    let config = AudioConfiguration {
        sample_rate: 44100,
        channels: 2,
    };

    let mut player = AudioPlayer::new(config).unwrap();
    player.play();

    // generate sine wave for 1 second
    let mut buffer = vec![];
    for i in 0..44100 {
        let sample = (i as f32 * 440.0 * 2.0 * std::f32::consts::PI / 44100.0).sin();
        let sample = (sample * std::i16::MAX as f32) as i16;
        let sample = sample.to_le_bytes();
        buffer.extend_from_slice(&sample);
    }

    player.queue_buffer(&buffer);

    std::thread::sleep(std::time::Duration::from_secs(1));
}
