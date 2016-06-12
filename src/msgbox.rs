// Message box to receive and send messages

#[derive(Default, Copy, Clone)]
pub struct Message {
    pub sender: u32,
    pub body: [u32; 16]
}

impl Message {
    pub fn new(sender: u32, body: [u32; 16]) -> Message {
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
    Full,
    NoSuchProcess
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
        if cfg!(feature="log-msgbox") {
            kprint!("Inside send message. first_unread = {}, first_empty = {}\n",
                    self.first_unread, self.first_empty);
        }
        
        if ((self.first_empty + 1) % NumberOfMessages) == self.first_unread {
            if cfg!(feature="log-msgbox") {
                kprint!("MessageBox is full\n");
            }

            return Err(MessageBoxResult::Full);
        }

        self.messages[self.first_empty] = msg;
        self.first_empty = (self.first_empty + 1) % NumberOfMessages;

        if cfg!(feature="log-msgbox") {
            kprint!("Message was sent.\n");
        }

        Ok(())
    }

    pub fn get_next_unread(&mut self) -> &Message {
        let unread = self.first_unread;
        self.first_unread = (self.first_unread + 1) % NumberOfMessages;

        &self.messages[unread]
    }
}
