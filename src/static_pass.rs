use std::time::Duration;

#[derive(Debug, Clone)]
enum CommandTypes {
    SomeCommand,
    SomeOtherCommand,
}

impl CommandTypeCollection for CommandTypes {}
trait CommandTypeCollection {}

#[derive(Debug)]
struct SomeObject<C> {
    receiver: Option<tokio::sync::broadcast::Receiver<C>>,
}

impl<C> SomeObject<C> {
    fn new() -> Self {
        Self { receiver: None }
    }
    async fn run() {}
}

impl<C> Subscriber for SomeObject<C> {
    type CommandType = CommandTypes;
}

#[derive(Debug)]
struct SomeOtherObject<C> {
    receiver: Option<tokio::sync::broadcast::Receiver<C>>,
}

impl<C> SomeOtherObject<C> {
    fn new() -> Self {
        Self { receiver: None }
    }
}

impl<C> Subscriber for SomeOtherObject<C> {
    type CommandType = CommandTypes;
}

trait Subscriber {
    type CommandType;
}

struct MessageProcessor<C> {
    subscribers: Vec<Box<dyn Subscriber<CommandType = C>>>,
    sender: tokio::sync::broadcast::Sender<C>,
    receiver: tokio::sync::mpsc::UnboundedReceiver<C>,
}

impl<C: Clone> MessageProcessor<C> {
    fn new(receiver: tokio::sync::mpsc::UnboundedReceiver<C>) -> Self {
        let (sender, _) = tokio::sync::broadcast::channel(8);

        Self {
            subscribers: Vec::new(),
            sender,
            receiver,
        }
    }

    fn register(&mut self, item: Box<dyn Subscriber<CommandType = C>>) {
        self.subscribers.push(item)
    }

    async fn run(self) {
        loop {}
    }
}

struct UI<C> {
    sender: tokio::sync::mpsc::UnboundedSender<C>,
}

impl<C> UI<C> {
    fn new(sender: tokio::sync::mpsc::UnboundedSender<C>) -> Self {
        Self { sender }
    }

    async fn run(self) {
        println!("start ui");
        tokio::time::sleep(Duration::from_millis(500)).await;
        // self.sender.send(CommandTypes::SomeCommand);
    }
}

pub fn static_pass() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();

    let ui = UI::new(sender);

    let some_object: SomeObject<CommandTypes> = SomeObject::new();
    let some_other_object: SomeOtherObject<CommandTypes> = SomeOtherObject::new();

    let mut message_processor = MessageProcessor::new(receiver);

    message_processor.register(Box::new(some_object));
    message_processor.register(Box::new(some_other_object));

    rt.block_on(async {
        tokio::spawn(async {
            ui.run().await;
        });

        message_processor.run().await;
    })
}
