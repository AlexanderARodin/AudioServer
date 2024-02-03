use std::error::Error;
use raalog::log;



pub struct OutputDeviceParameters {
    pub sample_rate: usize,
    pub channels_count: usize,
    pub channel_sample_count: usize,
}

pub(crate) mod prelude {
    use super::*;

    pub trait BaseAudioOutputDevice: Send { }
    pub fn run_output_device<C>(    _params: OutputDeviceParameters,
                                    _data_callback: C
                                ) -> Result<Box<dyn BaseAudioOutputDevice>, Box<dyn Error>> where C: FnMut(&mut [f32]) + Send + 'static,
    {
        log::info("DummyAudio::<run_output_device> <-----");
        Ok( Box::new( DummyAudio::new() ) )
    }
}


struct DummyAudio {}
impl DummyAudio {
    fn new() -> Self {
        log::creating("DummyAudio <-----");
        Self{}
    }
}
impl prelude::BaseAudioOutputDevice for DummyAudio {}
impl Drop for DummyAudio {
    fn drop(&mut self) {
        log::droping("DummyAudio <-----")
    }
}
