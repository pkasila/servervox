mod renderer;

use corevox::devices::device::Device;
use corevox::network::server::VoxServer;
use corevox::devices::science_fair_128::ScienceFair128;
use crate::renderer::BaseRenderer;

fn main() {
    println!("Vox Server");

    let device = Box::new(ScienceFair128 {});

    let serv = VoxServer {
        address: "0.0.0.0:1990".to_string(),
        device,
        renderer: Box::new(BaseRenderer {
            device_information: device.device_information()
        }),
    };

    println!("Starting listener on {}", serv.address);

    serv.start_listener().unwrap();
}
