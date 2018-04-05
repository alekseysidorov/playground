use std::borrow::Cow;
use std::time::SystemTime;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use bincode;

use exonum::storage::StorageValue;
use exonum::crypto::{self, hash, CryptoHash, Hash, PublicKey, SecretKey, Signature};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message<'a> {
    signature: Cow<'a, Signature>,
    payload: AuthorizedPayload<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthorizedPayload<'a> {
    version: u16,
    from: Cow<'a, PublicKey>,
    payload: Cow<'a, [u8]>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Transaction<'a> {
    service_id: u32,
    payload: Cow<'a, [u8]>,
}

#[derive(Debug, Serialize, Deserialize)]
enum Consensus<'a> {
    Connect { time: SystemTime },
    Block { state_hash: Hash },
    Transaction(Transaction<'a>),
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

impl<'a> AuthorizedPayload<'a> {
    fn new<'b, T: Into<Consensus<'b>>>(from: PublicKey, msg: T) -> AuthorizedPayload<'a> {
        AuthorizedPayload {
            version: 0,
            from: Cow::Owned(from),
            payload: bincode::serialize(&msg.into()).unwrap().into(),
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

impl<'a> Message<'a> {
    fn from_payload(payload: AuthorizedPayload<'a>, key: &SecretKey) -> Message<'a> {
        Message {
            signature: Cow::Owned(payload.sign(key)),
            payload,
        }
    }

    fn verify(&self) -> bool {
        self.payload.verify(&self.signature)
    }

    fn decode<'b>(self) -> Consensus<'b> {
        let bytes = self.payload.payload;
        bincode::deserialize(&bytes).unwrap()
    }
}

impl<'a> From<CryptoCurrencyTransactions> for Transaction<'a> {
    fn from(value: CryptoCurrencyTransactions) -> Transaction<'a> {
        Transaction {
            service_id: 10,
            payload: bincode::serialize(&value).unwrap().into(),
        }
    }
}

impl<'a, T: Into<Transaction<'a>>> From<T> for Consensus<'a> {
    fn from(tx: T) -> Consensus<'a> {
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
