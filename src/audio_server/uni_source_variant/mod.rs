use std::error::Error;
use std::sync::{Arc,Mutex};
use toml::{ Table, Value };

use raalog::log;


    use super::audio_core::AudioRender;
    use super::midi_lib::{MidiReceiver,MidiMessage};
    use super::synths::simple_synth::SimpleSynth;
    use super::synths::rusty_synth_wrapper::RustySynthWrapper;
    use super::midi_sequencer::MidiSequencer;



//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //
mod impl_new;

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
    pub(crate) fn new( au_tbl: &Table, sample_rate: &usize, time_increment: f32, data: Option<&[u8]> ) -> Result<Self, Box<dyn Error>> {
        if let Some(name_val) = au_tbl.get("Name") {
            if let Value::String(name) = name_val {
                match name.as_str() {
                    "Sequencer" => Self::new_sequencer( au_tbl, sample_rate, time_increment, data ),
                    _ => Self::create_synth(&name, sample_rate, data),
                }
            }else{
                return Err(Box::from("Name have to be text name of Synth or Sequencer"));
            }
        }else{
            return Err(Box::from("no Name in AudioSource"));
        }
    }

    fn new_sequencer( au_tbl: &Table, sample_rate: &usize, time_increment: f32, data: Option<&[u8]> ) -> Result<Self, Box<dyn Error>> {
        if let Some(au_seq_val) = au_tbl.get("Sequencer") {
            if let Value::Table(au_seq_tbl) = au_seq_val {
                return Self::create_sequencer( au_seq_tbl, sample_rate, time_increment, data );
            }else{
                return Err(Box::from("invalid Sequencer section"));
            }
        }else{
            return Err(Box::from("no Sequencer sub-section"));
        }
    }
}

//  //  //  //  //  //  //  //
//      interface
//  //  //  //  //  //  //  //
impl UniSourceVariant {
    pub(crate) fn send_to_synth(&mut self, midi: &MidiMessage) {
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
                locked_sequencer.send_to_synth(midi);
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
    fn create_rustysynth(sample_rate: &usize, data: Option<&[u8]> ) -> Result< Arc<Mutex<RustySynthWrapper>>, Box<dyn Error> > {
        if let Some(mut sf_source) = data {
            match RustySynthWrapper::new( &sample_rate, &mut sf_source ) {
                Ok(ryssyn) => {
                    let arcmut_wrapper = Arc::new(Mutex::new( ryssyn ));
                    return Ok( arcmut_wrapper );
                },
                Err(e) => {
                    log::error(&e.to_string());
                    return Err(
                            Box::from( "invoke_set_uni_source: error creating of RustySynthWrapper" )
                        );
                },
            }
        }else{
            return Err( Box::from("invoke_set_uni_source: there is no SoundFont for init") );
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
        if let Ok(_ryssy) = UniSourceVariant::create_rustysynth(&44100, None ) {
            assert!(false, "shound be error");
        }
    }
}
