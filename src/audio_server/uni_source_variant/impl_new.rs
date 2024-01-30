use std::error::Error;
use std::sync::{Arc,Mutex};
use toml::{ Table, Value };

    use super::super::synths::simple_synth::SimpleSynth;
    use super::super::midi_sequencer::MidiSequencer;


//  //  //  //  //  //  //  //
//      new impl
//  //  //  //  //  //  //  //
use super::UniSourceVariant;
use UniSourceVariant::*;

impl UniSourceVariant {
    pub(crate) fn create_synth( source_name: &str, sample_rate: &usize, data: Option<&[u8]> ) -> Result<Self, Box<dyn Error>> {
        match source_name {
            "None" => {
                return Ok( Silence );
            },
            "Simple" => {
                let synth = SimpleSynth::new(sample_rate);
                let arcmut_wrapper = Arc::new(Mutex::new(synth));
                return Ok( Simple(arcmut_wrapper) );
            },
            "RustySynth" => {
                let arcmut_wrapper = Self::create_rustysynth(&sample_rate, data )?;
                return Ok(Rusty( arcmut_wrapper ));
            },
            _ => {
                return Err( Box::from("invoke_set_uni_source: unknow config") );
            },
        }
    }
    pub(crate) fn create_sequencer( au_seq_tbl: &Table, sample_rate: &usize, time_increment: f32, data: Option<&[u8]> ) -> Result<Self, Box<dyn Error>> {
        if let Some(main_val) = au_seq_tbl.get("MainVoice") {
            if let Value::String(name) = main_val {
                let mut sequencer = MidiSequencer::new(time_increment);
                //let main_voice_synth = Self::create_synth(&name, sample_rate, time_increment, data)?;
                match Self::create_synth(&name, sample_rate, data)? {
                    Simple(sim) => {
                        sequencer.install_synth( Some( sim.clone()) );
                    },
                    Rusty(synth) => {
                        sequencer.install_synth( Some( synth.clone()) );
                    },
                    _ => {
                    },
                }
                let sequencer_wrapper = Arc::new(Mutex::new( sequencer ));
                return Ok( Sequencer(sequencer_wrapper) );
            }else{
                return Err(Box::from("Name of MainVoice have to be text name of Synth"));
            }
        }else{
            return Err(Box::from("no MainVoice in Sequencer"));
        }
    }
}
