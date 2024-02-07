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

	// read from X_GYRO_LOW (0x04)
	// 0x8000 | (reg_addr << 8); reg_addr = 0x04
	let tx_buf1 = [0x8400];
	let mut rx_buf1 = [0; 1];

	// read from X_GYRO_OUT (0x06)
	let tx_buf2 = [0x8600];
	let mut rx_buf2 = [0; 1];

	// SPI
	let result = {
		let mut transfers = vec![
			SpidevTransfer::write(&tx_buf1),
			SpidevTransfer::read(&mut rx_buf1),
			SpidevTransfer::write(&tx_buf2),
			SpidevTransfer::read(&mut rx_buf2),
		];
		spidev.transfer_multiple(&mut transfers)
	};

	// final value = (high<<16) | low
	match result {
		Ok(_) => {
			let result = rx_buf1 | (rx_buf2 << 16);
			println!("{:?} received", result);
		}
		Err(err) => println!("{:?}", err),
	}
}