

pub enum Event {
    UartTx(u8),
    UartRx(u8),
}


pub trait EnvModel {
    fn handle_event(evt: &mut Event);
}