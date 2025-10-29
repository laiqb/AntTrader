use std::{
    any::{Any, type_name},
    fmt::Debug,
    marker::PhantomData,
    rc::Rc,
};

use ant_core::UUID4;
use ustr::Ustr;

pub trait MessageHandler: Any{
    fn id(&self) -> Ustr;

    fn handle(&self, message: &dyn Any);

    fn as_any(&self) -> &dyn Any;
}

impl PartialEq for dyn MessageHandler{
    fn eq(&self, other: &Self) -> bool{
        self.id() == other.id()
    }
}

impl Eq for dyn MessageHandler{}

pub struct TypedMessageHandler<T: 'static + ?Sized, F: Fn(&T) + 'static>{
    id: Ustr,
    callback: F,
    _phantom: PhantomData<T>,
}

impl<T: 'static, F:Fn(&T) + 'static> TypedMessageHandler<T, F>{
    pub fn new<S: AsRef<str>>(id: Option<S>, callback:F) -> Self{
        let id_str = id
            .map(|s|Ustr::from(s.as_ref()))
            .unwrap_or_else(|| generate_handler_id(&callback));
        
        Self{
            id: id_str,
            callback,
            _phantom: PhantomData,
        }
    }

    pub fn from(callback: F) -> Self {
        Self::new::<Ustr>(None, callback)
    }
}

fn generate_handler_id<T: 'static + ?Sized, F: 'static + Fn(&T)>(callback: &F) -> Ustr{
    let callback_ptr = std::ptr::from_ref(&callback);
    let uuid = UUID4::new();
    Ustr::from(&format!("<{callback_ptr:?}>-{uuid}"))
}

impl<T:'static, F:Fn(&T) + 'static> MessageHandler for TypedMessageHandler<T, F>{
    fn id(&self)-> Ustr{
        self.id
    }

    fn handle(&self, message: &dyn Any){
        if let Some(typed_msg) = message.downcast_ref::<T>(){
            (self.callback)(typed_msg);
        } else {
            log::error!("Expected message of type {}", type_name::<T>());
        }
    }

    fn as_any(&self) -> &dyn Any{
        self
    }
}

impl<F: Fn(&dyn Any) + 'static> TypedMessageHandler<dyn Any, F>{
    pub fn new_any<S: AsRef<str>>(id: Option<S>, callback: F) -> Self{
        let id_ustr = id
            .map(|s|Ustr::from(s.as_ref()))
            .unwrap_or_else(|| generate_handler_id(&callback));
        Self{
            id: id_ustr,
            callback,
            _phantom: PhantomData,
        }
    }

    pub fn from_any<S: AsRef<str>>(id_opt: Option<S>, callback: F) -> Self{
        Self::new_any(id_opt, callback)
    }

    pub fn with_any(callback: F) -> Self{
        Self::new_any::<&str>(None, callback)
    }
}

impl<F:Fn(&dyn Any) + 'static> MessageHandler for TypedMessageHandler<dyn Any, F>{
    fn id(&self) -> Ustr{
        self.id
    }

    fn handle(&self, message: &dyn Any){
        (self.callback)(message);
    }

    fn as_any(&self) -> &dyn Any{
        self
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct ShareableMessageHandler(pub Rc<dyn MessageHandler>);

impl ShareableMessageHandler {
    pub fn id(&self) -> Ustr {
        self.0.id()
    }
}

impl Debug for ShareableMessageHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(ShareableMessageHandler))
            .field("id", &self.0.id())
            .field("type", &std::any::type_name::<Self>().to_string())
            .finish()
    }
}

impl From<Rc<dyn MessageHandler>> for ShareableMessageHandler {
    fn from(value: Rc<dyn MessageHandler>) -> Self {
        Self(value)
    }
}