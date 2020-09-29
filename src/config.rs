//! Bridger Config
use crate::result::{Error, Result};
use etc::{Etc, Read};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Ethereum Contract Tuple
#[derive(Serialize, Deserialize)]
pub struct EthereumContractTuple {
    /// Contract Address
    pub address: String,
    /// Contract Topic
    pub topic: String,
}

/// Ethereum Contracts
#[derive(Serialize, Deserialize)]
pub struct EthereumContract {
    /// Ring Contract
    pub ring: EthereumContractTuple,
    /// Kton Contract
    pub kton: EthereumContractTuple,
    /// Bank Contract
    pub bank: EthereumContractTuple,
    /// Issuing Contract
    pub issuing: EthereumContractTuple,
}

/// Ethereum Config
#[derive(Serialize, Deserialize)]
pub struct EthereumConfig {
    /// Ethereum start block number
    ///
    /// Ethereum bridger will scan start from this block
    pub start: u64,
    /// Ethereum rpc url
    pub rpc: String,
    /// Ethereum contracts
    pub contract: EthereumContract,
}

/// Bridger Config
#[derive(Serialize, Deserialize)]
pub struct Config {
    /// Darwinia node url
    pub node: String,
    /// Darwinia relayer proxy address
    pub relayer: String,
    /// Darwinia account seed
    pub seed: String,
    /// Shadow service url
    pub shadow: String,
    /// Ethereum Config
    pub eth: EthereumConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            node: "wss://crab.darwinia.network".to_string(),
            relayer: "".to_string(),
            seed: "//Alice".to_string(),
            shadow: "http://localhost:3000".to_string(),
            eth: EthereumConfig {
                rpc: "https://ropsten.infura.io/v3/0bfb9acbb13c426097aabb1d81a9d016".to_string(),
                start: 8647036,
                contract: EthereumContract {
                    ring: EthereumContractTuple {
                        address: "0xb52FBE2B925ab79a821b261C82c5Ba0814AAA5e0".to_string(),
                        topic: "0xc9dcda609937876978d7e0aa29857cb187aea06ad9e843fd23fd32108da73f10"
                            .to_string(),
                    },
                    kton: EthereumContractTuple {
                        address: "0x1994100c58753793D52c6f457f189aa3ce9cEe94".to_string(),
                        topic: "0xc9dcda609937876978d7e0aa29857cb187aea06ad9e843fd23fd32108da73f10"
                            .to_string(),
                    },
                    bank: EthereumContractTuple {
                        address: "0x6EF538314829EfA8386Fc43386cB13B4e0A67D1e".to_string(),
                        topic: "0xe77bf2fa8a25e63c1e5e29e1b2fcb6586d673931e020c4e3ffede453b830fb12"
                            .to_string(),
                    },
                    issuing: EthereumContractTuple {
                        address: "0x49262B932E439271d05634c32978294C7Ea15d0C".to_string(),
                        topic: "".to_string(),
                    },
                },
            },
        }
    }
}

impl Config {
    /// New config from pathbuf
    pub fn new(path: Option<PathBuf>) -> Result<Self> {
        let c = Etc::from(if let Some(conf) = path {
            conf
        } else if let Some(mut conf) = dirs::home_dir() {
            conf.push(".darwinia/config.toml");
            conf
        } else {
            return Err(Error::Bridger("Could not open home dir".to_string()));
        });

        if let Ok(config) = toml::from_slice(&c.read()?) {
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }
}
