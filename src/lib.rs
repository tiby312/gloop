


use gloo::events::EventListener;
use web_sys::{Event, EventTarget};
use std::borrow::Cow;


pub trait Listen{
    fn call(&mut self,a:&Event);
}


impl<F:FnMut(&Event)> Listen for FnWrapper<F>{
    fn call(&mut self,a:&Event){
        (self.0)(a)
    }
}

pub struct FnWrapper<F>(F);



pub struct EventListen<F>{
    _a:EventListener,
    func:std::marker::PhantomData<F>
    
}

impl<F:FnMut(&Event)> EventListen<FnWrapper<F>>{

    pub fn from_closure<S>(target:&EventTarget,event_type:S,func:F)->EventListen<FnWrapper<F>>
     where S:Into<Cow<'static,str>>{
        EventListen::new(target,event_type,FnWrapper(func))
    }

    pub fn from_closure_with_options<S>(target:&EventTarget,event_type:S,options:gloo::events::EventListenerOptions,func:F)->EventListen<FnWrapper<F>>
     where S:Into<Cow<'static,str>>{
        EventListen::new_with_options(target,event_type,options,FnWrapper(func))
    }
}

impl<F: Listen> EventListen<F>{
    pub fn new<S>(target: &EventTarget, event_type: S, func: F) -> Self
where
    S: Into<Cow<'static, str>>,
    {
        let options = gloo::events::EventListenerOptions::default();
        Self::new_with_options(target, event_type, options, func)
    }

    pub fn new_with_options<S>(target: &EventTarget, event_type: S, options:gloo::events::EventListenerOptions,func: F) -> Self
where
    S: Into<Cow<'static, str>>,
    {
        let callback:Box<dyn Listen>=Box::new(func);
        let j=Box::into_raw(callback);
        let j:*mut (dyn Listen+'static)=j as *mut _;

        
        let mut callback:Box<dyn Listen+'static>=unsafe{Box::from_raw(j)};
        //let options = gloo::events::EventListenerOptions::enable_prevent_default();

        let e=EventListener::new_with_options(target,event_type,options,move|e|callback.call(e));

        EventListen { _a: e, func:std::marker::PhantomData }
    }
}
