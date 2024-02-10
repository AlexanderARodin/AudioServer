#[allow(non_snake_case)]
use toml::Table;
use raalog::log;
use crate::prelude::*;

    mod audio_core;
    use audio_core::AudioCore;

    mod uni_source_variant;
    use uni_source_variant::{ UniSourceVariant, UniSourceVariant::* };

    mod midi_lib;
    use midi_lib::midi_sequence::MidiSequence;

    mod synths;
    mod midi_sequencer;



//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //
mod impl_config;
//mod impl_core_config;
//mod impl_config_sequence;
mod impl_exec;

mod impl_core_exec;

//  //  //  //  //  //  //  //
pub enum Config<'a> {
    CoreConfigFromStr(&'a str, Vec<&'static [u8]> ),
    CoreConfig(&'a Table,      Vec<&'static [u8]> ),
}

pub struct AudioServer {
    audio_core: AudioCore,
    uni_source: UniSourceVariant,
    midi_sequence: Option<MidiSequence>,
    core_config: Table,
    sf_array: Vec<&'static [u8]>,
}

impl AudioServer {
    pub fn new( ) -> Self {
        log::creating("AudioServer");
        Self{ 
            audio_core: AudioCore::new(),
            uni_source: UniSourceVariant::Silence,
            midi_sequence: None,
            core_config: Table::new(),
            sf_array: Vec::new(),
        }
    }
}
impl Drop for AudioServer {
    fn drop(&mut self) {
        //let _ = self.exec("stop");
        self.audio_core.stop();
        log::droping("AudioServer");
    }
}


//  //  //  //  //  //  //  //
//      main INTERFACE
//  //  //  //  //  //  //  //
impl AudioServer {

    //  //  //  //  //  //  //
    pub fn load_config( &mut self, setup: &Config  ) -> ResultOf<()> {
        match setup {
            Config::CoreConfig( tbl, sf_array ) => {
                self.invoke_core_config_loading( &tbl, sf_array )
            },
            Config::CoreConfigFromStr( txt, sf_array ) => {
                let tbl = txt.parse::<Table>()?;
                self.invoke_core_config_loading( &tbl, sf_array )
            },
        }
    }
/*
    //  //  //  //  //  //  //
    pub fn config( &mut self, setup: &str, data: Option<&[u8]>  ) -> ResultOf<()> {
        let tbl = setup.parse::<Table>()?;
        self.invoke_config_parsing( &tbl, data )
    }
    */

    //  //  //  //  //  //  //
    pub fn exec( &mut self, commands: &str) -> ResultOf<()> {
        match commands {
            "stop" => {
                self.audio_core.stop();
                return Ok(());
            },
            "start" => {
                return self.audio_core.start();
            },
            _ => {
                return self.invoke_exec_parsing( commands );
            },
        }
    }

    //  //  //  //  //  //  //
    pub fn state(&self) -> &'static str {
        if self.audio_core.is_active() {
            if let Sequencer(sequencer) = &self.uni_source {
                let locked_sequencer = sequencer.lock()
                    .expect("FATAL locking Sequencer");
                if locked_sequencer.is_sequence_finished() {
                    "running"
                }else{
                    "REALTIME"
                }
            }else{
                "running"
            }
        }else{
            "inactive"
        }
    }
}



