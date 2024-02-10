use toml::{ Table, Value };

    use super::midi_lib::MidiSequence;


//  //  //  //  //  //  //  //
//      sequence config impl
//  //  //  //  //  //  //  //
use super::AudioServer;
use super::ResultOf;

impl AudioServer {
    pub(crate) fn create_midi_sequence( seq_val: &Value) -> ResultOf<MidiSequence> {
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

fn extract_sequence( name: &str, seq_tbl: &Table, transpose: i32, speed: f32 ) -> ResultOf< Option<MidiSequence> > {
    if let Some(notes_value) = seq_tbl.get(name) {
        return Ok(Some( MidiSequence::from_toml_value(notes_value, transpose, speed)? ));
    }else{
        return Ok(None);
    }
}
fn extract_speed_or_default( seq_tbl: &Table ) -> ResultOf< f32 > {
    match seq_tbl.get( "speed" ) {
        None => return Ok( 1.0 ),
        Some(Value::Float(speed)) => Ok( *speed as f32 ),
        _ => Err( Box::from("incorrect type of <speed>") ),
    }
}
fn extract_transpose_or_default( seq_tbl: &Table ) -> ResultOf< i32 > {
    match seq_tbl.get( "transpose" ) {
        None => return Ok( 0 ),
        Some(Value::Integer(i)) => Ok( *i as i32 ),
        _ => Err( Box::from("incorrect type of <transpose>") ),
    }
}

