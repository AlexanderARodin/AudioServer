use toml::{ Table, Value };
use crate::prelude::*;

    use super::uni_source_variant::{ UniSourceVariant };
    use super::uni_source_variant::{ UniSourceVariant::* };


//  //  //  //  //  //  //  //
//      config impl
//  //  //  //  //  //  //  //
use super::AudioServer;

impl AudioServer {

    pub(crate) fn invoke_core_exec(&mut self, script_name: &str ) -> ResultOf< () > {
        if let Some(exec_value) = self.core_config.get( script_name ) {
            let list = call_list::from_toml_value(exec_value)?;
            return Ok(());
        }else{
            let msg = format!( "<AudioServer.invoke_core_exec>: incorrect script name <{script_name}>" );
            return Err( Box::from( msg.as_str() ) );
        }
/*
        if let Some(exec_value) = self.core_config.get( "autoexec" ) {
            let list = call_list::from_toml_value(exec_value)?;
            for i in list {
                match i {
                    call_list::CallItem::Simple(cmd) => {
                        println!(" --> : {}", cmd );
                    },
                    call_list::CallItem::WithParam(cmd, param ) => {
                        println!(" --> : {}({})", cmd, param );
                    },
                }
            }
        }
        Ok(())
        */
    }



}



//  //  //  //  //  //  //  //
//      TESTs
//  //  //  //  //  //  //  //
#[cfg(test)]
mod no {
    use super::*;

    #[test]
    fn no() {
    }
}


