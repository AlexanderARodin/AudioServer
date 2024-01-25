
pub struct MidiGeneral {
    pub channel: i32,
    pub command: i32,
    pub data1: i32,
    pub data2: i32
}
impl Clone for MidiGeneral {
    fn clone(&self) -> Self {
        Self {
            channel: self.channel,
            command: self.command,
            data1: self.data1,
            data2: self.data2
        }
    }
}

pub enum MidiMessage {
  General( MidiGeneral ), // channel, command, data1, data2
  NoteOn( i32, i32, i32 ), // channel, 0x90, key, velocity
  NoteOff( i32, i32, i32 ), // channel, 0x80, key, velocity
}

impl MidiMessage {
    #[allow(dead_code)]
    pub fn new(channel:i32, command:i32, data1:i32, data2:i32) -> Self {
        Self::General( MidiGeneral {
            channel,
            command,
            data1,
            data2
        })
    }
    pub fn from_midi_general( midi_general: &MidiGeneral ) -> Self {
        Self::General( midi_general.clone() )
    }
    #[allow(dead_code)]
    pub fn to_general(&self) -> Self {
        let midi_general = self.to_midi_general();
        Self::from_midi_general( &midi_general )
    }

    pub fn to_midi_general(&self) -> MidiGeneral {
        match self {
            Self::General( midi_general ) => {
                midi_general.clone()
            },
            Self::NoteOn( channel, key, velocity ) => {
                MidiGeneral {
                    channel: *channel,
                    command: 0x90,
                    data1: *key,
                    data2: *velocity
                }
            },
            Self::NoteOff( channel, key, velocity) => {
                MidiGeneral {
                    channel: *channel,
                    command: 0x80,
                    data1: *key,
                    data2: *velocity
                }
            },
        }
    }

    pub fn get_parsed(&self) -> Self {
        match self {
            Self::General( midi_general ) => {
                match midi_general.command {
                    0x80 => {
                        Self::NoteOff( midi_general.channel,
                                       midi_general.data1,
                                       midi_general.data2 )
                    },
                    0x90 => {
                        Self::NoteOn( midi_general.channel,
                                       midi_general.data1,
                                       midi_general.data2 )
                    },
                    _ => {
                        Self::General( midi_general.clone() )
                    }
                }
            },
            Self::NoteOn( channel, key, velocity ) => {
                Self::NoteOn( *channel, *key, *velocity )
            },
            Self::NoteOff( channel, key, velocity) => {
                Self::NoteOff( *channel, *key, *velocity)
            },
        }
    }

}

impl Clone for MidiMessage {
    fn clone(&self) -> Self {
        let midi_general = self.to_midi_general();
        MidiMessage::from_midi_general(&midi_general)
            .get_parsed()
    }
}






//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod test{
    use super::MidiMessage;

    #[test]
    fn note_on_2general() {
        let src_midi_msg = MidiMessage::NoteOn(1, 2, 3 );
        let dst_midi_msg = src_midi_msg.to_general();
        match dst_midi_msg {
            MidiMessage::General( midi ) => {
                assert!( midi.channel == 1, "wrong channel" );
                assert!( midi.command == 0x90, "wrong command" );
                assert!( midi.data1 == 2, "wrong key" );
                assert!( midi.data2 == 3, "wrong velocity" );
            },
            _ => {
                assert!(false, "incorrect conversion");
            }
        }
    }
    #[test]
    fn note_off_2general() {
        let src_midi_msg = MidiMessage::NoteOff(1, 2, 3);
        let dst_midi_msg = src_midi_msg.to_general();
        match dst_midi_msg {
            MidiMessage::General( midi ) => {
                assert!( midi.channel == 1, "wrong channel" );
                assert!( midi.command == 0x80, "wrong command" );
                assert!( midi.data1 == 2, "wrong key" );
                assert!( midi.data2 == 3, "wrong velocity" );
            },
            _ => {
                assert!(false, "incorrect conversion");
            }
        }
    }
    #[test]
    fn general_2general() {
        let src_midi_msg = MidiMessage::new(1, 2, 3, 4);
        let dst_midi_msg = src_midi_msg.to_general();
        match dst_midi_msg {
            MidiMessage::General( midi ) => {
                assert!( midi.channel == 1, "wrong channel" );
                assert!( midi.command == 2, "wrong command" );
                assert!( midi.data1 == 3, "wrong data1" );
                assert!( midi.data2 == 4, "wrong data2" );
            },
            _ => {
                assert!(false, "incorrect conversion");
            }
        }
    }

    #[test]
    fn parse_note_on() {
        let src_midi_msg = MidiMessage::NoteOn( 1, 2, 3);
        let dst_midi_msg = src_midi_msg.get_parsed();
        match dst_midi_msg {
            MidiMessage::NoteOn( channel, data1, data2 ) => {
                assert!( channel == 1, "wrong channel" );
                assert!( data1 == 2, "wrong key" );
                assert!( data2 == 3, "wrong velocity" );
            },
            _ => {
                assert!(false, "incorrect conversion");
            }
        }
    }
    #[test]
    fn parse_note_off() {
        let src_midi_msg = MidiMessage::NoteOff( 1, 2, 3 );
        let dst_midi_msg = src_midi_msg.get_parsed();
        match dst_midi_msg {
            MidiMessage::NoteOff( channel, data1, data2 ) => {
                assert!( channel == 1, "wrong channel" );
                assert!( data1 == 2, "wrong key" );
                assert!( data2 == 3, "wrong velocity" );
            },
            _ => {
                assert!(false, "incorrect conversion");
            }
        }
    }
    #[test]
    fn parse_general_2note_off() {
        let src_midi_msg = MidiMessage::new( 1, 0x80, 3, 4);
        let dst_midi_msg = src_midi_msg.get_parsed();
        match dst_midi_msg {
            MidiMessage::NoteOff( channel, data1, data2 ) => {
                assert!( channel == 1, "wrong channel" );
                assert!( data1 == 3, "wrong key" );
                assert!( data2 == 4, "wrong velocity" );
            },
            _ => {
                assert!(false, "incorrect conversion");
            }
        }
    }
    #[test]
    fn parse_general_2note_on() {
        let src_midi_msg = MidiMessage::new( 1, 0x90, 3, 4);
        let dst_midi_msg = src_midi_msg.get_parsed();
        match dst_midi_msg {
            MidiMessage::NoteOn( channel, data1, data2 ) => {
                assert!( channel == 1, "wrong channel" );
                assert!( data1 == 3, "wrong key" );
                assert!( data2 == 4, "wrong velocity" );
            },
            _ => {
                assert!(false, "incorrect conversion");
            }
        }
    }

}

