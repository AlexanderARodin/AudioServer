use crate::prelude::*;


    use super::audio_core::AudioRender;
    use super::synths::simple_synth::SimpleSynth;
    use super::synths::rusty_synth_wrapper::RustySynthWrapper;
    use super::midi_sequencer::MidiSequencer;



//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //
mod impl_new;
mod impl_send_to_synth;

pub(crate) enum UniSourceVariant {
    Silence,
    #[allow(dead_code)]
    Audio( ArcMut<dyn AudioRender> ),
    Simple( ArcMut<SimpleSynth> ),
    Rusty( ArcMut<RustySynthWrapper> ),
    Sequencer( ArcMut<MidiSequencer> ),
}

impl UniSourceVariant {

    pub(crate) fn new( nested_item: &call_list::CallItem, sample_rate: &usize, time_increment: &f32, sf_list: &Vec<&[u8]> ) -> ResultOf<Self> {
        match nested_item {
            call_list::CallItem::Simple( s ) => {
                return Self::new_from_str( s as &str, &sample_rate );
            },
            call_list::CallItem::WithNested( key, sub_item ) => {
                return Self::new_from_withnested( key as &str, &sub_item, &sample_rate, &time_increment, sf_list );
            },
        }
    }
}


