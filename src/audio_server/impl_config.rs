use toml::{ Table };
use crate::prelude::*;

//    use super::uni_source_variant::{ UniSourceVariant };
    use super::uni_source_variant::{ UniSourceVariant::* };


//  //  //  //  //  //  //  //
//      config impl
//  //  //  //  //  //  //  //
use super::AudioServer;

impl AudioServer {

    pub(crate) fn invoke_core_config_loading(&mut self, tbl: &Table, sf_array: &Vec<&'static [u8]> ) -> ResultOf< () > {
        self.sf_array = sf_array.clone();
        self.core_config = tbl.clone();
        return self.invoke_core_exec( "autoexec" );
    }

    //  //  //  //  //  //  //
    pub(crate) fn install_source_to_audio(&mut self) {
        match &self.uni_source {
            Silence => {
                self.audio_core.install_render(None);
            },
            Audio(wrapped_audio_render) => {
                self.audio_core.install_render(Some( wrapped_audio_render.clone() ));
            },
            Simple(simsyn) => {
                self.audio_core.install_render(Some( simsyn.clone() ));
            },
            Rusty(ryssyn) => {
                self.audio_core.install_render(Some( ryssyn.clone() ));
            },
            Sequencer(sequencer) => {
                self.audio_core.install_render(Some( sequencer.clone() ));
            },
        }
    }
}

