mod renderer;

use corevox::network::server::VoxServer;
use corevox::devices::science_fair_128::ScienceFair128;
use crate::renderer::BaseRenderer;

fn main() {
    println!("Vox Server");

    let serv = VoxServer {
        address: "0.0.0.0:1990".to_string(),
        device: Box::new(ScienceFair128 {}),
        renderer: Box::new(BaseRenderer { device_information: None }),
    };

    println!("Starting listener on {}", serv.address);

    serv.start_listener().unwrap();
}
