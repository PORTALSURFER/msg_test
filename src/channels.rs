use std::time::Duration;

use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}struct SomeObject {
    label: String,
    receiver: Option<tokio::sync::mpsc::UnboundedReceiver<Box<dyn Message>>>,
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
                println!("{} received msg", self.label)
            }
        }
    }
}

impl Subscriber for SomeObject {
    type CommandType = Box<Command>;

    fn set_receiver(&mut self, receiver: tokio::sync::mpsc::UnboundedReceiver<Box<dyn Message>>) {
        self.receiver = Some(receiver);
    }

    fn get_name(&self) -> String {
        self.label.to_owned()
    }
}

trait Subscriber {
    type CommandType: Message;
    fn set_receiver(&mut self, receiver: tokio::sync::mpsc::UnboundedReceiver<Box<dyn Message>>);
    fn get_name(&self) -> String;
}

#[derive(Debug)]
enum Command {
    One,
    Two,
}

impl Message for Box<Command> {}

impl Message for Command {}

trait Message: Send {}

trait SenderType {
    type Command;
}

pub struct MessageProcessor {
    message_tunnels: Vec<MessageTunnel>,
}

struct MessageTunnel {
    sender: tokio::sync::mpsc::UnboundedSender<Box<dyn Message>>,
}

impl MessageTunnel {
    fn new(subscriber: &mut impl Subscriber) -> Self {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        subscriber.set_receiver(receiver); // subscriber passing wrong message type
        Self { sender }
    }
}

impl MessageProcessor {
    fn new() -> Self {
        Self {
            message_tunnels: Vec::new(),
        }
    }

    fn subscribe(&mut self, subscriber: &mut impl Subscriber) {
        println!("subscribing {}", subscriber.get_name());
        let message_tunnel = MessageTunnel::new(subscriber);
        self.message_tunnels.push(message_tunnel);
    }

    async fn run(&self) {
        loop {
            println!("processing messages..");
            tokio::time::sleep(Duration::from_millis(1000)).await;

            for msg_type in &self.message_tunnels {
                println!("sending msg");
                let _ = msg_type.sender.send(Box::new(Command::One));
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
