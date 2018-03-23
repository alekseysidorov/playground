extern crate bincode;
extern crate exonum;
#[macro_use]
extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use exonum::storage::StorageValue;
use exonum::crypto::{hash, CryptoHash, Hash, Signature, PublicKey};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
struct ConsenusConfiguration {
    validators_count: u16,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct BlockchainConfiguration {
    consensus: ConsenusConfiguration,
    services: BTreeMap<u16, ServiceConfiguration>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
struct ServiceMetadata {
    id: u16,
    version: u8,
    enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct ServiceConfiguration {
    meta: ServiceMetadata,
    data: Vec<u8>,
}

impl ServiceConfiguration {
    pub fn new<T: Serialize>(meta: ServiceMetadata, value: &T) -> Result<Self, failure::Error> {
        let data = bincode::serialize(&value)?;
        Ok(ServiceConfiguration { meta, data })
    }
}

impl BlockchainConfiguration {
    fn new<I: IntoIterator<Item = ServiceConfiguration>>(
        consensus: ConsenusConfiguration,
        services: I,
    ) -> Result<Self, failure::Error> {
        Ok(BlockchainConfiguration {
            consensus,
            services: services
                .into_iter()
                .map(|item| (item.meta.id, item))
                .collect::<BTreeMap<_, _>>(),
        })
    }

    fn service_config<T>(&self, id: u16) -> Result<T, failure::Error>
        where for<'de> T: Deserialize<'de>
    {
        self.services
            .get(&id)
            .ok_or(format_err!("Unable to find config for the given id"))
            .and_then(|service| bincode::deserialize(service.data.as_ref()).map_err(Into::into))
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct FirstService {
    name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct SecondService {
    name: String,
}

impl CryptoHash for BlockchainConfiguration {
    fn hash(&self) -> Hash {
        hash(&bincode::serialize(&self).unwrap())
    }
}

impl StorageValue for BlockchainConfiguration {
    fn into_bytes(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }

    fn from_bytes(value: ::std::borrow::Cow<[u8]>) -> Self {
        bincode::deserialize(value.as_ref()).unwrap()
    }
}

fn main() {
    let blockchain_config = BlockchainConfiguration::new(
        ConsenusConfiguration {
            validators_count: 10,
        },
        vec![
            ServiceConfiguration::new(
                ServiceMetadata {
                    id: 10,
                    version: 1,
                    enabled: false,
                },
                &FirstService {
                    name: "Hello services".to_owned(),
                },
            ).unwrap(),
            ServiceConfiguration::new(
                ServiceMetadata {
                    id: 1,
                    version: 1,
                    enabled: false,
                },
                &SecondService {
                    name: "I am first".to_owned(),
                },
            ).unwrap(),
        ],
    ).unwrap();

    let bytes = blockchain_config.clone().into_bytes();
    let blockchain_config2 = BlockchainConfiguration::from_bytes(bytes.into());

    println!(
        "{}",
        serde_json::to_string_pretty(&blockchain_config).unwrap()
    );

    println!("{:?}", blockchain_config2.service_config::<SecondService>(1).unwrap());

    assert_eq!(blockchain_config.hash(), blockchain_config2.hash());
}
