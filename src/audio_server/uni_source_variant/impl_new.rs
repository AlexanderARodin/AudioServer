use crate::prelude::*;

    use super::super::synths::rusty_synth_wrapper::RustySynthWrapper;
    use super::super::synths::simple_synth::SimpleSynth;
    use super::super::midi_sequencer::MidiSequencer;


//  //  //  //  //  //  //  //
//      new impl
//  //  //  //  //  //  //  //
use super::UniSourceVariant;

//  //  //  //  //  //  //  //
//      FROM simple str
//  //  //  //  //  //  //  //
impl UniSourceVariant {

    pub(crate) fn new_from_str( s: &str, sample_rate: &usize) -> ResultOf<Self> {
        match s as &str {
            "None" => {
                return Ok( Self::Silence );
            },
            "Simple" => {
                return Ok( Self::Simple( Self::create_simple(&sample_rate)?) );
            },
            _ => {
                let msg = format!( "<UniSourceVariant::new(...)>: there is no preset <AudioSource={s}>" );
                return Err( Box::from( msg.as_str() ) );
            },
        }
    }

    fn create_simple(sample_rate: &usize ) -> ResultOf< ArcMut<SimpleSynth> > {
        let synth = SimpleSynth::new(sample_rate);
        let arcmut_wrapper = new_arcmut(synth);
        return Ok( arcmut_wrapper );
    }
}


//  //  //  //  //  //  //  //
//      FROM with_nested
//  //  //  //  //  //  //  //
impl UniSourceVariant {

    pub(crate) fn new_from_withnested( key: &str, nested_item: &call_list::CallItem, sample_rate: &usize, time_increment: &f32, sf_list: &Vec<&[u8]> ) -> ResultOf<Self> {
        match key as &str {
            "Rusty" => {
                if let call_list::CallItem::Simple(sf_name) = nested_item {
                    let index:usize = sf_name.parse()?;
                    let data = sf_list.get(index).ok_or("unknow SoundFont")?;
                    return Ok( Self::Rusty( Self::create_rustysynth(sample_rate, data)? ) );
                }else{
                    let msg = format!( "<UniSourceVariant::new(...)>: invalid SoundFont <AudioSource={{Rusty=...}}>" );
                    return Err( Box::from( msg.as_str() ) );
                }
            },
            "Sequencer" => {
                return Ok( Self::Sequencer( Self::create_sequencer(nested_item, &sample_rate, &time_increment, sf_list)? ) );
            },
            _ => {
                let msg = format!( "<UniSourceVariant::new(...)>: there is no preset <AudioSource={{{key}=...}}>" );
                return Err( Box::from( msg.as_str() ) );
            },
        }
    }

    fn create_rustysynth(sample_rate: &usize, mut data: &[u8] ) -> ResultOf< ArcMut<RustySynthWrapper> > {
        let ryssyn = RustySynthWrapper::new( &sample_rate, &mut data )?;
        let arcmut_wrapper = new_arcmut( ryssyn );
        return Ok( arcmut_wrapper );
    }
}


//  //  //  //  //  //  //  //
//      FROM simple str
//  //  //  //  //  //  //  //
impl UniSourceVariant {

    pub(crate) fn create_sequencer( nested_item: &call_list::CallItem, sample_rate: &usize, time_increment: &f32, sf_list: &Vec<&[u8]> ) -> ResultOf< ArcMut<MidiSequencer> > {
        let mut sequencer = MidiSequencer::new(*time_increment);
        match nested_item {
            call_list::CallItem::Simple(s) => {
                let newone = Self::new_from_str( s, &sample_rate )?;
                match newone {
                    Self::Silence => {
                        sequencer.install_synth( None );
                    },
                    Self::Simple(simsyn) => {
                        sequencer.install_synth( Some(simsyn) );
                    },
                    _ => {
                        let msg = format!( "<UniSourceVariant::new(...)>: invalid Sequencer synth <{s}>" );
                        return Err( Box::from( msg.as_str() ) );
                    },
                }
            },
            call_list::CallItem::WithNested( key, item ) => {
                let newone = Self::new_from_withnested(key, item, &sample_rate, &time_increment, sf_list )?;
                match newone {
                    Self::Rusty(russyn) => {
                        sequencer.install_synth( Some(russyn) );
                    },
                    _ => {
                        let msg = format!( "<UniSourceVariant::new(...)>: invalid Sequencer synth <{key}>" );
                        return Err( Box::from( msg.as_str() ) );
                    },
                }
            },
        }
        let sequencer_wrapper = new_arcmut( sequencer );
        return Ok( sequencer_wrapper );
    }
}

