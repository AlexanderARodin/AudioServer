use crate::prelude::*;
use toml::{ Value, value::Array };


use crate::audio_server::midi_lib::MidiMessage;

//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //
use super::MidiSequence;

impl MidiSequence {

    pub fn from_toml_array( tml_array: &Array, transpose: &i32, speed: &f32 ) -> ResultOf< Self > {
        let mut seq = Self::new();
        let mut time_offset = 0_f32;

        for item in tml_array {
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
                Value::Table( _ ) => {
                    let msg = format!( "<<MidiSequence::from_toml_array>: midi via Table not supported yet" );
                    return Err( Box::from( msg.as_str() ) );
                },
                _ => {
                    let msg = format!( "<<MidiSequence::from_toml_array>: invalid element in array" );
                    return Err( Box::from( msg.as_str() ) );
                },
            }
        }
        return Ok( seq );
    }
}

