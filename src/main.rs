use spidev::{Spidev, SpidevOptions, SpiModeFlags};
use spidev::spidevioctl::SpidevTransfer;
use std::thread;
use std::time::Duration;
use std::io::prelude::*;

fn main() {
	let mut imu = Spidev::open("/dev/spidev0.0").unwrap();
	let imu_options = SpidevOptions::new()
		.lsb_first(false)
		.bits_per_word(16)
		.max_speed_hz(2_000_000)
		.mode(SpiModeFlags::SPI_MODE_3)
		.build();
	imu.configure(&imu_options)
		.expect("failed to configure SPI for the IMU");

	loop {
		// read_imu(&imu);

		// let mut rx_buf = [0_u8; 2];
		// imu.write(&[0x0C, 0x0E]).expect("Could not write tx buffer");
		// imu.read(&mut rx_buf).expect("Could not read rx buffer");
		// if rx_buf == [0_u8; 2] {
		// 	println!("Failed to retrieve information over SPI");
		// } else {
		// 	println!("{:?}", rx_buf);
		// }
		let tx_buf = [0x04, 0x00, 0x06, 0x00];
		let mut rx_buf = [0; 4];
		{
			let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
			imu.transfer(&mut transfer).expect("Could not transfer over SPI");
		}
		println!("{:?}", rx_buf);

		thread::sleep(Duration::from_secs(2));
	}
}

fn read_imu(imu: &Spidev) {
	// let mut rx_buf = [0_u8; 4];
    // imu.write(&[0x01, 0x02, 0x03])?;
    // imu.read(&mut rx_buf)?;
    // println!("{:?}", rx_buf);

	read_spi2([0x1C, 0x00], &imu, String::from("TEMP"));
	// read_spi2([0x1E, 0x00], &imu, String::from("TIME"));

	// read_spi4([0x04, 0x00, 0x06, 0x00], &imu, String::from("X_GYRO"));
	// read_spi4([0x08, 0x00, 0x0A, 0x00], &imu, String::from("Y_GYRO"));
	// read_spi4([0x0C, 0x00, 0x0E, 0x00], &imu, String::from("Z_GYRO"));

	// read_spi4([0x10, 0x00, 0x12, 0x00], &imu, String::from("X_ACCL"));
	// read_spi4([0x14, 0x00, 0x16, 0x00], &imu, String::from("Y_ACCL"));
	// read_spi4([0x18, 0x00, 0x1A, 0x00], &imu, String::from("Z_ACCL"));

	// read_spi4([0x24, 0x00, 0x26, 0x00], &imu, String::from("X_DELTANG"));
	// read_spi4([0x28, 0x00, 0x2A, 0x00], &imu, String::from("Y_DELTANG"));
	// read_spi4([0x2C, 0x00, 0x2E, 0x00], &imu, String::from("Z_DELTANG"));

	// read_spi4([0x30, 0x00, 0x32, 0x00], &imu, String::from("X_DELTVEL"));
	// read_spi4([0x34, 0x00, 0x36, 0x00], &imu, String::from("Y_DELTVEL"));
	// read_spi4([0x38, 0x00, 0x3A, 0x00], &imu, String::from("Z_DELTVEL"));
}

fn read_spi4(tx_buf: [u8; 4], spi: &Spidev, s: String) {
	let mut rx_buf = [0; 4];
	let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
    let result = spi.transfer(&mut transfer);
	match result {
        Ok(_) => {
            println!("{s}: {:?}", rx_buf);
        }
        Err(err) => println!("{:?}", err),
    }
}

fn read_spi2(tx_buf: [u8; 2], spi: &Spidev, s: String) {
	let mut rx_buf = [0; 2];
	let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
    let result = spi.transfer(&mut transfer);
	match result {
        Ok(_) => {
            println!("{s}: {:?}", rx_buf);
        }
        Err(err) => println!("{:?}", err),
    }
}