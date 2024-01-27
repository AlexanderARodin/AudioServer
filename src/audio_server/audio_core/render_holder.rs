use std::sync::{Arc,Mutex};

use raalog::log;


pub trait AudioRender: Sync + Send {
    fn render(&mut self, left: &mut [f32], right: &mut [f32]);
}

//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //
pub(crate) struct RenderHolder {
    //pub(crate) time_increment: f32,
    pub(crate) audio_render: Option< Arc<Mutex<dyn AudioRender>> >,
}

impl RenderHolder {
    pub fn new_arc_mutex() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new( Self::new() ))
    }
    pub fn new() -> Self {
        log::creating("RenderHolder");
        Self{ 
            audio_render: None
        }
    }
}
impl Drop for RenderHolder {
    fn drop(&mut self) {
        log::droping("RenderHolder");
    }
}

//  //  //  //  //  //  //  //
//      MAIN interface
//  //  //  //  //  //  //  //
impl RenderHolder {
    pub(crate) fn render(&mut self, left: &mut [f32], right: &mut [f32]) {
        //log::tick();

        match &self.audio_render {
            None => {
                fill_silence(left);
                fill_silence(right);
            },
            Some(render) => {
                let mut locked_render = render.lock()
                    .expect("FATAL: can't lock AudioRender!");
                locked_render.render(left, right);
            }
        }
    }
}

//  //  //  //  //  //  //  //
//      UTIL
//  //  //  //  //  //  //  //
fn fill_silence(buf: &mut [f32]) {
    for sample in buf {
        *sample = 0_f32;
   }
}

