use super::{BevyApiCommand, BevyApiResponse};
use anyhow::Result;
use bevy::prelude::*;
use crossbeam::channel::{Receiver, Sender, unbounded};

/// bevy-http-api: HTTP help half
#[derive(Clone, Debug)]
pub struct HttpApi {
    rx: Receiver<BevyApiResponse>,
    tx: Sender<BevyApiCommand>,
}

impl HttpApi {
    pub fn recv(&mut self) -> Option<BevyApiResponse> {
        let Ok(cmd) = self.rx.try_recv() else {
            return None;
        };

        Some(cmd)
    }

    pub fn send(&mut self, response: BevyApiCommand) -> Result<()> {
        self.tx.send(response)?;

        Ok(())
    }
}

/// bevy-http-api: Bevy help half
#[derive(Clone, Debug, Resource)]
pub struct BevyApi {
    rx: Receiver<BevyApiCommand>,
    tx: Sender<BevyApiResponse>,
}

impl BevyApi {
    pub fn recv(&mut self) -> Option<BevyApiCommand> {
        let Ok(cmd) = self.rx.try_recv() else {
            return None;
        };

        Some(cmd)
    }

    pub fn send(&mut self, response: BevyApiResponse) -> Result<()> {
        self.tx.send(response)?;

        Ok(())
    }
}

pub fn mk_api_channel() -> (HttpApi, BevyApi) {
    let (tx_1, rx_1) = unbounded();
    let (tx_2, rx_2) = unbounded();

    (
        HttpApi { rx: rx_1, tx: tx_2 },
        BevyApi { rx: rx_2, tx: tx_1 },
    )
}
