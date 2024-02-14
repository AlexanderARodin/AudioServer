
use audio_server::{ AudioServer, Config, Exec };
use audio_server::ResultOf;


#[test]
fn ordinary_basic() -> ResultOf< () > {
    let mut au = AudioServer::new();
    let core_cfg = r#"
        [workflows]
        autoexec = [
            'stop',
            { AudioSource = { Sequencer = 'Simple' } },
            'start',
        ]
        "#;
    let core_setup = Config::CoreConfigFromStr( core_cfg, Vec::new() );
    au.load_config( &core_setup )?;
    //
    let config = r#"
        [workflows]
        seq-A = [
            { load = 'seq-1.notes' },
            {  transpose = { 4 = { load = 'seq-1.notes' } } },
            {  speed = { 4 = { load = 'seq-1.notes' } } },
            'play',
        ]
        [seq-1]
        notes = [ 
                 [1  , 'on',  90, 80  ],
            0.5, [1  , 'off', 90, 80  ],
                 [1  , 'on',  91, 80  ],
            0.5, [1  , 'off', 91, 80  ],
                 [1  , 'on',  92, 80  ],
            0.5, [1  , 'off', 92, 80  ],
                 [1  , 'on',  91, 80  ],
            0.5, [1  , 'off', 91, 80  ],
            1.0, [1  , 'off', 92, 80  ],
        ]
        transpose = 7
        speed = 2.0
    "#;
    let setup = Config::OrdinaryConfigFromStr( config );
    au.load_config( &setup )?;
    let cmd = Exec::OrdinaryExec( "seq-A" );
    au.exec(&cmd)?;
    Ok(())
}

#[test]
fn core_basic() -> ResultOf< () > {
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

