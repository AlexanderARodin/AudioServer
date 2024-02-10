
use audio_server::{ AudioServer, Config };
use audio_server::ResultOf;


#[test]
fn basic() -> ResultOf< () > {
    let mut au = AudioServer::new();
    let config = r#"
        [workflows]
        autoexec = [
            'stop',
            { AudioSource = 'None' },
            { AudioSource = 'Simple' },
            #{ AudioSource = { Rusty = '0' } },
            { AudioSource = { Sequencer = 'None' } },
            { AudioSource = { Sequencer = 'Simple' } },
            #{ AudioSource = { Sequencer = { Rusty = '0' } } },
            'start',
        ]
        "#;
    let setup = Config::CoreConfigFromStr( config, Vec::new() );
    au.load_config( &setup )?;
    Ok(())
}

