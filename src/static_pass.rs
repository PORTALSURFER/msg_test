use self::messaging::DefineCommand;
use std::time::Duration;

mod messaging {
    pub trait DefineCommand: Sized {
        type Command: Clone;
    }
}

#[derive(Clone)]
enum TestCommand {
    One,
    Two,
}

struct SomeObject<C: DefineCommand> {
    receiver: tokio::sync::broadcast::Receiver<C::Command>,
}

impl<R> SomeObject<R>
where
    R: DefineCommand<Command = TestCommand>,
{
    async fn run(mut self) {
        println! {"running"};
        while let Ok(command) = self.receiver.recv().await {
            match command {
                TestCommand::One => println!("received One"),
                TestCommand::Two => println!("received Two"),
            }
        }
    }
}

struct Command {}

impl DefineCommand for Command {
    type Command = TestCommand;
}

struct UserInterface<C: DefineCommand> {
    sender: tokio::sync::broadcast::Sender<C::Command>,
}

impl<C> UserInterface<C>
where
    C: DefineCommand<Command = TestCommand>,
{
    async fn run(self) {
        loop {
            let _ = self.sender.send(TestCommand::One);
            let _ = self.sender.send(TestCommand::Two);

            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }
}

pub fn static_pass() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let (sender, receiver) = tokio::sync::broadcast::channel(8);

    let some_object: SomeObject<Command> = SomeObject { receiver };
    let user_interface: UserInterface<Command> = UserInterface { sender };

    rt.block_on(async {
        rt.spawn(async {
            some_object.run().await;
        });

        user_interface.run().await;
    })
}
