use std::io::Write;
use std::process::{Child, Command, Stdio};
use std::thread;
use corevox::devices::device::Device;
use corevox::network::messages::{DeviceInformation, VoxPack};
use corevox::network::server::renderer::Renderer;

pub struct BaseRenderer {
    pub device_information: DeviceInformation,
    ffmpeg_process: Option<Child>,
}

impl BaseRenderer {
    pub fn new(device_information: DeviceInformation) -> Self {
        let mut r = BaseRenderer {
            device_information,
            ffmpeg_process: None,
        };

        r.start_daemon();

        return r;
    }

    fn start_daemon(&mut self) {
        self.ffmpeg_process = match Command::new("/bin/sh")
            .arg("-c")
            .arg(format!("ffmpeg -re -f rawvideo -pix_fmt rgb565le -s:v {}x{} -r 168 -i pipe: -f fbdev /dev/fb0",
                         self.device_information.frame_size[0], self.device_information.frame_size[1]))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn() {
            Err(why) => panic!("couldn't spawn ffmpeg: {}", why),
            Ok(process) => Some(process),
        };
    }
}

impl Renderer for BaseRenderer {
    fn handle_vox_pack(&self, pack: VoxPack) {
        let mut p = pack;
        let framerate = p.z * self.device_information.pov_frequency;
        println!("Received {} bytes (framerate needed: {})", p.raw.capacity(), framerate);
        for i in 0..24 {
            self.ffmpeg_process.as_ref().unwrap().stdin.as_ref()
                .unwrap().write(&*p.raw);
        }
    }
}