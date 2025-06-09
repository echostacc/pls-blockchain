use pls_blockchain::message::{PLSMessageType, message};

fn main() {
    let message = message(PLSMessageType::Proof, vec![1, 2, 3, 4]);
    println!("Message: {}", message);
}
