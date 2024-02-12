use raalog::log;


    use super::super::midi_lib::{MidiReceiver,MidiMessage};


//  //  //  //  //  //  //  //
//      new impl
//  //  //  //  //  //  //  //
use super::UniSourceVariant;
use UniSourceVariant::*;

impl UniSourceVariant {

    pub(crate) fn send_to_synth(&mut self, midi: &MidiMessage) {
        match &self {
            Silence => {
                return
            },
            Simple( simsyn ) => {
                let mut locked_receiver = simsyn.lock()
                    .expect("panick on locking UniSourceVariant::Simple( simsyn )");
                locked_receiver.process_midi_command( midi.channel, 
                                                      midi.command, 
                                                      midi.data1, 
                                                      midi.data2 );
            },
            Rusty( ryssyn ) => {
                let mut locked_receiver = ryssyn.lock()
                    .expect("panick on locking UniSourceVariant::Rusty( ryssyn )");
                locked_receiver.process_midi_command( midi.channel, 
                                                      midi.command, 
                                                      midi.data1, 
                                                      midi.data2 );
            },
            Sequencer( sequencer ) => {
                let mut locked_sequencer = sequencer.lock()
                    .expect("panick on locking UniSourceVariant::Sequencer( sequencer )");
                locked_sequencer.send_to_synth(midi);
            },
            _ => {
                log::error("----> outstanding");
            }
        }
    }
}

