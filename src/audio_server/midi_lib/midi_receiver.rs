pub trait MidiReceiver {
    fn reset(&mut self);
    fn process_midi_command(&mut self, channel: i32, command: i32, data1: i32, data2: i32);
}
