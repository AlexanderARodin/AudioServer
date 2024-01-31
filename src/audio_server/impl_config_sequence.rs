use std::error::Error;
use toml::{ Table, Value };

    use super::midi_lib::MidiSequence;

//  //  //  //  //  //  //  //
//      sequence config impl
//  //  //  //  //  //  //  //
use super::AudioServer;

impl AudioServer {
    pub(crate) fn create_midi_sequence( seq_val: &Value) -> Result<MidiSequence, Box<dyn Error>> {
        if let Value::Table(seq_tbl) = seq_val {
            let transpose = extract_transpose_or_default(seq_tbl)?;
            let speed = extract_speed_or_default(seq_tbl)?;
            if let Some(main_seq) = extract_sequence( "notes", seq_tbl, transpose, speed )? {
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
        return Ok(Some( MidiSequence::from_toml_value(notes_value, transpose, speed)? ));
    }else{
        return Ok(None);
    }
}
fn extract_speed_or_default( seq_tbl: &Table ) -> Result< f32, Box<dyn Error> > {
    match seq_tbl.get( "speed" ) {
        None => return Ok( 1.0 ),
        Some(Value::Float(speed)) => Ok( *speed as f32 ),
        _ => Err( Box::from("incorrect type of <speed>") ),
    }
}
fn extract_transpose_or_default( seq_tbl: &Table ) -> Result< i32, Box<dyn Error> > {
    match seq_tbl.get( "transpose" ) {
        None => return Ok( 0 ),
        Some(Value::Integer(i)) => Ok( *i as i32 ),
        _ => Err( Box::from("incorrect type of <transpose>") ),
    }
}

