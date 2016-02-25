// Message box to receive and send messages

#[derive(Default)]
pub struct Message {
    pub sender: i32,
    pub body: [i32; 16]
}

impl Message {
    fn new(sender: i32, body: [i32; 16]) -> Message {
        Message {
            sender: sender,
            body: body
        }
    }
}

const NumberOfMessages: usize = 10;

#[derive(Default)]
pub struct MessageBox {
    pub messages: [Message; NumberOfMessages],
    pub first_unread: usize,
    pub first_empty: usize,
}

pub enum MessageBoxResult {
    Full
}

impl MessageBox {
    fn new () -> MessageBox {
        MessageBox {
            ..Default::default()
        }
    }

    fn is_empty(&self) -> bool {
        self.first_unread == self.first_empty
    }

    fn send_message(&mut self, msg: Message) -> Result<(), MessageBoxResult> {
        if (self.first_empty + 1 % NumberOfMessages) == self.first_unread {
            return Err(MessageBoxResult::Full);
        }

        self.messages[self.first_empty] = msg;
        self.first_empty = (self.first_empty + 1) % NumberOfMessages;

        Ok(())
    }

    fn get_next_unread(&mut self) -> &Message {
        let unread = self.first_unread;
        self.first_unread = (self.first_unread + 1) % NumberOfMessages;

        &self.messages[unread]
    }
}
