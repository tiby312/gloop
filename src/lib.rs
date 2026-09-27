use gloo::events::EventListener;
use web_sys::{Event, EventTarget};

pub struct EventHandler {
    _a: EventListener,
    back: futures::channel::mpsc::UnboundedReceiver<(Event, &'static str)>,
}

impl EventHandler {
    pub async fn recv(&mut self) -> (Event, &'static str) {
        match self.back.recv().await {
            Ok((e, s)) => (e, s),
            Err(e) => panic!("failed {:?}", e),
        }
    }
}

pub fn register_event<F: FnMut(&Event)>(
    target: &EventTarget,
    event_type: &'static str,
) -> EventHandler {
    let (tx, rx) = futures::channel::mpsc::unbounded();

    let e = EventListener::new(target, event_type, move |e| {
        tx.unbounded_send((e.clone(), event_type)).unwrap();
    });

    EventHandler { _a: e, back: rx }
}
