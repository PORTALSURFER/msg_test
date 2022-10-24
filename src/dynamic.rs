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

trait Subscriber {
    fn process(&self, message: Message);
    fn get_name(&self) -> String;
}

struct MessageProcessor {
    subscribers: Vec<ReceiverObject>,
}

pub fn dynamic_pass() {
    println!("Running");
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async move {
        let some_obj = SomeObject;
        let mut msg_proc = MessageProcessor::new();

        msg_proc.register(Box::new(some_obj));

        println!("Entering processors..");
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
            tokio::time::sleep(Duration::from_secs(1)).await;
            for subscriber in &self.subscribers {
                println!("send msg to ({})", subscriber.get_name());
                let message = Box::new(self::SomeObjectCommand::One);
                subscriber.process(message);
            }
        }
    }
}
