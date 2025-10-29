pub mod core;
pub mod handler;
pub mod switchboard;
pub mod message;
pub mod listener;
pub mod matching;
pub mod stubs;
pub mod database;

#[cfg(test)]
mod tests;

pub use core::MessageBus;
use core::{Endpoint, Subscription};
use std::{
    self,
    any::Any,
    cell::{OnceCell, RefCell},
    rc::Rc
};

use handler::ShareableMessageHandler;
use matching::is_matching_backtracking;
use stubs::get_stub_shareable_handler;

use crate::messages::data::DataResponse;
use ant_core::{collections, UUID4};
use ant_model::data::DataType;
use ustr::Ustr;

pub use crate::msgbus::core::{MStr, Pattern, Topic};
pub use crate::msgbus::message::BusMessage;
pub use crate::msgbus::stubs::*;

thread_local! {
    static MESSAGE_BUS: OnceCell<Rc<RefCell<MessageBus>>> = const { OnceCell::new() };
}

fn set_message_bus(msgbus: Rc<RefCell<MessageBus>>){
    MESSAGE_BUS.with(|bus|{
        if bus.set(msgbus).is_err(){
            panic!("Faild to set MessageBus: already initialized for this thread");
        }
    });
}

// 获取本地线程的message bus
pub fn get_message_bus() -> Rc<RefCell<MessageBus>>{
    MESSAGE_BUS.with(|bus|{
        bus.get_or_init(||{
            let msgbus = MessageBus::default();
            Rc::new(RefCell::new(msgbus))
        })
        .clone()
    })
}

pub fn send_any(endpoint: MStr<Endpoint>, message: &dyn Any){
    let handler = get_message_bus()
        .borrow()
        .get_endpoint(endpoint).cloned();

    if let Some(handler) = handler{
        handler.0.handle(message);
    } else {
        log::error!("send_any: no registerd endpoinst '{endpoint}'")
    }
}

pub fn send<T: 'static>(endpoint: MStr<Endpoint>, message: T){
    let handler = get_message_bus()
        .borrow()
        .get_endpoint(endpoint).cloned();
    if let Some(handler) = handler{
        handler.0.handle(&message);
    } else {
        log::error!("send: no registered endpoint '{endpoint}'")
    }
}

pub fn send_response(correlation_id: &UUID4, message: &DataResponse){
    let handler = get_message_bus()
        .borrow()
        .get_response_handler(correlation_id)
        .cloned();

    if let Some(handler) = handler {
        handler.0.handle(message);
    } else {
        log::error!("send_response: handler not found for correlation_id '{correlation_id}'")
    }
}

// pub fn publish_data(topic: &Ustr, message: Data){
//     let matching_subs = get_message_bus()
//         .borrow_mut()
//         .matching_subscriptions(topic);

//     for sub in matching_subs{
//         sub.hanlder.0.handle(&message);
//     }
// }

pub fn response(correlation_id: &UUID4, message: &dyn Any){
    let handler = get_message_bus()
        .borrow()
        .get_response_handler(correlation_id)
        .cloned();

    if let Some(handler) = handler{
        handler.0.handle(message);
    } else {
        log::error!("response: handler not found for correlation_id '{correlation_id}'")
    }
}

pub fn register_response_handler(correlation_id: &UUID4, handler: ShareableMessageHandler){
    if let Err(e) = get_message_bus()
        .borrow_mut()
        .register_response_handler(correlation_id, handler)
    {
        log::error!("Failed to register request handler: {e}");
    }
}

pub fn publish(topic: MStr<Topic>, message: &dyn Any){
    log::trace!("Publising topic '{topic}' {message:?}");
    let matching_subs = get_message_bus()
        .borrow_mut()
        .inner_matching_subscriptions(topic);

    log::trace!("Matched {} subscription", matching_subs.len());

    for sub in matching_subs{
        log::trace!("Matched {sub:?}");
        sub.hanlder.0.handle(message);
    }    
}

pub fn register(endpoint: MStr<Endpoint>, handler:ShareableMessageHandler){
    log::debug!("Register endpoint '{endpoint}' with handler ID {}", handler.0.id());
    get_message_bus()
        .borrow_mut()
        .endpoints
        .insert(endpoint, handler);
}

pub fn deregister(endpoint: MStr<Endpoint>){
    log::debug!("Deregistering endpoint '{endpoint}'");

    get_message_bus()
        .borrow_mut()
        .endpoints
        .shift_remove(&endpoint);
}

pub fn subscribe(pattern: MStr<Pattern>, 
    handler: ShareableMessageHandler,
    priority: Option<u8>){
    
    let msgbus = get_message_bus();
    let mut msgbus_ref_mut = msgbus.borrow_mut();

    let sub = Subscription::new(pattern, handler, priority);

    log::debug!(
        "Subscribing {:?} for pattern '{}'",
        sub.hanlder,
        sub.pattern
    );

    if msgbus_ref_mut.subscriptions.contains(&sub){
        log::warn!("{sub:?} already exists");
        return;
    }

    for(topic, subs) in msgbus_ref_mut.topics.iter_mut(){
        if is_matching_backtracking(*topic, sub.pattern){
            subs.push(sub.clone());
            subs.sort();
            log::debug!("Added subscription for '{topic}'");
        }
    }

    msgbus_ref_mut.subscriptions.insert(sub);
}

pub fn subscribe_topic(topic: MStr<Topic>, handler: ShareableMessageHandler,  priority: Option<u8>){
    subscribe(topic.into(), handler, priority);
}

pub fn subscribe_str<T: AsRef<str>>(
    pattern: T,
    handler: ShareableMessageHandler,
    priority: Option<u8>,
){
    subscribe(MStr::from(pattern), handler, priority);
}

pub fn unsubscribe(pattern: MStr<Pattern>, handler: ShareableMessageHandler){
    log::debug!("Unsubscribing {handler:?} from pattern '{pattern}'");

    let sub = core::Subscription::new(pattern, handler, None);

    // get_message_bus()
    //     .borrow_mut()
    //     .topics
    //     .values_mut()
    //     .for_each(|subs|{
    //         if let Ok(index) = sub.binary_search(&sub){
    //             subs.remove(index);
    //         }
    //     });

    // let removed = get_message_bus().borrow_mut().subscriptions.remove(&sub);
}

pub fn unsubscribe_topic(topic: MStr<Topic>, handler: ShareableMessageHandler) {
    unsubscribe(topic.into(), handler);
}


pub fn unsubscribe_str<T: AsRef<str>>(pattern: T, handler: ShareableMessageHandler) {
    unsubscribe(MStr::from(pattern), handler);
}

pub fn is_subscribed<T: AsRef<str>>(pattern: T, handler: ShareableMessageHandler) -> bool {
    let pattern = MStr::from(pattern.as_ref());
    let sub = Subscription::new(pattern, handler, None);
    get_message_bus().borrow().subscriptions.contains(&sub)
}

pub fn subscriptions_count<T: AsRef<str>>(topic: T) -> usize {
    get_message_bus().borrow().subscriptions_count(topic)
}
