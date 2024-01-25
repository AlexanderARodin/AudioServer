
pub mod simple_synth;
pub mod rusty_synth_wrapper;




use super::audio_core::AudioRender;
use super::midi_lib::MidiReceiver;

pub trait MidiSynth: AudioRender + MidiReceiver {
    fn get_as_midi_receiver(&mut self) -> &mut dyn MidiReceiver;
}

