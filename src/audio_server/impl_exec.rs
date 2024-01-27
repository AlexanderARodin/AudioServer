use std::error::Error;
use raalog::log;

    use super::midi_lib::{ MidiMessage, MidiSequence };




//  //  //  //  //  //  //  //
//      exec impl
//  //  //  //  //  //  //  //
use super::AudioServer;

impl AudioServer {
    pub(crate) fn invoke_exec(&mut self, commands: &str) -> Result<(), Box<dyn std::error::Error>> {
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

