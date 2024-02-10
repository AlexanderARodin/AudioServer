use std::sync::Arc;
use rustysynth::*;

use raalog::log;

use super::super::audio_core::AudioRender;

use super::super::midi_lib::MidiReceiver;
//  //  //  //  //  //  //
use super::MidiSynth;


//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //
pub struct RustySynthWrapper{
    synth: Synthesizer,
}
impl RustySynthWrapper {
    pub fn new( sample_rate: &usize, mut sound_font_source: &[u8] ) -> Result< Self, Box<dyn std::error::Error> > {
        let init_params = SynthesizerSettings::new( *sample_rate as i32 );
        let arc_snd_fnt = Arc::new( SoundFont::new( &mut sound_font_source )? );
        let new_synth = Synthesizer::new(&arc_snd_fnt, &init_params)?;
        Ok( Self{synth: new_synth} )
    }
}
impl Drop for RustySynthWrapper {
    fn drop(&mut self) {
        self.reset();
        log::droping("RustySynthWrapper");
    }
}

//  //  //  //  //  //  //  //
//      RENDER interface
//  //  //  //  //  //  //  //
impl AudioRender for RustySynthWrapper {
    fn render(&mut self, left: &mut [f32], right: &mut [f32]) {
        self.synth.render(&mut left[..], &mut right[..]);
    }
}


//  //  //  //  //  //  //  //
//      ?
//  //  //  //  //  //  //  //
impl MidiReceiver for RustySynthWrapper {
    fn reset(&mut self) {
        log::info("RustySynthWrapper: reset");
        self.synth.reset();
    }
    fn process_midi_command(&mut self, 
                            channel: i32, command: i32, 
                            data1: i32, data2: i32) 
    {
        self.synth.process_midi_message(channel, command, 
                            data1, data2)
    }
}

impl MidiSynth for RustySynthWrapper {
    fn get_as_midi_receiver(&mut self) -> &mut dyn MidiReceiver {
        self
    }
}

