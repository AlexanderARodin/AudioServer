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
    pub(crate) fn new( tbl: &Table, sample_rate: &usize, time_increment: f32, data: Option<&[u8]> ) -> Result<Self, Box<dyn Error>> {
        let main_synth_name;
        if let Some(main_val) = tbl.get("Main") {
            if let Value::String(main_name) = main_val {
                main_synth_name = main_name.as_str();
            }else{
                return Err(Box::from("Main have to be name of Synth"));
            }
        }else{
            return Err(Box::from("no Main in AudioSource"));
        }

        match main_synth_name {
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
            "Sequencer" => {
                let mut sequencer = MidiSequencer::new(time_increment);

                let arcmut_wrapper = Self::create_rustysynth(&sample_rate, data )?;
                sequencer.install_synth( Some(arcmut_wrapper.clone()) );

                let sequencer_wrapper = Arc::new(Mutex::new( sequencer ));
                return Ok( Sequencer(sequencer_wrapper) );
            },
            _ => {
                return Err( Box::from("invoke_set_uni_source: unknow config") );
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
