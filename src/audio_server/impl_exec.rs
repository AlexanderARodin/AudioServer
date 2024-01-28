use std::error::Error;
use raalog::log;

    use super::midi_lib::{ MidiMessage, MidiSequence };
    use super::uni_source_variant::UniSourceVariant::Sequencer;



//  //  //  //  //  //  //  //
//      exec impl
//  //  //  //  //  //  //  //
use super::AudioServer;

impl AudioServer {
    pub(crate) fn invoke_exec_parsing(&mut self, commands: &str) -> Result<(), Box<dyn std::error::Error>> {
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


