use std::time::Duration;
type Message = Box<dyn Command>;

type ReceiverObject = Box<dyn Subscriber>;

trait Command {
    fn get_name(&self) -> String;
}

#[derive(Debug)]
struct SomeObject;

impl Subscriber for SomeObject {
    fn process(&self, message: Message) {
        println!("Processing some_object, ({})", message.get_name());
    }

    fn get_name(&self) -> String {
        String::from("SomeObject")
    }
}

pub enum SomeObjectCommand {
    One,
    Two,
}

impl Command for SomeObjectCommand {
    fn get_name(&self) -> String {
        String::from("SomeObjectCommand")
    }
}

#[derive(Debug)]
struct SomeOtherObject;

impl Subscriber for SomeOtherObject {
    fn process(&self, message: Message) {
        println!("Processing some_other_object, ({})", message.get_name());
    }

    fn get_name(&self) -> String {
        String::from("SomeOtherObject")
    }
}

pub enum SomeOtherObjectCommand {
    OneOther,
    TwoOther,
}

impl Command for SomeOtherObjectCommand {
    fn get_name(&self) -> String {
        String::from("SomeOtherObjectCommand")
    }
}

trait Subscriber {
    fn process(&self, message: Message);
    fn get_name(&self) -> String;
}

struct MessageProcessor {
    subscribers: Vec<ReceiverObject>,
}

pub fn dynamic_pass() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async move {
        let some_object = SomeObject;
        let some_other_object = SomeOtherObject;

        let mut msg_proc = MessageProcessor::new();

        msg_proc.register(Box::new(some_object));
        msg_proc.register(Box::new(some_other_object));

        loop {
            msg_proc.run().await;
        }
    });
}

impl MessageProcessor {
    pub fn new() -> Self {
        let subscribers = vec![];

        Self { subscribers }
    }

    pub fn register(&mut self, item: ReceiverObject) {
        self.subscribers.push(item);
    }

    pub async fn run(&self) {
        loop {
            println!("DYNAMIC: processing commands..");

            tokio::time::sleep(Duration::from_millis(500)).await;
            for subscriber in &self.subscribers {
                let message = Box::new(self::SomeObjectCommand::One);
                subscriber.process(message);
            }
            tokio::time::sleep(Duration::from_millis(1750)).await;
            for subscriber in &self.subscribers {
                let message = Box::new(self::SomeObjectCommand::Two);
                subscriber.process(message);
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
            for subscriber in &self.subscribers {
                let message = Box::new(self::SomeOtherObjectCommand::OneOther);
                subscriber.process(message);
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
            for subscriber in &self.subscribers {
                let message = Box::new(self::SomeOtherObjectCommand::TwoOther);
                subscriber.process(message);
            }
        }
    }
}
