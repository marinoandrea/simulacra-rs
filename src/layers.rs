use crate::events::EventHandler;

pub trait Layer: EventHandler {
    fn on_attach(&mut self);
    fn on_detach(&mut self);
}
