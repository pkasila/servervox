mod renderer;

use std::ops::Deref;
use corevox::devices::device::Device;
use corevox::devices::prototype_device::{Frame, PrototypeDevice};
use corevox::network::server::VoxServer;
use corevox::devices::science_fair_128::ScienceFair128;
use crate::renderer::BaseRenderer;

#[tokio::main]
pub async fn main() {
    println!("Vox Server");

    let device = Box::new(ScienceFair128 {});
    let info = device.device_information();

    let serv = VoxServer {
        address: "0.0.0.0:1990".to_string(),
        device,
        renderer: Box::new(BaseRenderer {
            device_information: info
        }),
    };

    println!("Starting listener on {}", serv.address);

    serv.start_listener().await;
}
