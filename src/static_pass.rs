type SubscriberReceiver = tokio::sync::broadcast::Receiver<CommandTypes>;
type SubscriberSender = tokio::sync::broadcast::Sender<CommandTypes>;

#[derive(Debug, Clone)]
enum CommandTypes {
    One,
    Two,
}

#[derive(Debug)]
struct SomeObject {
    receiver: Option<SubscriberReceiver>,
}

impl SomeObject {
    async fn run(self) {
        if let Some(mut receiver) = self.receiver {
            while let Ok(command) = receiver.recv().await {
                match command {
                    CommandTypes::One => println!("One"),
                    CommandTypes::Two => println!("Two"),
                }
            }
        }
    }
}

struct MessageProcessor {
    subscriber_sender: SubscriberSender,
}

impl MessageProcessor {
    fn new() -> Self {
        let (subscriber_sender, _) = tokio::sync::broadcast::channel(8);
        Self { subscriber_sender }
    }

    fn register(&mut self) -> SubscriberReceiver {
        self.subscriber_sender.subscribe()
    }

    async fn run(self) {
        println!("processing commands..");
        loop {
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            let command = CommandTypes::One;
            let _ = self.subscriber_sender.send(command);

            tokio::time::sleep(std::time::Duration::from_millis(800)).await;
            let command = CommandTypes::Two;
            let _ = self.subscriber_sender.send(command);
        }
    }
}

pub fn static_pass() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let mut some_object = SomeObject { receiver: None };
    let mut message_processor = MessageProcessor::new();

    some_object.receiver = Some(message_processor.register());

    rt.block_on(async {
        tokio::spawn(async {
            some_object.run().await;
        });

        message_processor.run().await;
    })
}
