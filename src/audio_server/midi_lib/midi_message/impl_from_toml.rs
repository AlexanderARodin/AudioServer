use std::error::Error;
use toml::{Table, Value};


//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //
use super::MidiMessage;

impl MidiMessage {

    pub fn from( tml: &toml::Value ) -> Result< Self, Box<dyn Error> > {
        match tml {
            Value::Array(arr) => {
                let channel = try_get_integer(arr, 0)?;
                let command = try_get_command(arr, 1)?;
                let data1   = try_get_integer(arr, 2)?;
                let data2   = try_get_integer(arr, 3)?;
                return Ok(MidiMessage::new(channel, command, data1, data2));
            },
            _ => {
                return Err( Box::from("<MidiMessage::from>: argument is not an array") );
            },
        }
    }
}

//  //  //  //  //  //  //  //
//      UTILs
//  //  //  //  //  //  //  //
fn try_get_integer( arr: &Vec<Value>, index: usize ) -> Result< i32, Box<dyn Error> > {
    match arr.get(index) {
        Some( Value::Integer(item) ) => {
            return Ok( *item as i32 );
        },
        _ => {
            return Err( Box::from("<try_get_integer>: unable parse toml to midi") );
        },
    }
}
fn try_get_command( arr: &Vec<Value>, index: usize ) -> Result< i32, Box<dyn Error> > {
    match arr.get(index) {
        Some( Value::String(s) ) => {
            match s.as_str() {
                "on" => {
                    return Ok( 0x90 );
                },
                "off" => {
                    return Ok( 0x80 );
                },
                _ => {
                    return Err( Box::from("<try_get_command>: unsupported command parse toml to midi") );
                },
            }
        },
        _ => {
            return Err( Box::from("<try_get_command>: unable parse toml to midi") );
        },
    }
}


//  //  //  //  //  //  //  //
//      TESTs
//  //  //  //  //  //  //  //
#[cfg(test)]
mod midi_from_toml {
    use super::*;
    use raalog::log;

    #[test]
    fn note_off() {
        let tml = r#"
                    notes = [ 3, 'off', 4, 5, 'some additional text', ]
                    "#
                    .parse::<Table>().unwrap();
        let mist;
        let val = tml.get("notes").unwrap();
        match MidiMessage::from( val ) {
            Ok(midi) => {
                if midi.channel != 3 {
                    mist = "incorrect channel";
                }else if midi.command != 0x80 {
                    mist = "incorrect command";
                }else if midi.data1 != 4 {
                    mist = "incorrect key";
                }else if midi.data2 != 5 {
                    mist = "incorrect velocity";
                }else{
                    mist = "";
                }
            },
            Err(e) => {
                mist = "has not to be Error";
                log::error(&e.to_string());
            },
        }
        assert!( mist == "", ">> {mist} <<");
    }
    #[test]
    fn note_on() {
        let tml = r#"
                    notes = [ 1, 'on', 66, 80, 'some additional text', ]
                    "#
                    .parse::<Table>().unwrap();
        let mist;
        let val = tml.get("notes").unwrap();
        match MidiMessage::from( val ) {
            Ok(midi) => {
                if midi.channel != 1 {
                    mist = "incorrect channel";
                }else if midi.command != 0x90 {
                    mist = "incorrect command";
                }else if midi.data1 != 66 {
                    mist = "incorrect key";
                }else if midi.data2 != 80 {
                    mist = "incorrect velocity";
                }else{
                    mist = "";
                }
            },
            Err(e) => {
                mist = "has not to be Error";
                log::error(&e.to_string());
            },
        }
        assert!( mist == "", ">> {mist} <<");
    }
    #[test]
    fn wrong_type() {
        let tml = r#"
                    notes = { city = 3}
                    "#
                    .parse::<Table>().unwrap();
        let mist;
        let val = tml.get("notes").unwrap();
        match MidiMessage::from( val ) {
            Ok(_midi) => {
                mist = "have to be Error";
            },
            Err(e) => {
                mist = "";
                log::error(&e.to_string());
            },
        }
        assert!( mist == "", ">> {mist} <<");
    }
    #[test]
    fn empty() {
        let tml = r#"
                    notes = [ ]
                    "#
                    .parse::<Table>().unwrap();
        let mist;
        let val = tml.get("notes").unwrap();
        match MidiMessage::from( val ) {
            Ok(_midi) => {
                mist = "have to be Error";
            },
            Err(e) => {
                mist = "";
                log::error(&e.to_string());
            },
        }
        assert!( mist == "", ">> {mist} <<");
    }
}

