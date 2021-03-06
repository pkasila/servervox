mod renderer;

use corevox::devices::device::Device;
use corevox::network::server::VoxServer;
use corevox::devices::science_fair_240::ScienceFair240;
use crate::renderer::BaseRenderer;

#[tokio::main]
pub async fn main() {
    println!("Vox Server");

    let device = Box::new(ScienceFair240 {});
    let info = device.device_information();

    let serv = VoxServer {
        address: "0.0.0.0:1990".to_string(),
        device,
        renderer: Box::new(BaseRenderer::new(info)),
    };

    println!("Starting listener on {}", serv.address);

    serv.start_listener().await;
}
