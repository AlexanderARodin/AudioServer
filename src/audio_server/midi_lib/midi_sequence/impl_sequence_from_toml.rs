use std::error::Error;
use toml::{Value};


use crate::audio_server::midi_lib::MidiMessage;

//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //
use super::MidiSequence;

impl MidiSequence {

    pub fn from_toml_value( tml: &toml::Value, transpose: i32, speed: f32 ) -> Result< Self, Box<dyn Error> > {
        let mut seq = Self::new();
        let mut time_offset = 0_f32;
        match tml {
            Value::Array(arr) => {
                for item in arr {
                    match item {
                        Value::Float(t) => {
                            time_offset += *t as f32;
                        },
                        Value::Array(arr) => {
                            let mut midi = MidiMessage::from(arr)?;
                            midi.data1 += transpose;
                            seq.push( time_offset / speed, &midi );
                            time_offset = 0.0;
                        },
                        _ => {
                            return Err( Box::from("<MidiSequence::from>: invalid element in array") );
                        },
                    }
                }
                return Ok(seq);
            },
            _ => {
                return Err( Box::from("<MidiSequence::from>: argument is not an array") );
            },
        }
    }
}

