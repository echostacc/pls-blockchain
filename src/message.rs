#[derive(Debug)]
pub enum PLSMessageType {
    Proof,
    Link,
    Signature,
    Verify
}

impl std::fmt::Display for PLSMessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PLSMessageType::Proof => write!(f, "Proof"),
            PLSMessageType::Link => write!(f, "Link"),
            PLSMessageType::Signature => write!(f, "Signature"),
            PLSMessageType::Verify => write!(f, "Verify"),
        }
    }
}

#[derive(Debug)]
pub struct PLSMessage {
    pub message_type: PLSMessageType,
    pub data: Vec<u8>,
}

impl std::fmt::Display for PLSMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PLSMessage {{ type: {}, data: {:?} }}", self.message_type, self.data)
    }
}

pub fn message(message_type: PLSMessageType, data: Vec<u8>) -> PLSMessage {
    match message_type {
        PLSMessageType::Proof => println!("Creating a Proof message"),
        PLSMessageType::Link => println!("Creating a Link message"),
        PLSMessageType::Signature => println!("Creating a Signature message"),
        PLSMessageType::Verify => println!("Creating a Verify message"),
    }
    PLSMessage { message_type, data }
}