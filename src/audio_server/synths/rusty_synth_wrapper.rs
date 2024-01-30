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
    pub fn new( sample_rate: &usize, mut sound_font_source: &[u8] ) -> Result<Self, SynthesizerError> {
        let init_params = SynthesizerSettings::new( *sample_rate as i32 );
        let snd_fnt = Arc::new( SoundFont::new(&mut sound_font_source).expect("loading SF error... need to proceccing") );
        let new_synth = Synthesizer::new(&snd_fnt, &init_params);
        match new_synth {
            Err(e) => {
                let errmsg = err_to_string( &e );
                log::error( &format!("RustySynthWrapper: {errmsg}") );
                Err(e)
                },
            Ok(loaded_synth) => {
                log::creating("RustySynthWrapper");
                Ok(
                    Self{
                        synth: loaded_synth
                    }
                )
            }
        }
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
        //log::tick();
        self.synth.render(&mut left[..], &mut right[..]);
    }
}

//  //  //  //  //  //  //  //
//      RENDER interface
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

//  //  //  //  //  //  //  //
//      Err
//  //  //  //  //  //  //  //
fn err_to_string( e: &SynthesizerError ) -> String {
    match e {
        SynthesizerError::SampleRateOutOfRange(sample_rate) => {
            return format!("SynthesizerError.SampleRateOutOfRange: <{}>", sample_rate);
        },
        SynthesizerError::BlockSizeOutOfRange(size) => {
            return format!("SynthesizerError.BlockSizeOutOfRange: <{}>", size);
        },
        SynthesizerError::MaximumPolyphonyOutOfRange(size) => {
            return format!("SynthesizerError.MaximumPolyphonyOutOfRange: <{}>", size);
        },
        _ => {
            return format!("SynthesizerError.<unknown>");
        },
    }
}
