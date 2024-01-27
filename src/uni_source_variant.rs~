//use std::error::Error;
use std::sync::{Arc,Mutex};

use raalog::log;


use crate::audio_core::AudioRender;

use crate::midi_lib::{MidiReceiver,MidiMessage};

use crate::synths::simple_synth::SimpleSynth;
use crate::synths::rusty_synth_wrapper::RustySynthWrapper;

use crate::midi_sequencer::MidiSequencer;



//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //
pub(crate) enum UniSourceVariant {
    Silence,
    #[allow(dead_code)]
    Audio( Arc<Mutex<dyn AudioRender>> ),
    Simple( Arc<Mutex<SimpleSynth>> ),
    Rusty( Arc<Mutex<RustySynthWrapper>> ),
    Sequencer( Arc<Mutex<MidiSequencer>> ),
}
use UniSourceVariant::*;

impl UniSourceVariant {
    pub(crate) fn new<'a>( config: &str, sample_rate: &usize, time_increment: f32, data: Option<&[u8]> ) -> Result<Self, Box<&'a str>> {
        match config {
            "None" => {
                return Ok( Silence );
            },
            "SimpleSynth" => {
                let synth = SimpleSynth::new(sample_rate);
                let arcmut_wrapper = Arc::new(Mutex::new(synth));
                return Ok( Simple(arcmut_wrapper) );
            },
            "RustySynt" => {
                let arcmut_wrapper = Self::createRustySynth(&sample_rate, data )?;
                return Ok(Rusty( arcmut_wrapper ));
            },
            "Sequencer:RustySynt" => {
                let mut sequencer = MidiSequencer::new(time_increment);

                let arcmut_wrapper = Self::createRustySynth(&sample_rate, data )?;
                sequencer.install_synth( Some(arcmut_wrapper.clone()) );

                let sequencer_wrapper = Arc::new(Mutex::new( sequencer ));
                return Ok( Sequencer(sequencer_wrapper) );
            },
            _ => {
                return Err( Box::new("invoke_set_uni_source: unknow config") );
            },
        }
    }
}

//  //  //  //  //  //  //  //
//      interface
//  //  //  //  //  //  //  //
impl UniSourceVariant {
    pub(crate) fn send_to_synth(&mut self, midi_msg: &MidiMessage) {
        let midi = midi_msg.to_midi_general();
        match &self {
            Silence => {
                return
            },
            Simple( simsyn ) => {
                let mut locked_receiver = simsyn.lock()
                    .expect("panick on locking UniSourceVariant::Simple( simsyn )");
                locked_receiver.process_midi_command( midi.channel, 
                                                      midi.command, 
                                                      midi.data1, 
                                                      midi.data2 );
            },
            Rusty( ryssyn ) => {
                let mut locked_receiver = ryssyn.lock()
                    .expect("panick on locking UniSourceVariant::Rusty( ryssyn )");
                locked_receiver.process_midi_command( midi.channel, 
                                                      midi.command, 
                                                      midi.data1, 
                                                      midi.data2 );
            },
            Sequencer( sequencer ) => {
                let mut locked_sequencer = sequencer.lock()
                    .expect("panick on locking UniSourceVariant::Sequencer( sequencer )");
                locked_sequencer.send_to_synth(midi_msg);
            },
            _ => {
                log::info("outstanding");
            }
        }
    }
}


//  //  //  //  //  //  //  //
//      internal
//  //  //  //  //  //  //  //
impl UniSourceVariant {
    fn createRustySynth<'a>(sample_rate: &usize, data: Option<&[u8]> ) -> Result< Arc<Mutex<RustySynthWrapper>>, Box<&'a str> > {
        if let Some(mut sf_source) = data {
            match RustySynthWrapper::new( &sample_rate, &mut sf_source ) {
                Ok(ryssyn) => {
                    let arcmut_wrapper = Arc::new(Mutex::new( ryssyn ));
                    return Ok( arcmut_wrapper );
                },
                Err(e) => {
                    log::error(&e.to_string());
                    return Err(
                            Box::new( "invoke_set_uni_source: error creating of RustySynthWrapper" )
                        );
                },
            }
        }else{
            return Err( Box::new("invoke_set_uni_source: there is no SoundFont for init") );
        }
    }
}



//  //  //  //  //  //  //  //
//      TESTs
//  //  //  //  //  //  //  //
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rusty_error_on_empty_data() {
        if let Ok(_ryssy) = UniSourceVariant::createRustySynth(&44100, None ) {
            assert!(false, "shound be error");
        }
    }
}
