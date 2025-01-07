use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use web_sys::{wasm_bindgen::JsValue, AudioContext, OscillatorType};

use crate::{
    audio_tracks::{SoundSequenceInfo, SoundTrackOutput, TRACKS},
    time::{get_current_ts, sleep},
};

#[derive(Debug, Clone, Copy)]
pub enum AudioEvent {
    StartSpin,
    HaveResults,
    WheelStop { wheel_id: u32, pcnl_count: u32 },
    WheelsFinished,
    StopAudio,
}


#[derive(Debug, Clone, Copy)]
pub struct AudioSettings {
    volume: f64,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self { volume: 0.25 }
    }
}

pub fn make_audio_loop_coroutine() {
    let mut oscillators: Signal<Option<OscillatorList>> = use_signal(|| None);
    let audio_settings = use_signal(|| AudioSettings::default());
    use_context_provider(move || audio_settings);

    let co = use_coroutine(move |mut _rx| async move {
        let mut last_event = None;
        let mut last_event_time = get_current_ts();
        let mut ding_id = 0;

        let mut last_outputs = vec![SoundTrackOutput::default(); TRACKS.len()];
        loop {
            ding_id += 1;
            if let Ok(Some(event)) = _rx.try_next() {
                last_event = Some(event);
                last_event_time = get_current_ts();
                match event {
                    AudioEvent::StartSpin => {
                        oscillators.set(Some(OscillatorList::new(TRACKS.len() as u8)));
                    }
                    AudioEvent::StopAudio => {
                        oscillators.set(None);
                    }
                    _ => {}
                }
            }

            if let (Some(fms), Some(last_event)) = (oscillators.write().as_mut(), last_event) {
                let sequence_info = SoundSequenceInfo {
                    last_event,
                    ding_id,
                    time_since_event: (get_current_ts() - last_event_time).max(0.0001),
                };

                for ((_fn, _last_output), _fm) in
                    (TRACKS.iter().zip(last_outputs.iter_mut())).zip(fms.v.iter_mut())
                {
                    *_last_output = _fn(sequence_info, *_last_output);
                    _fm.set_gain(_last_output.gain.clamp(0.0, 1.0) * audio_settings.peek().volume.clamp(0.0,1.0) as f32);
                    _fm.set_note(_last_output.note.clamp(20,127));
                    _fm.set_fm_amount(_last_output.fm_amount.clamp(0.0,1.0));
                    _fm.set_fm_frequency(_last_output.fm_freq.clamp(0.0,1.0));
                }
            } else {
                // no oscillators = no audio
            }
            sleep(0.11).await;
        }
    });
    let tx: UnboundedSender<AudioEvent> = co.tx();

    use_context_provider(move || tx);
}

pub fn send_audio_event(ev: AudioEvent) {
    let tx = use_context::<UnboundedSender<AudioEvent>>();
    if let Err(e) = tx.unbounded_send(ev) {
        info!("audio event send error: {e}");
    } else {
    }
}

struct OscillatorList {
    v: Vec<FmOsc>,
}
impl OscillatorList {
    fn new(osc_count: u8) -> Self {
        assert!(osc_count < 10 && osc_count > 0);
        let mut v = vec![];
        for _x in 0..osc_count {
            let mut fm = FmOsc::new().unwrap();
            fm.set_note(60);
            fm.set_fm_frequency(0.0);
            fm.set_fm_amount(0.0);
            fm.set_gain(0.0);
            v.push(fm);
        }

        Self { v }
    }
}

/// Converts a midi note to frequency
///
/// A midi note is an integer, generally in the range of 21 to 108
fn midi_to_freq(note: u8) -> f32 {
    27.5 * 2f32.powf((note as f32 - 21.0) / 12.0)
}

struct FmOsc {
    ctx: AudioContext,
    /// The primary oscillator.  This will be the fundamental frequency
    primary: web_sys::OscillatorNode,

    /// Overall gain (volume) control
    gain: web_sys::GainNode,

    /// Amount of frequency modulation
    fm_gain: web_sys::GainNode,

    /// The oscillator that will modulate the primary oscillator's frequency
    fm_osc: web_sys::OscillatorNode,

    /// The ratio between the primary frequency and the fm_osc frequency.
    ///
    /// Generally fractional values like 1/2 or 1/4 sound best
    fm_freq_ratio: f32,

    fm_gain_ratio: f32,
}

impl Drop for FmOsc {
    fn drop(&mut self) {
        let _ = self.ctx.close();
    }
}

impl FmOsc {
    pub fn new() -> Result<FmOsc, JsValue> {
        let ctx = web_sys::AudioContext::new()?;

        // Create our web audio objects.
        let primary = ctx.create_oscillator()?;
        let fm_osc = ctx.create_oscillator()?;
        let gain = ctx.create_gain()?;
        let fm_gain = ctx.create_gain()?;

        // Some initial settings:
        primary.set_type(OscillatorType::Sine);
        primary.frequency().set_value(440.0); // A4 note
        gain.gain().set_value(0.0); // starts muted
        fm_gain.gain().set_value(0.0); // no initial frequency modulation
        fm_osc.set_type(OscillatorType::Sine);
        fm_osc.frequency().set_value(0.0);

        // Connect the nodes up!

        // The primary oscillator is routed through the gain node, so that
        // it can control the overall output volume.
        primary.connect_with_audio_node(&gain)?;

        // Then connect the gain node to the AudioContext destination (aka
        // your speakers).
        gain.connect_with_audio_node(&ctx.destination())?;

        // The FM oscillator is connected to its own gain node, so it can
        // control the amount of modulation.
        fm_osc.connect_with_audio_node(&fm_gain)?;

        // Connect the FM oscillator to the frequency parameter of the main
        // oscillator, so that the FM node can modulate its frequency.
        fm_gain.connect_with_audio_param(&primary.frequency())?;

        // Start the oscillators!
        primary.start()?;
        fm_osc.start()?;

        Ok(FmOsc {
            ctx,
            primary,
            gain,
            fm_gain,
            fm_osc,
            fm_freq_ratio: 0.0,
            fm_gain_ratio: 0.0,
        })
    }

    /// Sets the gain for this oscillator, between 0.0 and 1.0.
    pub fn set_gain(&self, mut gain: f32) {
        gain = gain.clamp(0.0, 1.0);
        self.gain.gain().set_value(gain);
    }

    pub fn set_primary_frequency(&self, freq: f32) {
        self.primary.frequency().set_value(freq);

        // The frequency of the FM oscillator depends on the frequency of the
        // primary oscillator, so we update the frequency of both in this method.
        self.fm_osc.frequency().set_value(self.fm_freq_ratio * freq);
        self.fm_gain.gain().set_value(self.fm_gain_ratio * freq);
    }

    pub fn set_note(&self, note: u8) {
        let freq = midi_to_freq(note);
        self.set_primary_frequency(freq);
    }

    /// This should be between 0 and 1, though higher values are accepted.
    pub fn set_fm_amount(&mut self, amt: f32) {
        self.fm_gain_ratio = amt;

        self.fm_gain
            .gain()
            .set_value(self.fm_gain_ratio * self.primary.frequency().value());
    }

    /// This should be between 0 and 1, though higher values are accepted.
    pub fn set_fm_frequency(&mut self, amt: f32) {
        self.fm_freq_ratio = amt;
        self.fm_osc
            .frequency()
            .set_value(self.fm_freq_ratio * self.primary.frequency().value());
    }
}
