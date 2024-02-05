use spidev::{SpiModeFlags, Spidev, SpidevOptions};

fn main() {
	let mut spi = Spidev::open("/dev/spidev0.0").unwrap();
	let imu_options = SpidevOptions::new()
		.lsb_first(false)
		.bits_per_word(16)
		.max_speed_hz(2_000_000)
		.mode(SpiModeFlags::SPI_MODE_3)
		.build();

	spi.configure(&imu_options)
		.expect("failed to configure SPI for the IMU");

	println!("Hello, space!");
}
