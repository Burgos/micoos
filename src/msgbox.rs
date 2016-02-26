// Message box to receive and send messages

#[derive(Default, Copy, Clone)]
pub struct Message {
    pub sender: i32,
    pub body: [i32; 16]
}

impl Message {
    pub fn new(sender: i32, body: [i32; 16]) -> Message {
        Message {
            sender: sender,
            body: body
        }
    }
}

const NumberOfMessages: usize = 10;

#[derive(Default, Copy, Clone)]
pub struct MessageBox {
    pub messages: [Message; NumberOfMessages],
    pub first_unread: usize,
    pub first_empty: usize,
}

pub enum MessageBoxResult {
    Full
}

impl MessageBox {
    pub fn new () -> MessageBox {
        MessageBox {
            ..Default::default()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.first_unread == self.first_empty
    }

    pub fn send_message(&mut self, msg: Message) -> Result<(), MessageBoxResult> {
        if (self.first_empty + 1 % NumberOfMessages) == self.first_unread {
            return Err(MessageBoxResult::Full);
        }

        self.messages[self.first_empty] = msg;
        self.first_empty = (self.first_empty + 1) % NumberOfMessages;

        Ok(())
    }

    pub fn get_next_unread(&mut self) -> &Message {
        let unread = self.first_unread;
        self.first_unread = (self.first_unread + 1) % NumberOfMessages;

        &self.messages[unread]
    }
}
