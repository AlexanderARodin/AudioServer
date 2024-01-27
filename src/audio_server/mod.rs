#[allow(non_snake_case)]
use std::error::Error;
use toml::Table;
use raalog::log;

    mod audio_core;
    use audio_core::AudioCore;

    mod uni_source_variant;
    use uni_source_variant::{ UniSourceVariant, UniSourceVariant::* };

    mod midi_lib;
    //use midi_lib::{ MidiMessage};
    use midi_lib::{ MidiSequence };

    mod synths;
    mod midi_sequencer;


//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //
mod impl_config;
mod impl_exec;

pub struct AudioServer {
    audio_core: AudioCore,
    uni_source: UniSourceVariant,
}

impl AudioServer {
    pub fn new( ) -> Self {
        log::creating("AudioServer");
        Self{ 
            audio_core: AudioCore::new(),
            uni_source: UniSourceVariant::Silence,
        }
    }
}
impl Drop for AudioServer {
    fn drop(&mut self) {
        let _ = self.exec("stop");
        self.audio_core.stop();
        log::droping("AudioServer");
    }
}


//  //  //  //  //  //  //  //
//      main INTERFACE
//  //  //  //  //  //  //  //
impl AudioServer {

    //  //  //  //  //  //  //
    pub fn config( &mut self, setup: &str, data: Option<&[u8]>  ) -> Result<(), Box<dyn Error>> {
        let tbl = setup.parse::<Table>()?;
        self.invoke_config_parsing( &tbl, data )
    }

    //  //  //  //  //  //  //
    pub fn exec( &mut self, commands: &str) -> Result<(), Box<dyn Error>> {
        #[cfg(test)]
        if commands == "error" {
            return Err(Box::from("error on error"));
        }
        if commands == "" {
            return Ok(());
        }
        match commands {
            "stop" => {
                self.audio_core.stop();
                return Ok(());
            },
            "start" => {
                return self.audio_core.start();
            },
            _ => {
                return self.invoke_exec( commands );
            },
        }
    }

    //  //  //  //  //  //  //
    pub fn state(&self) -> &'static str {
        if self.audio_core.is_active() {
            match &self.uni_source {
                Sequencer(sequencer) => {
                    let locked_sequencer = sequencer.lock()
                        .expect("FATAL locking Sequencer");
                    if locked_sequencer.get_state() {
                        "running"
                    }else{
                        "REALTIME"
                    }
                },
                _ => {
                    "running"
                }
        }
        }else{
            "inactive"
        }
    }
}


//  //  //  //  //  //  //  //
//      internal
//  //  //  //  //  //  //  //
impl AudioServer {
    fn install_source_to_audio(&mut self) {
        match &self.uni_source {
            Silence => {
                self.audio_core.install_render(None);
            },
            Audio(wrapped_audio_render) => {
                self.audio_core.install_render(Some( wrapped_audio_render.clone() ));
            },
            Simple(simsyn) => {
                self.audio_core.install_render(Some( simsyn.clone() ));
            },
            Rusty(ryssyn) => {
                self.audio_core.install_render(Some( ryssyn.clone() ));
            },
            Sequencer(sequencer) => {
                self.audio_core.install_render(Some( sequencer.clone() ));
            },
        }
    }
    fn set_sequence(&mut self, seq: MidiSequence, is_auto_repeat: bool ) {
        match &self.uni_source {
            Sequencer(sequencer) => {
                let mut locked_sequencer = sequencer.lock()
                    .expect("FATAL of locking Sequencer");
                locked_sequencer.set_midi_sequence(seq, is_auto_repeat );
            },
            _ => {
                log::error("set_sequence: NOT a Sequencer.Ignoring")
            },
            
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
    fn exec_ok() {
        let mut audio = AudioServer::new();
        let res = audio.exec("");
        if let Ok(()) = res {
        }else{
            assert!( false, "EXEC shoud be Ok(())");
        }
    }
    #[test]
    fn exec_error() {
        let mut audio = AudioServer::new();
        let res = audio.exec("error");
        if let Err(e) = res {
            let err_msg = &e.to_string();
            log::info(err_msg);
            assert!( err_msg == "error on error", "EXEC.Err shoud be <error on error>");
        }else{
            assert!( false, "EXEC shoud be Err");
        }
    }
}


