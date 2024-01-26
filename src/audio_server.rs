#![allow(non_snake_case)]

use raalog::log;

use crate::audio_core::AudioCore;
use crate::uni_source_variant::{ UniSourceVariant };
use crate::uni_source_variant::UniSourceVariant::*;

use crate::midi_lib::{MidiMessage,MidiSequence};


//static SF_PIANO:   &'static [u8] = include_bytes!("../SoundFonts/Piano Grand.SF2");
//static SF_STRINGS: &'static [u8] = include_bytes!("../SoundFonts/String Marcato.SF2");
//static SF_ORGAN:   &'static [u8] = include_bytes!("../../SoundFonts/Organ Chorus.SF2");


//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //
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
        self.audio_core.stop();
        //
        log::droping("AudioServer");
    }
}


//  //  //  //  //  //  //  //
//      main INTERFACE
//  //  //  //  //  //  //  //
impl AudioServer {
    pub fn config( &mut self, setup: &str, data: Option<&[u8]>  ) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(test)]
        if setup == "error" {
            return Err(Box::from("error on error"));
        }
        if setup == "" {
            return Ok(());
        }

        self.invoke_set_uni_source( setup, data );
        Ok(())
    }
    
    pub fn exec( &mut self, commands: &str) -> Result<(), Box<dyn std::error::Error>> {
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
//      invoking
//  //  //  //  //  //  //  //
impl AudioServer {
    fn invoke_set_uni_source(&mut self, config: &str, data: Option<&[u8]> ) {
        log::info( &format!("--> <{config}>") );
        let sample_rate = self.audio_core.get_sample_rate();
        let time_increment = self.audio_core.get_time_increment();
        self.uni_source = match UniSourceVariant::new( config, &sample_rate, time_increment, data ) {
            Ok(variant) => variant,
            Err(e) => {
                log::error(&e);
                Silence
            },
        };
        self.install_source_to_audio();
    }
    fn invoke_exec(&mut self, commands: &str) -> Result<(), Box<dyn std::error::Error>> {
        match commands {
            "note ON" => {
                let midi = MidiMessage::NoteOn(1,60,127);
                self.uni_source.send_to_synth( &midi );
                Ok(())
            },
            "note ON2" => {
                let midi = MidiMessage::NoteOn(1,67,64);
                self.uni_source.send_to_synth( &midi );
                Ok(())
            },
            "note ON3" => {
                let midi = MidiMessage::NoteOn(1,72,1);
                self.uni_source.send_to_synth( &midi );
                Ok(())
            },
            "note OFF" => {
                let midi = MidiMessage::NoteOff(1,60,100);
                self.uni_source.send_to_synth( &midi );
                Ok(())
            },
            "seq 1" => {
                let mut seq = MidiSequence::new();
                seq.push( 0.0, &MidiMessage::NoteOn( 1,90,80) );
                seq.push( 0.5, &MidiMessage::NoteOff(1,90,80) );
                seq.push( 0., &MidiMessage::NoteOn( 1,91,80) );
                seq.push( 0.5, &MidiMessage::NoteOff(1,91,80) );
                seq.push( 0., &MidiMessage::NoteOn( 1,92,80) );
                seq.push( 0.5, &MidiMessage::NoteOff(1,92,80) );
                seq.push( 0., &MidiMessage::NoteOn( 1,91,80) );
                seq.push( 0.5, &MidiMessage::NoteOff(1,91,80) );
                seq.push( 1., &MidiMessage::NoteOff(1,92,80) );
                self.set_sequence(seq, false);
                Ok(())
            },
            "seq auto" => {
                let mut seq = MidiSequence::new();
                seq.push( 0.0, &MidiMessage::NoteOn( 1,90,80) );
                seq.push( 0.5, &MidiMessage::NoteOff(1,90,80) );
                seq.push( 0., &MidiMessage::NoteOn( 1,91,80) );
                seq.push( 0.5, &MidiMessage::NoteOff(1,91,80) );
                seq.push( 0., &MidiMessage::NoteOn( 1,92,80) );
                seq.push( 0.5, &MidiMessage::NoteOff(1,92,80) );
                seq.push( 0., &MidiMessage::NoteOn( 1,91,80) );
                seq.push( 0.5, &MidiMessage::NoteOff(1,91,80) );
                self.set_sequence(seq, true);
                Ok(())
            },
            _ => {
                log::error(commands);
                return Err(Box::from("EXEC not implemented"));
            },
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
    fn config_ok() {
        let mut audio = AudioServer::new();
        let res = audio.config("", None );
        if let Ok(()) = res {
        }else{
            assert!( false, "CONFIG shoud be Ok(())");
        }
    }
    #[test]
    fn config_error() {
        let mut audio = AudioServer::new();
        let res = audio.config("error", None );
        if let Err(e) = res {
            let err_msg = &e.to_string();
            log::info(err_msg);
            assert!( err_msg == "error on error", "CONFIG.Err shoud be <error on error>");
        }else{
            assert!( false, "CONFIG shoud be Err");
        }
    }
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


