#[derive(Debug)]
enum SomeCommand {
    One,
    Two,
}

#[derive(Debug)]
enum SomeOtherCommand {
    OneOther,
    TwoOther,
}

#[derive(Debug)]
enum CommandTypes {
    SomeCommand(SomeCommand),
    SomeOtherCommand(SomeOtherCommand),
}

impl CommandTypeCollection for CommandTypes {}
trait CommandTypeCollection {}

#[derive(Debug)]
struct SomeObject;

impl Subscriber for SomeObject {
    fn process(&self, command: &CommandTypes) {
        match command {
            CommandTypes::SomeCommand(command) => {
                println!("{:?} is processing: {:?}", self, command);
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
struct SomeOtherObject;

impl Subscriber for SomeOtherObject {
    fn process(&self, command: &CommandTypes) {
        match command {
            CommandTypes::SomeOtherCommand(command) => {
                println!("{:?} is processing: {:?}", self, command);
            }
            _ => {}
        }
    }
}

trait Message {}

trait Subscriber {
    fn process(&self, command: &CommandTypes);
}

struct MessageProcessor {
    subscribers: Vec<Box<dyn Subscriber>>,
}

impl MessageProcessor {
    fn new() -> Self {
        Self {
            subscribers: Vec::new(),
        }
    }

    fn register(&mut self, item: Box<dyn Subscriber>) {
        self.subscribers.push(item)
    }

    async fn run(self) {
        loop {
            println!("STATIC: processing commands..");

            // cmd
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            let command = CommandTypes::SomeCommand(SomeCommand::One);
            for subscriber in &self.subscribers {
                subscriber.process(&command);
            }

            // cmd
            tokio::time::sleep(std::time::Duration::from_millis(750)).await;
            let command = CommandTypes::SomeCommand(SomeCommand::Two);
            for subscriber in &self.subscribers {
                subscriber.process(&command);
            }

            // cmd
            tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
            let command = CommandTypes::SomeOtherCommand(SomeOtherCommand::OneOther);
            for subscriber in &self.subscribers {
                subscriber.process(&command);
            }
            // cmd
            tokio::time::sleep(std::time::Duration::from_millis(1250)).await;
            let command = CommandTypes::SomeOtherCommand(SomeOtherCommand::TwoOther);
            for subscriber in &self.subscribers {
                subscriber.process(&command);
            }
        }
    }
}

pub fn static_pass() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let some_object = SomeObject;
    let some_other_object = SomeOtherObject;

    let mut message_processor = MessageProcessor::new();

    message_processor.register(Box::new(some_object));
    message_processor.register(Box::new(some_other_object));

    rt.block_on(async {
        message_processor.run().await;
    })
}
