#[derive(Debug, PartialEq)]
pub enum PLSMessageType {
    Proof,
    Link,
    Signature,
    Verify,
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
        write!(
            f,
            "PLSMessage {{ type: {}, data: {:?} }}",
            self.message_type, self.data
        )
    }
}

pub fn message(message_type: PLSMessageType, data: Vec<u8>) -> PLSMessage {
    use crate::crypto::hash;

    match message_type {
        PLSMessageType::Proof => {
            println!("Creating a Proof message");

            // P0 = H(N0) where N0 is the nonce
            let proof = hash(&data);
            return PLSMessage {
                message_type,
                data: proof.to_vec(),
            };
        }
        PLSMessageType::Link => println!("Creating a Link message"),
        PLSMessageType::Signature => println!("Creating a Signature message"),
        PLSMessageType::Verify => println!("Creating a Verify message"),
    }
    PLSMessage { message_type, data }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::hash;

    #[test]
    fn test_message_creation() {
        let data = vec![1, 2, 3, 4];
        let message = message(PLSMessageType::Proof, data.clone());
        assert_eq!(message.message_type, PLSMessageType::Proof);
        assert_eq!(message.data, hash(&data).to_vec());
    }

    #[test]
    fn test_message_display() {
        let data = vec![1, 2, 3, 4];
        let message = message(PLSMessageType::Proof, data);
        let expected_hash = hash(&[1, 2, 3, 4]);
        assert_eq!(
            format!("{}", message),
            format!(
                "PLSMessage {{ type: Proof, data: {:?} }}",
                expected_hash.to_vec()
            )
        );
    }

    #[test]
    fn test_message_type_display() {
        let message_type = PLSMessageType::Proof;
        assert_eq!(format!("{}", message_type), "Proof");
    }
}
