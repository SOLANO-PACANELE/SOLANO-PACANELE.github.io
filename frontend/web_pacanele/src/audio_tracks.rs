use crate::audio::AudioEvent;
use dioxus_logger::tracing::info;

#[derive(Debug, Clone, Copy)]
pub struct SoundSequenceInfo {
    pub ding_id: u32,
    pub last_event: AudioEvent,
    pub time_since_event: f64,
}

/// Set FmOsc to these values. FLoats are [0,1], note is midi int.
#[derive(Debug, Clone, Copy, Default)]
pub struct SoundTrackOutput {
    pub note: u8,
    pub gain: f32,
    pub fm_freq: f32,
    pub fm_amount: f32,
}

pub const TRACKS: [fn(SoundSequenceInfo, SoundTrackOutput) -> SoundTrackOutput; 3] =
    [sound_track_1, sound_track_2, sound_track_3];

fn random_arp_note(base: u8, prev_note: u8, arp: &[u8]) -> u8 {
    use rand::{thread_rng, Rng};
    if arp.is_empty() {
        info!("empty arp");
        return base;
    }
    if arp.len() == 1 {
        return base + arp[0];
    }
    loop {
        let note_id = thread_rng().gen::<usize>() % arp.len();
        let note = base + arp[note_id];
        if note == prev_note {
            continue;
        }
        return note;
    }
}

fn sound_track_1(info: SoundSequenceInfo, prev: SoundTrackOutput) -> SoundTrackOutput {
    let arp = match info.last_event {
        AudioEvent::StartSpin => vec![0, 4, 7],
        AudioEvent::HaveResults => vec![0, 4, 7, 11],
        AudioEvent::WheelStop { .. } => vec![4, 7, 11],
        _ => vec![0],
    };

    let gain = match info.last_event {
        AudioEvent::StartSpin => 0.2 + 0.2 * (info.time_since_event.clamp(0.0, 2.0) / 2.0),
        AudioEvent::HaveResults => 0.4 + 0.2 * (info.time_since_event.clamp(0.0, 2.0) / 2.0),
        AudioEvent::WheelStop { .. } => 0.6 + 0.1 * (info.time_since_event.clamp(0.0, 2.0) / 2.0),
        _ => 0.0,
    } as f32;

    let note = random_arp_note(60, prev.note, &arp);

    info!("ding {note}");

    SoundTrackOutput {
        note,
        gain,
        fm_amount: 0.0,
        fm_freq: 0.0,
    }
}

fn sound_track_2(info: SoundSequenceInfo, _prev: SoundTrackOutput) -> SoundTrackOutput {
    let gain = match info.last_event {
        AudioEvent::WheelStop { .. } => {
            if info.time_since_event < 0.2 {
                1.0
            } else if info.time_since_event < 0.4 {
                (1.0 - (info.time_since_event - 0.2) * 2.0).clamp(0.0, 1.0)
            } else {
                0.0
            }
        }
        _ => 0.0,
    } as f32;

    let note = match info.last_event {
        AudioEvent::WheelStop { wheel_id } => 72 + wheel_id as u8,
        _ => 0,
    };

    SoundTrackOutput {
        note,
        gain,
        fm_amount: 1.0,
        fm_freq: 1.0,
    }
}

fn sound_track_3(info: SoundSequenceInfo, _prev: SoundTrackOutput) -> SoundTrackOutput {
    let mut r = sound_track_2(info, _prev);
    if r.note > 36 {
        r.note -= 24;
    };
    r
}
