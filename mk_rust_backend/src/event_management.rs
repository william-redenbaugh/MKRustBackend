use std::rc::Rc;
use std::sync::Mutex;
use std::thread;
use std::sync::mpsc::channel;

enum event_types_t{
    ACK = 0,
    LED_ANIMATION_UPDATE,
    KEYBOARD_MACRO_UPDATE,
    HAPTIC_FEEDBACK_UPDATE,
}

struct PublishedEvent{
    event_type: event_types_t,
    data_str: String,
    data: Vec<u8>
}

struct EventSubscriber{
    name: String,

}

struct EventManagement{

}

fn eventmanagement_thread(){

}

fn start_eventmanagement_thread(){
    thread::spawn(||eventmanagement_thread);
}

fn publish_event(event: event_types_t){

}

