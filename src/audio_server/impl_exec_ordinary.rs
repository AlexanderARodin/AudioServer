use crate::{prelude::*, audio_server::midi_lib::MidiSequence};



//  //  //  //  //  //  //  //
//      exec_ordinary impl
//  //  //  //  //  //  //  //
use super::AudioServer;

impl AudioServer {

    pub(super) fn invoke_exec_ordinary(&mut self, script_name: &str ) -> ResultOf< () > {
        let path = format!( "workflows.{script_name}" );
        let exec_list = call_list::from_toml_table( &self.ordinary_config, &path )?;
        for exec_item in exec_list {
            self.process_exec_ordinay_item( &exec_item )?;
        }
        Ok(())
    }

    fn process_exec_ordinay_item(&mut self, exec_item: &call_list::CallItem ) -> ResultOf< () > {
        match exec_item {
            call_list::CallItem::Simple( s ) => {
                return self.exec_ordinary_simple( s );
            },
            call_list::CallItem::WithNested( key, nested_item ) => {
                let transpose = 0;
                let speed = 1_f32;
                return self.exec_ordinary_withparam( key, nested_item, &transpose, &speed );
            },
        }
    }
}

//  //  //  //  //  //  //  //
impl AudioServer {

    fn exec_ordinary_simple(&mut self, cmd: &str ) -> ResultOf<()> {
        match cmd {
            "play"|"play-loop" => {
                match &self.uni_source {
                    crate::audio_server::UniSourceVariant::Sequencer( seqer ) => {
                        let mut locked_seqer = seqer.lock()
                                .expect("ERROR locking UniSourceVariant<Sequencer>");
                        if let Some( seq ) = &self.midi_sequence {
                            locked_seqer.set_midi_sequence( seq.clone(), cmd == "play-loop" );
                        }
                        return Ok(());
                    },
                    _ => {
                        let msg = format!( "<AudioServer.exec_ordinary_simple>: <{cmd}> requires Sequencer" );
                        return Err( Box::from( msg.as_str() ) );
                    },
                }
            },
            _ => {
                let msg = format!( "<AudioServer.exec_ordinary_simple>: unknown command <{cmd}>" );
                return Err( Box::from( msg.as_str() ) );
            },
        }
    }

    fn exec_ordinary_withparam(&mut self, key: &str, nested_item: &call_list::CallItem, transpose: &i32, speed: &f32 ) -> ResultOf<()> {
        match (key, nested_item) {
            ("load", call_list::CallItem::Simple(s)) => {
                return self.load_sequence_from( s, &transpose, &speed );
            },
            ("transpose", call_list::CallItem::WithNested( subkey, subitem )) => {
                return self.parse_transpose(subkey, subitem, &speed );
            },
            ("speed", call_list::CallItem::WithNested( subkey, subitem )) => {
                return self.parse_speed(subkey, subitem, &transpose);
            },
            _ => {
                let msg = format!( "<AudioServer.exec_ordinary_withparam>: unknown key <{key}>" );
                return Err( Box::from( msg.as_str() ) );
            },
        }
    }

    fn parse_speed(&mut self, key: &str, nested_item: &call_list::CallItem, transpose: &i32 ) -> ResultOf<()> {
        let speed = key.parse::<f32>()?;
        match nested_item {
            call_list::CallItem::WithNested( subkey, subitem ) => {
                return self.exec_ordinary_withparam( subkey, subitem, transpose, &speed)
            },
            _ => {
                let msg = format!( "<AudioServer.parse_transpose>: unknown " );
                return Err( Box::from( msg.as_str() ) );
            },
        }
    }
    fn parse_transpose(&mut self, key: &str, nested_item: &call_list::CallItem, speed: &f32 ) -> ResultOf<()> {
        let transpose = key.parse::<i32>()?;
        match nested_item {
            call_list::CallItem::WithNested( subkey, subitem ) => {
                return self.exec_ordinary_withparam( subkey, subitem, &transpose, speed)
            },
            _ => {
                let msg = format!( "<AudioServer.parse_transpose>: unknown " );
                return Err( Box::from( msg.as_str() ) );
            },
        }
    }

}

//  //  //  //  //  //  //  //
impl AudioServer {

    fn load_sequence_from(&mut self, path: &str, transpose: &i32, speed: &f32 ) -> ResultOf< () > {
        match call_list::get_value_by_path(&self.ordinary_config, path) {
            None => {
                let msg = format!( "<AudioServer.load_sequence_from>: invalid path <{path}>" );
                return Err( Box::from( msg.as_str() ) );
            },
            Some( toml::Value::Array( arr ) ) => {
                self.midi_sequence = Some( MidiSequence::from_toml_array(arr, &transpose, &speed)? );
                return Ok(());
            },
            Some(_) => {
                let msg = format!( "<AudioServer.load_sequence_from>: <{path}> must be an array" );
                return Err( Box::from( msg.as_str() ) );
            },
        }
    }
}




