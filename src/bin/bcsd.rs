//! bcs daemon !//
//! handles shit !//

use bluer::{Adapter, AdapterEvent, Address, DeviceEvent, DiscoveryFilter, DiscoveryTransport};
use futures::{StreamExt, pin_mut, stream::SelectAll};
use std::{collections::HashSet, env};
use bcs::DeviceInfo; 

async fn query_device(adapter: &Adapter, addr: Address, obj_ref: &mut DeviceInfo) -> bluer::Result<()> {
    // Address type | name | Icon | class | paired | connected | trusted | modalias | rssi | tx
    // power | manufacturer_data | service data
    let device = adapter.device(addr)?;
    obj_ref.address_type = device.address_type().await?;
    obj_ref.name = device.name().await?;
    obj_ref.icon = device.icon().await?;
    obj_ref.class = device.class().await?;
    obj_ref.uuids = Some(device.uuids().await?.unwrap_or_default());
    obj_ref.paired = device.is_paired().await?;
    obj_ref.connected = device.is_connected().await?;
    obj_ref.trusted = device.is_trusted().await?;
    obj_ref.modalias = device.modalias().await?;
    obj_ref.rssi = device.rssi().await?;
    obj_ref.tx_power = device.tx_power().await?;
    obj_ref.manufacturer_data = device.manufacturer_data()
        .await?.unwrap_or_default()
        .into_iter().collect();
    obj_ref.service_data = device.service_data().await?
        .unwrap_or_default().into_iter()
        .collect();
    obj_ref.completed = true;
    Ok(())
}

fn main() -> Result<(), u8> {
    println!("bcsd - todo: everything");
    Ok(())
} 
