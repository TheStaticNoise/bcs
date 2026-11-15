//! bcs standard defines
//!

use std::collections::{BTreeMap, HashSet};
use bluer::{AddressType, Modalias, Uuid};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub address_type: AddressType,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub class: Option<u32>,
    pub uuids: Option<HashSet<Uuid>>,
    pub paired: bool,
    pub connected: bool,
    pub trusted: bool,
    pub modalias: Option<Modalias>,
    pub rssi: Option<i16>,
    pub tx_power: Option<i16>,
    pub manufacturer_data: BTreeMap<u16, Vec<u8>>,
    pub service_data: BTreeMap<Uuid, Vec<u8>>,
    pub completed: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
enum ResponseKind {
    Status,
    DeviceList,
    Info,
    Error,
}
