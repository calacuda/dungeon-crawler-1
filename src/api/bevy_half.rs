use super::{
    BevyApiCommand,
    coms::{BevyApi, mk_api_channel},
    http::start_api,
};
use bevy::prelude::*;
use std::thread::{JoinHandle, spawn};

pub struct HttpApiPlugin {
    api_channel: BevyApi,
    // http_half: HttpApi,
    _jh: JoinHandle<Result<(), std::io::Error>>,
}

impl HttpApiPlugin {
    pub fn new() -> Self {
        let (http_half, api_channel) = mk_api_channel();
        let jh = spawn(move || start_api(http_half));

        Self {
            api_channel,
            _jh: jh,
        }
    }
}

impl Plugin for HttpApiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.api_channel.clone())
            .add_event::<BevyApiCommand>()
            .add_systems(Update, step_api);
    }
}

fn step_api(mut cmd_ev: EventWriter<BevyApiCommand>, mut api_chan: ResMut<BevyApi>) {
    if let Some(cmd) = api_chan.recv() {
        cmd_ev.send(cmd);
    }
}
