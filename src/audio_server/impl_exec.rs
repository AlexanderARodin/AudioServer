use std::error::Error;
use raalog::log;

    use super::midi_lib::{ MidiMessage, MidiSequence };
    use super::uni_source_variant::UniSourceVariant::Sequencer;



//  //  //  //  //  //  //  //
//      exec impl
//  //  //  //  //  //  //  //
use super::AudioServer;

impl AudioServer {
    pub(crate) fn invoke_exec_parsing(&mut self, commands: &str) -> Result<(), Box<dyn Error>> {
        match interpret_as_midi(commands) {
            Ok(Some(midi)) => {
                self.uni_source.send_to_synth(&midi);
                return Ok(());
            },
            Err(e) => return Err(e),
            Ok(None) => {},
        }
        match commands {
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
//      UTILs
//  //  //  //  //  //  //  //
fn interpret_as_midi( cmd: &str ) -> Result< Option<MidiMessage>, Box<dyn Error> > {
    if cmd.starts_with("on") {
        let from_index = 2;
        let (key, velocity) = extract_onoff_params(cmd.get(from_index..))?;
        return Ok(Some(MidiMessage::NoteOn(1,key,velocity)));
    }
    if cmd.starts_with("off") {
        let from_index = 3;
        let (key, velocity) = extract_onoff_params(cmd.get(from_index..))?;
        return Ok(Some(MidiMessage::NoteOff(1,key,velocity)));
    }
    Ok(None)
}
fn extract_onoff_params( opt_params_str: Option<&str> ) -> Result< (i32, i32) , Box<dyn Error> > {
    if let Some(params_str) = opt_params_str {
        let delimeter_index = match params_str.rfind('#') {
            Some(index) => index,
            None => params_str.len(),
        };
        let key = extract_key( params_str.get(0..delimeter_index) )?;
        let vel = extract_velociy_or_return_64( params_str.get( (1+delimeter_index).. ))?;
        return Ok( (key,vel) );
    }else{
        return Err(Box::from("On//Off has to have valid params"))
    }
}
fn extract_key( opt_key_str: Option<&str> ) -> Result< i32, Box<dyn Error> > {
    if let Some(key_str) = opt_key_str {
        return Ok( key_str.parse::<i32>()? );
    }else{
        return Err(Box::from("key has to be presented"));
    }
}
fn extract_velociy_or_return_64( opt_key_str: Option<&str> ) -> Result< i32, Box<dyn Error> > {
    if let Some(key_str) = opt_key_str {
        return Ok( key_str.parse::<i32>()? );
    }else{
        return Ok(64);
    }
}


//  //  //  //  //  //  //  //
//      TESTs
//  //  //  //  //  //  //  //
#[cfg(test)]
mod interpret_notes {
    use super::*;

    #[test]
    fn on5() {
        let mist;
        match interpret_as_midi( "on5" ) {
            Ok(Some(midi)) => {
                let gmidi = midi.to_midi_general();
                if gmidi.channel != 1 {
                    mist = "midi channel has to be 1";
                }else if gmidi.command != 0x90  {
                    mist = "midi command has to be NoteOn";
                }else if gmidi.data1 != 5 {
                    mist = "midi data1 has to be key5";
                }else if gmidi.data2 != 64 {
                    mist = "midi data2 has to be velo64";
                }else{
                    mist = "";
                }
            },
            Ok(None) => {
                mist = "has NOT be None";
            },
            Err(e) => {
                mist = "has NOT to be Error";
                log::error(&e.to_string());
            },
        }
        assert!( mist == "", ">> {mist} <<");
    }
    #[test]
    fn off_93_12() {
        let mist;
        match interpret_as_midi( "off93#12" ) {
            Ok(Some(midi)) => {
                let gmidi = midi.to_midi_general();
                if gmidi.channel != 1 {
                    mist = "midi channel has to be 1";
                }else if gmidi.command != 0x80  {
                    mist = "midi command has to be NoteOff";
                }else if gmidi.data1 != 93 {
                    mist = "midi data1 has to be key93";
                }else if gmidi.data2 != 12 {
                    mist = "midi data2 has to be velo12";
                }else{
                    mist = "";
                }
            },
            Ok(None) => {
                mist = "has NOT be None";
            },
            Err(e) => {
                mist = "has NOT to be Error";
                log::error(&e.to_string());
            },
        }
        assert!( mist == "", ">> {mist} <<");
    }
    #[test]
    fn off_32() {
        let mist;
        match interpret_as_midi( "off32" ) {
            Ok(Some(midi)) => {
                let gmidi = midi.to_midi_general();
                if gmidi.channel != 1 {
                    mist = "midi channel has to be 1";
                }else if gmidi.command != 0x80  {
                    mist = "midi command has to be NoteOff";
                }else if gmidi.data1 != 32 {
                    mist = "midi data1 has to be key32";
                }else if gmidi.data2 != 64 {
                    mist = "midi data2 has to be velo64";
                }else{
                    mist = "";
                }
            },
            Ok(None) => {
                mist = "has NOT be None";
            },
            Err(e) => {
                mist = "has NOT to be Error";
                log::error(&e.to_string());
            },
        }
        assert!( mist == "", ">> {mist} <<");
    }
    #[test]
    fn on66_77() {
        let mist;
        match interpret_as_midi( "on66#77" ) {
            Ok(Some(midi)) => {
                let gmidi = midi.to_midi_general();
                if gmidi.channel != 1 {
                    mist = "midi channel has to be 1";
                }else if gmidi.command != 0x90  {
                    mist = "midi command has to be NoteOn";
                }else if gmidi.data1 != 66 {
                    mist = "midi data1 has to be key66";
                }else if gmidi.data2 != 77 {
                    mist = "midi data2 has to be velo77";
                }else{
                    mist = "";
                }
            },
            Ok(None) => {
                mist = "has NOT be None";
            },
            Err(e) => {
                mist = "has NOT to be Error";
                log::error(&e.to_string());
            },
        }
        assert!( mist == "", ">> {mist} <<");
    }
    #[test]
    fn on_invalid() {
        let mist;
        if let Err(e) = interpret_as_midi( "on_" ) {
            mist = "";
            log::error(&e.to_string());
        }else{
            mist = "ON with invalid params has to return Error";
        }
        assert!( mist == "", ">> {mist} <<");
    }
    #[test]
    fn on_() {
        let mist;
        if let Err(e) = interpret_as_midi( "on#" ) {
            mist = "";
            log::error(&e.to_string());
        }else{
            mist = "ON with invalid params has to return Error";
        }
        assert!( mist == "", ">> {mist} <<");
    }
    #[test]
    fn empty_on() {
        let mist;
        if let Err(e) = interpret_as_midi( "on" ) {
            mist = "";
            log::error(&e.to_string());
        }else{
            mist = "ON without params has to return Error";
        }
        assert!( mist == "", ">> {mist} <<");
    }
    #[test]
    fn empty_off() {
        let mist;
        if let Err(e) = interpret_as_midi( "off" ) {
            mist = "";
            log::error(&e.to_string());
        }else{
            mist = "OFF without params has to return Error";
        }
        assert!( mist == "", ">> {mist} <<");
    }
    #[test]
    fn empty_command() {
        let mist;
        if let Ok(None) = interpret_as_midi( "" ) {
            mist = "";
        }else{
            mist = "empty EXEC shoud be Ok(None)";
        }
        assert!( mist == "", ">> {mist} <<");
    }
}


