use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;
use corevox::network::messages::{DeviceInformation, VoxPack};
use corevox::network::server::renderer::Renderer;
use framebuffer::{Framebuffer, KdMode};
use rppal::gpio::Gpio;

pub struct BaseRenderer {
    pub device_information: DeviceInformation,
    gpio: Gpio,
}

impl BaseRenderer {
    pub fn new(device_information: DeviceInformation) -> Self {
        let mut r = BaseRenderer {
            device_information,
            gpio: Gpio::new().unwrap(),
        };

        return r;
    }
}

impl Renderer for BaseRenderer {
    fn handle_vox_pack(&self, pack: VoxPack) {
        println!("Received {} bytes", pack.raw.len());

        let mut t = Command::new("/bin/sh")
            .arg("-c")
            .arg("ffmpeg -f mp4 -c:v h264_v4l2m2m -i pipe: -pix_fmt bgra -f rawvideo pipe:")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        if let Some(mut stdin) = t.stdin.take() {
            stdin.write_all(&pack.raw).unwrap();
        }

        let mut data: Vec<u8> = vec![];
        t.stdout.unwrap().read_to_end(&mut data).unwrap();

        let mut fb = Framebuffer::new(Path::new("/dev/fb0")).unwrap();

        let chunk_size = (4 * self.device_information.vox_size[0] * self.device_information.vox_size[1]) as usize;
        let mut output_data: Vec<u8> = (*data).to_vec();
        // reversed order of frames
        for chunk in data.chunks(chunk_size).rev() {
            output_data.append(&mut chunk.to_vec());
        }

        // get backlight pin
        let mut pin = self.gpio.get(17).unwrap().into_output();

        // output for 30 seconds
        for _ in 0..30 {
            for _ in 0..self.device_information.pov_frequency / 2 {
                for chunk in output_data.chunks(chunk_size).step_by(2) {
                    // write to framebuffer
                    fb.write_frame(chunk);
                    // sleep for display panel refresh time
                    sleep(Duration::new(0, 10000000));
                    // turn on the backlight
                    pin.set_high();
                    // sleep for one layer time
                    sleep(Duration::new(0, (1000000000 / (self.device_information.pov_frequency * self.device_information.vox_size[2])) as u32));
                    // turn off the backlight
                    pin.set_low();
                    // sleep for one layer time excluding refresh time of the display panel
                    sleep(Duration::new(0, (1000000000 / (self.device_information.pov_frequency * self.device_information.vox_size[2]) - 9000000) as u32));
                }
            }
        }

        // enable screen after processing
        pin.set_high();
    }
}