#[derive(Debug)]
struct SomeObject;

impl Subscriber for SomeObject {}

trait Subscriber: std::fmt::Debug {}

type SubscriberType = Box<dyn Subscriber>;

pub struct MessageProcessor {
    subscribers: Vec<SubscriberType>,
}

impl MessageProcessor {
    fn new() -> Self {
        Self {
            subscribers: Vec::new(),
        }
    }

    fn subscribe(&mut self, subscriber: SubscriberType) {
        println!("subscribed {:?}", subscriber);
        self.subscribers.push(subscriber);
    }
}

pub fn channels() {
    let mut message_processor = MessageProcessor::new();
    let some_object = SomeObject;

    message_processor.subscribe(Box::new(some_object));
}
