use std::time::SystemTime;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use bincode;

use exonum::storage::StorageValue;
use exonum::crypto::{self, hash, CryptoHash, Hash, PublicKey, SecretKey, Signature};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    signature: Signature,
    payload: AuthorizedPayload,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthorizedPayload {
    from: PublicKey,
    payload: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Transaction {
    service_id: u32,
    payload: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
enum Consensus {
    Connect { time: SystemTime },
    Block { state_hash: Hash },
    Transaction(Transaction),
}

#[derive(Debug, Serialize, Deserialize)]
enum CryptoCurrencyTransactions {
    Issue {
        amount: u64,
        seed: u32,
    },
    Transfer {
        to: PublicKey,
        amount: u64,
        seed: u32,
    },
}

impl AuthorizedPayload {
    fn new<T: Into<Consensus>>(from: PublicKey, msg: T) -> AuthorizedPayload {
        AuthorizedPayload {
            from,
            payload: bincode::serialize(&msg.into()).unwrap(),
        }
    }

    fn sign(&self, secret_key: &SecretKey) -> Signature {
        let bytes = bincode::serialize(self).unwrap();
        crypto::sign(&bytes, secret_key)
    }

    fn verify(&self, signature: &Signature) -> bool {
        let bytes = bincode::serialize(self).unwrap();
        crypto::verify(signature, &bytes, &self.from)
    }
}

impl Message {
    fn from_payload(payload: AuthorizedPayload, key: &SecretKey) -> Message {
        Message {
            signature: payload.sign(key),
            payload,
        }
    }

    fn verify(&self) -> bool {
        self.payload.verify(&self.signature)
    }

    fn decode(self) -> Consensus {
        let bytes = self.payload.payload;
        bincode::deserialize(&bytes).unwrap()
    }
}

impl From<CryptoCurrencyTransactions> for Transaction {
    fn from(value: CryptoCurrencyTransactions) -> Transaction {
        Transaction {
            service_id: 10,
            payload: bincode::serialize(&value).unwrap(),
        }
    }
}

impl<T: Into<Transaction>> From<T> for Consensus {
    fn from(tx: T) -> Consensus {
        Consensus::Transaction(tx.into())
    }
}

#[test]
fn test_serialize_verify() {
    let tx = CryptoCurrencyTransactions::Issue {
        amount: 1000,
        seed: 10,
    };
    let keypair = crypto::gen_keypair();
    let msg = Message::from_payload(AuthorizedPayload::new(keypair.0, tx), &keypair.1);
    assert!(msg.verify());

    let consensus = msg.decode();
    if let Consensus::Transaction(tx) = consensus {
        let tx: CryptoCurrencyTransactions = bincode::deserialize(&tx.payload).unwrap();
        println!("{:?}", tx);
    } else {
        panic!("Ooops");
    }
}
