use std::time::Duration;

struct SomeObject {}

impl SomeObject {
    fn new() -> Self {
        Self {}
    }
}

impl Subscriber for SomeObject {
    fn process(&self) {
        print!("Processing someobject")
    }
}

trait Subscriber {
    fn process(&self);
}

struct MessageProcessor<I>
where
    I: Subscriber,
{
    subscribers: Vec<I>,
}

fn main() {
    print!("Running");

    let runtime = tokio::runtime::Runtime::new().unwrap();

    runtime.block_on(async move {
        let some_obj = SomeObject::new();
        let mut msg_proc = MessageProcessor::new();

        msg_proc.register(some_obj);
        print!("Entering processors..");
        loop {
            msg_proc.run().await
        }
    });
}

impl<I> MessageProcessor<I>
where
    I: Subscriber,
{
    pub fn new() -> Self {
        let subscribers = vec![];

        Self { subscribers }
    }

    pub fn register(&mut self, item: I)
    where
        I: Subscriber,
    {
        self.subscribers.push(item);
    }

    pub async fn run(&self) {
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_millis(500)).await;
                print!("World");
            }
        });
    }
}
