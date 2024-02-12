use toml::{ Table };
use crate::prelude::*;



//  //  //  //  //  //  //  //
//      config impl
//  //  //  //  //  //  //  //
use super::AudioServer;

impl AudioServer {

    //  //  //  //  //  //  //
    pub(super) fn invoke_core_config_loading(&mut self, tbl: &Table, sf_array: &Vec<&'static [u8]> ) -> ResultOf< () > {
        self.sf_array = sf_array.clone();
        self.core_config = tbl.clone();
        return self.invoke_exec_core( "autoexec" );
    }

    //  //  //  //  //  //  //
    pub(super) fn invoke_ordinary_loading(&mut self, tbl: &Table ) -> ResultOf< () > {
        self.ordinary_config = tbl.clone();
        Ok(())
    }
}

