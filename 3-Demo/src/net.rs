//! # ESP-NOW Network Driver
//!
//! This driver allows broadcasting info via the ESP-NOW ad-hoc network.
//!
//! The datagrams are encrypted using AES-256-CCM but may be replayed!
//! Don't use this crypto for productive systems!
//!
//! Example for starting the network:
//!
//! ```rust
//! use embassy_time::{Ticker, Duration};
//! use embassy_futures::select::{Either, select};
//!
//! pub async fn main(spawner: embassy_executor::Spawner, peripherals: esp_hal::peripherals::Peripherals) {
//!     // Start RTOS for time driver
//!     rtos::start(peripherals.TIMG0, peripherals.SW_INTERRUPT);
//!
//!     // Init the network stack
//!     let secret = b"Change me!"; // This is the shared network secret!
//!     let (net_rx, net_tx) = net::start_net(&spawner, peripherals.WIFI, secret);
//!
//!     // Init the message ticker
//!     let mut tick = Ticker::every(Duration::from_secs(1));
//!
//!     let msg = b"hello";
//!     loop {
//!         match select(net_rx.recv(), tick.next()).await {
//!             Either::First(msg) => {
//!                 defmt::info!("Received message: {:a}", msg);
//!             }
//!             Either::Second(_) => {
//!                 net_tx.send(msg.clone()).await;
//!                 defmt::info!("Sent message: {:a}", msg);
//!             }
//!         }
//!     }
//! }
//! ```

use core::marker::PhantomData;

use aes::Aes256;
use alloc::vec::Vec;
use ccm::{AeadInPlace as _, Ccm, Key, KeyInit as _, Nonce, consts::U8};
use embassy_executor::Spawner;
use embassy_futures::select::{Either, select};
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::{Channel, Receiver, Sender},
};
use esp_hal::peripherals::WIFI;
use esp_radio::{
    esp_now::{BROADCAST_ADDRESS, ReceivedData},
    wifi::{self, WifiMode, sta_mac},
};
use serde::{Serialize, de::DeserializeOwned};
use static_cell::StaticCell;

type Aes256Ccm = Ccm<Aes256, U8, U8>;

#[embassy_executor::task]
async fn net_task(
    wifi: WIFI<'static>,
    tx: Receiver<'static, CriticalSectionRawMutex, Vec<u8>, 1>,
    rx: Sender<'static, CriticalSectionRawMutex, ReceivedData, 1>,
) {
    // Start WIFI stack
    let radio_init = esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller");
    let (mut controller, interfaces) = wifi::new(&radio_init, wifi, Default::default())
        .expect("Failed to initialize Wi-Fi controller");

    // Configure ESP-NOW
    controller.set_mode(WifiMode::Sta).unwrap();
    controller.start().unwrap();
    let mut esp_now = interfaces.esp_now;
    esp_now.set_channel(11).unwrap();

    loop {
        match select(esp_now.receive_async(), tx.receive()).await {
            Either::First(frm) => {
                //defmt::info!("net rx from {}: {}", frm.info.src_address, frm.data());
                rx.send(frm).await
            }
            Either::Second(data) => {
                //defmt::info!("net tx: {}", data.as_slice());
                esp_now.send_async(&BROADCAST_ADDRESS, &data).await.unwrap();
            }
        }
    }
}

fn mac_to_nonce(mac: [u8; 6]) -> Nonce<U8> {
    let mut nonce = [0u8; 8];
    nonce[..6].copy_from_slice(&mac);
    nonce.into()
}

/// Helper for receiving items from the network
pub struct NetRx<T: DeserializeOwned> {
    ch: Receiver<'static, CriticalSectionRawMutex, ReceivedData, 1>,
    key: &'static Aes256Ccm,
    _p: PhantomData<T>,
}

impl<T: DeserializeOwned> NetRx<T> {
    fn new(
        ch: Receiver<'static, CriticalSectionRawMutex, ReceivedData, 1>,
        key: &'static Aes256Ccm,
    ) -> Self {
        Self {
            ch,
            key,
            _p: PhantomData,
        }
    }

    /// Receives an item from the network
    pub async fn recv(&self) -> T {
        loop {
            let frm = self.ch.receive().await;
            let nonce = mac_to_nonce(frm.info.src_address);
            let mut data = Vec::from(frm.data());
            if let Err(_) = self.key.decrypt_in_place(&nonce, &[], &mut data) {
                defmt::warn!("Cannot decrypt data!");
            } else {
                match postcard::from_bytes(&data) {
                    Ok(item) => break item,
                    Err(_) => defmt::warn!("Deserialize error"),
                }
            }
        }
    }
}

/// Helper for broadcasting items on the network
pub struct NetTx<T: Serialize> {
    ch: Sender<'static, CriticalSectionRawMutex, Vec<u8>, 1>,
    key: &'static Aes256Ccm,
    nonce: Nonce<U8>,
    _p: PhantomData<T>,
}

impl<T: Serialize> NetTx<T> {
    fn new(
        ch: Sender<'static, CriticalSectionRawMutex, Vec<u8>, 1>,
        key: &'static Aes256Ccm,
        nonce: [u8; 6],
    ) -> Self {
        Self {
            ch,
            key,
            nonce: mac_to_nonce(nonce),
            _p: PhantomData,
        }
    }

    /// Broadcasts the given item on the network
    pub async fn send(&self, item: T) {
        if let Ok(mut data) = postcard::to_allocvec(&item)
            && let Ok(()) = self.key.encrypt_in_place(&self.nonce, &[], &mut data)
        {
            self.ch.send(data).await;
        }
    }
}

/// Starts the network stack.
///
/// - `spawner` is a [embassy_executor::Spawner] which will be used to start the internal networking task
/// - `wifi` is the WIFI peripheral
/// - `secret` is the shared secret and must be the same for all network nodes.
pub fn start_net<T: DeserializeOwned + Serialize>(
    spawner: &Spawner,
    wifi: WIFI<'static>,
    secret: &[u8],
) -> (NetRx<T>, NetTx<T>) {
    let tx = {
        static CELL: StaticCell<Channel<CriticalSectionRawMutex, Vec<u8>, 1>> = StaticCell::new();
        CELL.init(Channel::new())
    };

    let rx = {
        static CELL: StaticCell<Channel<CriticalSectionRawMutex, ReceivedData, 1>> =
            StaticCell::new();
        CELL.init(Channel::new())
    };

    let mut key = [0u8; 32];
    let key = {
        let copy_len = key.len().min(secret.len());
        key[..copy_len].copy_from_slice(&secret[..copy_len]);
        Key::<Aes256>::from_slice(&key)
    };

    let key = {
        static CELL: StaticCell<Aes256Ccm> = StaticCell::new();
        CELL.init(Aes256Ccm::new(key))
    };

    spawner.must_spawn(net_task(wifi, tx.receiver(), rx.sender()));

    (
        NetRx::new(rx.receiver(), key),
        NetTx::new(tx.sender(), key, sta_mac()),
    )
}
