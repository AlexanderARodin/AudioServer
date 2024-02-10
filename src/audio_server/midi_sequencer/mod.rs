use raalog::log;

use crate::prelude::*;
    use super::audio_core::AudioRender;

    use super::midi_lib::{MidiReceiver,MidiSequence,MidiMessage};
pub use super::synths::MidiSynth as MidiSynth;


//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //
pub struct MidiSequencer{
    time_increment: f32,
    sequence: MidiSequence,
    is_auto_repeat: bool,
    midi_synth: Option< ArcMut<dyn MidiSynth> >,
}

impl MidiSequencer {
    pub fn new( time_increment: f32 ) -> Self {
        log::creating("MidiSequencer");
        Self{
            time_increment,
            sequence: MidiSequence::new(),
            is_auto_repeat: false,
            midi_synth: None,
        }
    }
}
impl Drop for MidiSequencer {
    fn drop(&mut self) {
        log::droping("MidiSequencer");
    }
}

//  //  //  //  //  //  //  //
//      interface
//  //  //  //  //  //  //  //
impl MidiSequencer {

    pub fn set_midi_sequence(&mut self, seq: MidiSequence, is_auto_repeat: bool) {
        self.is_auto_repeat = is_auto_repeat;
        self.sequence = seq;
    }
    pub fn install_synth(&mut self, new_synth: Option< ArcMut<dyn MidiSynth> >) {
        self.midi_synth = new_synth;
    }
    pub fn send_to_synth(&mut self, midi: &MidiMessage) {
        if let Some( synth ) = &self.midi_synth {
            let mut locked_synth = synth.lock()
                .expect("FATAL: can't lock MidiSynth!");
            let midi_recevier: &mut dyn MidiReceiver = locked_synth.get_as_midi_receiver();
            midi_recevier.process_midi_command( midi.channel, 
                                                midi.command, 
                                                midi.data1, 
                                                midi.data2 );
        }
    }
    pub fn is_sequence_finished(&self) -> bool {
        self.sequence.is_finished()
    }
}
//  //  //  //  //  //  //  //
//      RENDER interface
//  //  //  //  //  //  //  //
impl AudioRender for MidiSequencer {

    fn render(&mut self, left: &mut [f32], right: &mut [f32]) {
        match &self.midi_synth {
            None => {
                fill_silence(left);
                fill_silence(right);
            },
            Some(synth) => {
                let mut locked_synth = synth.lock()
                    .expect("FATAL: can't lock MidiSynth!");
                let midi_recevier: &mut dyn MidiReceiver = locked_synth.get_as_midi_receiver();
                self.sequence.send_next_sequence( self.time_increment, midi_recevier );
                locked_synth.render(left, right);
                if self.sequence.is_finished() {
                    if self.is_auto_repeat {
                        self.sequence.restart();
                    }
                }
            }
        }
    }
}


//  //  //  //  //  //  //  //
//      UTIL
//  //  //  //  //  //  //  //
fn fill_silence(buf: &mut [f32]) {
    for sample in buf {
        *sample = 0_f32;
   }
}

