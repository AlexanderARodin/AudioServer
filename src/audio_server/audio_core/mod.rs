use std::error::Error;
use std::sync::{Arc,Mutex};

use tinyaudio::OutputDeviceParameters;
use tinyaudio::prelude::{BaseAudioOutputDevice,run_output_device};

use raalog::log;

//  //  //  //  //  //  //  //
mod render_holder;
use render_holder::RenderHolder;
pub use render_holder::AudioRender as AudioRender;
mod audio_core_parameters;
use audio_core_parameters::AudioCoreParameters;

//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //
pub struct AudioCore {
    params: AudioCoreParameters,
    device: Option< Box<dyn BaseAudioOutputDevice> >,
    render_holder: Arc<Mutex<RenderHolder>>,
}

impl AudioCore {
    pub fn new( ) -> Self {
        log::creating("AudioCore");
        Self{ 
            params: Default::default(),
            device: None,
            render_holder: RenderHolder::new_arc_mutex(),
        }
    }
}
impl Drop for AudioCore {
    fn drop(&mut self) {
        if self.is_active() {
            self.stop();
        }
        log::droping("AudioCore");
    }
}

//  //  //  //  //  //  //  //
//      pub MAIN
//  //  //  //  //  //  //  //
impl AudioCore {
    pub fn start(&mut self) -> Result< (), Box<dyn Error> > {
        if self.is_active() {
            self.stop();
            #[cfg(test)]
            log::info("AudioCore: restarting");
        }else{
            #[cfg(test)]
            log::info("AudioCore: starting");
        }
        self.activate_device_loop()
    }
    pub fn stop(&mut self) {
        self.device = None;
        #[cfg(test)]
        log::info("AudioCore: stop");
    }
    pub fn is_active(&self) -> bool {
        match self.device {
            None => false,
            _ => true
        }
    }

    pub fn get_sample_rate(&self) -> usize {
        self.params.sample_rate
    }
    pub fn get_time_increment(&self) -> f32 {
        self.params.get_tick_time()
    }
    
    pub fn install_render(&mut self, new_render: Option<Arc<Mutex<dyn AudioRender>>>) {
        let mut locked_holder = self.render_holder.lock()
            .expect("can't lock RenderHolder");
        locked_holder.audio_render = new_render;
    }
}

//  //  //  //  //  //  //  //
//      PRIVATE lvl0
//  //  //  //  //  //  //  //
impl AudioCore {

//    fn refresh_tick_time(&self) {
//        let mut locked_holder = self.render_holder.lock()
//            .expect("panic on lockin holder_lock");
//        locked_holder.tick_time = self.params.get_tick_time();
//    }

    fn activate_device_loop(&mut self) -> Result< (), Box<dyn Error>> {
        // prepare parameters
        let params = self.params.get_output_device_parameters();
        let render_holder_clone = self.render_holder.clone();
        let block_chunk = 2*self.params.block_size;
        // try to start device loop
        let device = invoke_run_output_device( params,
                                             render_holder_clone,
                                             block_chunk,
                                             self.params.block_size );
        // check result for errors
        match device {
            Err(e) => {
                let errmsg = format!("{:?}",e);
                log::error( &format!("AudioCore: {errmsg}") );
                return Err(e)
            },
            Ok(running_device) => self.device = Some(running_device),
        }
        Ok(())
    }
}

//  //  //  //  //  //  //  //
//      PRIVATE lvl1
//  //  //  //  //  //  //  //
fn invoke_run_output_device( params: OutputDeviceParameters,
                           render_holder_clone: Arc<Mutex<RenderHolder>>,
                           block_chunk: usize,
                           block_size: usize ) -> Result< Box<dyn BaseAudioOutputDevice>, Box<dyn Error> > {
        
        run_output_device( params, {
            let mut left :Vec<f32> = vec![ 0_f32; block_size ];
            let mut right:Vec<f32> = vec![ 0_f32; block_size ];
            //
            move |data: &mut [f32]| {
                let mut locked_holder = render_holder_clone.lock()
                    .expect("panic on locking render_holder_lock");
                for chunk in data.chunks_mut(block_chunk) {
                    locked_holder.render( &mut left, &mut right );
                    for (i, l_sample) in left.iter().enumerate() {
                        chunk[i*2    ] = *l_sample;
                        chunk[i*2 + 1] =  right[i];
                    }
                }
            }
        })
}


//  //  //  //  //  //  //  //
//          TESTS
//  //  //  //  //  //  //  //
#[cfg(test)]
mod native_only {
    use super::*;

    #[test]
    #[ignore]
    fn create_inactive() {
        let audio = AudioCore::new();
        assert!(!audio.is_active());
    }
    #[test]
    #[ignore]
    fn start_active() {
        let mut audio = AudioCore::new();
        let _ = audio.start();
        assert!(audio.is_active());
    }
    #[test]
    #[ignore]
    fn start_stop() {
        let mut audio = AudioCore::new();
        let _ = audio.start();
        assert!(audio.is_active());
        audio.stop();
        assert!(!audio.is_active());
    }
}

