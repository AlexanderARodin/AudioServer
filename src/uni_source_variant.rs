//use std::error::Error;
use std::sync::{Arc,Mutex};

use raalog::log;


use crate::audio_core::AudioRender;

use crate::midi_lib::{MidiReceiver,MidiMessage};

use crate::synths::simple_synth::SimpleSynth;
use crate::synths::rusty_synth_wrapper::RustySynthWrapper;

use crate::midi_sequencer::MidiSequencer;



static SF_PIANO:   &'static [u8] = include_bytes!("../SoundFonts/Piano Grand.SF2");
static SF_STRINGS: &'static [u8] = include_bytes!("../SoundFonts/String Marcato.SF2");
//static SF_ORGAN:   &'static [u8] = include_bytes!("../../SoundFonts/Organ Chorus.SF2");


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
    pub(crate) fn new<'a>( config: &str, sample_rate: &usize, time_increment: f32 ) -> Result<Self, Box<&'a str>> {
        match config {
            "None" => {
                return Ok( Silence );
            },
            "SimpleSynth" => {
                let synth = SimpleSynth::new(sample_rate);
                let arcmut_wrapper = Arc::new(Mutex::new(synth));
                return Ok( Simple(arcmut_wrapper) );
            },
            "RustySynt - Strings" => {
                let mut sf_source = SF_STRINGS;
                if let Ok(ryssyn) = RustySynthWrapper::new( &sample_rate, &mut sf_source ) {
                    let arcmut_wrapper = Arc::new(Mutex::new( ryssyn ));
                    return Ok( Rusty(arcmut_wrapper) );
                }
                return Err( Box::new("invoke_set_uni_source: err 1") );
            },
            "RustySynt - Piano" => {
                let mut sf_source = SF_PIANO;
                if let Ok(ryssyn) = RustySynthWrapper::new( &sample_rate, &mut sf_source ) {
                    let arcmut_wrapper = Arc::new(Mutex::new( ryssyn ));
                    return Ok( Rusty(arcmut_wrapper) );
                }
                return Err( Box::new("invoke_set_uni_source: err 2") );
            },
            "Sequencer:RustySynt - Strings" => {
                let mut sequencer = MidiSequencer::new(time_increment);
                let mut sf_source = SF_STRINGS;
                if let Ok(ryssyn) = RustySynthWrapper::new( &sample_rate, &mut sf_source ) {
                    let arcmut_wrapper = Arc::new(Mutex::new( ryssyn ));
                    sequencer.install_synth( Some(arcmut_wrapper.clone()) );
                }
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

