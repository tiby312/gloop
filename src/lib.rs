


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





// pub fn from_closure<F:FnMut(&Event),S>(target:&EventTarget,event_type:S,func:F)->EventListenerWrapper<impl Listen> where
// S: Into<Cow<'static, str>>,{
//     EventListenerWrapper::new(target,event_type,func)
// }


pub struct EventListenerWrapper<F>{
    _a:EventListener,
    func:std::marker::PhantomData<F>
    
}

impl<F:FnMut(&Event)> EventListenerWrapper<FnWrapper<F>>{

    pub fn from_closure<S>(target:&EventTarget,event_type:S,func:F)->EventListenerWrapper<FnWrapper<F>>
     where S:Into<Cow<'static,str>>{
        EventListenerWrapper::new(target,event_type,FnWrapper(func))
    }
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
        let options = gloo::events::EventListenerOptions::enable_prevent_default();

        let e=EventListener::new_with_options(target,event_type,options,move|e|callback.call(e));

        EventListenerWrapper { _a: e, func:std::marker::PhantomData }
    }
}


// pub struct EventListenerWrapper2<F>{
//     _a:EventListener,
//     func:std::marker::PhantomData<F>
    
// }


// impl<F: FnMut(&Event)> EventListenerWrapper2<F>{
//     pub fn new<S>(target: &EventTarget, event_type: S, func: F) -> Self
// where
//     S: Into<Cow<'static, str>>,
//     {
//         let callback:Box<dyn FnMut(&Event)>=Box::new(func);
//         let j=Box::into_raw(callback);
//         let j:*mut (dyn FnMut(&Event)+'static)=j as *mut _;

        
//         let mut callback:Box<dyn FnMut(&Event)+'static>=unsafe{Box::from_raw(j)};

//         let options = gloo::events::EventListenerOptions::enable_prevent_default();

//         //EventListener::new_with_options(target, event_type, options, callback)

//         let e=EventListener::new_with_options(target,event_type,options,move|e|callback.call(e));

//         EventListenerWrapper2 { _a: e, func:std::marker::PhantomData }
//     }
// }


