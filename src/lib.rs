


use gloo::events::EventListener;
use web_sys::{Event, EventTarget};
use std::borrow::Cow;


pub trait Listen{
    fn call(&mut self,a:&Event);
}

impl<F:FnMut(&Event)> Listen for F{
    fn call(&mut self,a:&Event){
        (*self)(a)
    }
}

pub struct EventListenerWrapper<F>{
    _a:EventListener,
    func:std::marker::PhantomData<F>
}


impl<F: Listen> EventListenerWrapper<F>{
    pub fn new<S>(target: &EventTarget, event_type: S, func: F) -> Self
where
    S: Into<Cow<'static, str>>,
    {
        let callback:Box<dyn Listen>=Box::new(func);
        let j=Box::into_raw(callback);
        let j:*mut (dyn Listen+'static)=j as *mut _;

        
        let mut callback:Box<dyn Listen+'static>=unsafe{Box::from_raw(j)};

        let e=EventListener::new(target,event_type,move|e|callback.call(e));

        EventListenerWrapper { _a: e, func:std::marker::PhantomData }
    }
}


