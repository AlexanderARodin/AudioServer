use std::error::Error;
use toml::{ Table, Value };
use raalog::log;

    use super::midi_lib::MidiSequence;

//  //  //  //  //  //  //  //
//      sequence config impl
//  //  //  //  //  //  //  //
use super::AudioServer;

impl AudioServer {
    pub(crate) fn create_midi_sequence( seq_val: &Value) -> Result<MidiSequence, Box<dyn Error>> {
    let transpose = 0;
    let speed = 1_f32;
        if let Value::Table(seq_tbl) = seq_val {
            if let Some(main_seq) = extract_sequence( "Main", seq_tbl, transpose, speed )? {
                return Ok( main_seq );
            }else{
                return Err(Box::from("has to be Main array"));
            }
        }else{
            return Err(Box::from("invalid section Sequence"));
        }
    }
}

fn extract_sequence( name: &str, seq_tbl: &Table, transpose: i32, speed: f32 ) -> Result< Option<MidiSequence>, Box<dyn Error> > {
    if let Some(notes_value) = seq_tbl.get(name) {
        if let Value::Array(notes_array) = notes_value {
            return Ok(Some( parse_notes(notes_array, transpose, speed)? ));
        }else{
            return Err(Box::from("notes array has incorrect type"));
        }
    }else{
        return Ok(None);
    }
}

fn parse_notes( array: &Vec<Value>, transpose: i32, speed: f32 ) -> Result< MidiSequence, Box<dyn Error> > {
    let mut seq = MidiSequence::new();
    let mut time_offset = 0_f32;
        for item in array {
            match item {
                Value::String(midi_str) => {
                    if let Some(midi_src) = super::impl_exec::interpret_as_midi(midi_str)? {
                        let mut midi = midi_src;
                        midi.data1 += transpose;
                        seq.push( speed*time_offset, &midi );
                        time_offset = 0.0;
                    }else{
                        log::info( "empty note in array?" );
                    }
                },
                Value::Float(curr_offset) => {
                    time_offset += *curr_offset as f32;
                },
                _ => {
                    return Err(Box::from("incorrect array item"));
                },
            }
        }
    return Ok(seq);
}

