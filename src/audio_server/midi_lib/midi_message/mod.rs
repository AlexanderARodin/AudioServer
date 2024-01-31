

//  //  //  //  //  //  //  //
//      CORE
//  //  //  //  //  //  //  //
mod impl_from_toml;

#[derive(Clone)]
pub struct MidiMessage {
    pub channel: i32,
    pub command: i32,
    pub data1: i32,
    pub data2: i32
}

#[allow(dead_code)]
impl MidiMessage {

    pub fn new( channel: i32, command: i32, data1: i32, data2: i32 ) -> Self {
        Self{
            channel,
            command,
            data1,
            data2,
        }
    }

    pub fn note_on( channel: i32, key: i32, velocity: i32 ) -> Self {
        Self{
            channel: channel,
            command: 0x90,
            data1:   key,
            data2:   velocity,
        }
    }

    pub fn note_off( channel: i32, key: i32, velocity: i32 ) -> Self {
        Self{
            channel: channel,
            command: 0x80,
            data1:   key,
            data2:   velocity,
        }
    }
}



//  //  //  //  //  //  //  //
//      TESTs
//  //  //  //  //  //  //  //
#[cfg(test)]
mod midi_message {
    use super::*;

    #[test]
    fn note_on() {
        let midi = MidiMessage::note_on(1, 2, 3 );
        assert!( midi.channel == 1, "wrong channel" );
        assert!( midi.command == 0x90, "wrong command" );
        assert!( midi.data1 == 2, "wrong key" );
        assert!( midi.data2 == 3, "wrong velocity" );
    }
    #[test]
    fn note_off() {
        let midi = MidiMessage::note_off(1, 2, 3 );
        assert!( midi.channel == 1, "wrong channel" );
        assert!( midi.command == 0x80, "wrong command" );
        assert!( midi.data1 == 2, "wrong key" );
        assert!( midi.data2 == 3, "wrong velocity" );
    }
}

