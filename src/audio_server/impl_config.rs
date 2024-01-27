use std::error::Error;
use toml::{ Table, Value };
use raalog::log;

    use super::uni_source_variant::{ UniSourceVariant };
//    use super::uni_source_variant::{ UniSourceVariant::* };

//  //  //  //  //  //  //  //
//      config impl
//  //  //  //  //  //  //  //
use super::AudioServer;

impl AudioServer {
    pub(crate) fn invoke_config_parsing(&mut self, tbl: &Table, data: Option<&[u8]> ) -> Result<(), Box<dyn Error>> {
        let sample_rate = self.audio_core.get_sample_rate();
        let time_increment = self.audio_core.get_time_increment();

        if let Some(au_val) = tbl.get("AudioSource") {
            if let Value::Table(au_tbl) = au_val {
                self.uni_source = UniSourceVariant::new( &au_tbl, &sample_rate, time_increment, data )?;
                self.install_source_to_audio();
            }else{
                return Err(Box::from("invalid section AudioSource"));
            }
        }

        Ok(())
    }
}



//  //  //  //  //  //  //  //
//      TESTs
//  //  //  //  //  //  //  //
#[cfg(test)]
mod basic {
    use super::*;

    #[test]
    fn no_error() {
        let mut aud = AudioServer::new();
        let flag;
        if let Ok(()) = aud.config(TEST_CONFIG, None) {
            flag = true;
        }else{
            flag = false;
        }
        assert!(flag, "TEST_CONFIG is not valid");
    }
    #[test]
    fn is_error() {
        let mut aud = AudioServer::new();
        let flag;
        if let Err(_) = aud.config("]][{", None) {
            flag = true;
        }else{
            flag = false;
        }
        assert!(flag, "should be ERROR");
    }
}

#[cfg(test)]
static TEST_CONFIG: &str = r#"
anystring = 'any'
[AudioSource]
Main = 'None'
"#;

#[cfg(test)]
mod audio_source {
    use super::*;

    #[test]
    fn audio_source_error() {
        let mut aud = AudioServer::new();
        let flag;
        if let Err(e) = aud.config("AudioSource = 'err'", None) {
            flag = true;
            log::error(&e.to_string());
        }else{
            flag = false;
        }
        assert!(flag, "should be ERROR");
    }
    #[test]
    fn no_main() {
        let mut aud = AudioServer::new();
        let flag;
        if let Err(e) = aud.config("[AudioSource]", None) {
            flag = true;
            log::error(&e.to_string());
        }else{
            flag = false;
        }
        assert!(flag, "should be ERROR");
    }
    #[test]
    fn invalid_main() {
        let mut aud = AudioServer::new();
        let flag;
        if let Err(e) = aud.config("[AudioSource]\nMain = ['d']", None) {
            flag = true;
            log::error(&e.to_string());
        }else{
            flag = false;
        }
        assert!(flag, "should be ERROR");
    }
}

