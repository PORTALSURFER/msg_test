use std::time::Duration;

#[derive(Debug)]
struct SomeObject {
    label: String,
    receiver: Option<Receiver>,
}

impl SomeObject {
    fn new(label: &str) -> Self {
        Self {
            label: label.to_owned(),
            receiver: None,
        }
    }

    async fn run(self) {
        println!("running {}", self.label);
        if let Some(mut receiver) = self.receiver {
            while let Some(message) = receiver.recv().await {
                println!("{} received msg ({:?})", self.label, message);
            }
        }
    }
}

impl Subscriber for SomeObject {
    fn set_receiver(&mut self, receiver: Receiver) {
        self.receiver = Some(receiver);
    }

    fn get_name(&self) -> String {
        self.label.to_owned()
    }
}

trait Subscriber: std::fmt::Debug {
    fn set_receiver(&mut self, receiver: Receiver);
    fn get_name(&self) -> String;
}

#[derive(Debug)]
enum Command {
    One,
    Two,
}

type Sender = tokio::sync::mpsc::UnboundedSender<Command>;
type Receiver = tokio::sync::mpsc::UnboundedReceiver<Command>;

trait SenderType {
    type Command;
}

pub struct MessageProcessor {
    subscribers: Vec<Sender>,
}

impl MessageProcessor {
    fn new() -> Self {
        Self {
            subscribers: Vec::new(),
        }
    }

    fn subscribe(&mut self, subscriber: &mut impl Subscriber) {
        println!("subscribing {}", subscriber.get_name());
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();

        subscriber.set_receiver(receiver);
        self.subscribers.push(sender);
    }

    async fn run(&self) {
        loop {
            println!("processing messages..");
            tokio::time::sleep(Duration::from_millis(1000)).await;

            for subscriber in &self.subscribers {
                let _ = subscriber.send(Command::One);
            }
        }
    }
}

pub fn channels() {
    let mut message_processor = MessageProcessor::new();
    let mut some_object = SomeObject::new("SomeObject");
    message_processor.subscribe(&mut some_object);

    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.spawn(async {
        some_object.run().await;
    });

    rt.block_on(async {
        message_processor.run().await;
    });
}
