use spidev::{Spidev, SpidevOptions, SpiModeFlags};
use spidev::spidevioctl::SpidevTransfer;
use std::thread;
use std::time::Duration;
// use std::fs::File;
// use std::io::Write;
use gpio::{Gpio, Pin};
use gpio::PinMode::Output;
use gpio::PinValue::Low;

pub mod gpio;

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

	let gpio0 = Gpio::open(0);
	let clk = Pin::open(&gpio0, 36);
	clk.mode(Output);
	clk.digital_write(Low);

	// let mut f = match File::create("log.txt") {
	// 	Ok(f) => f,
	// 	Err(e) => {
	// 		panic!("{}", e);
	// 	}
	// };

	loop {
		readspi2([0x1C, 0x00], &spi, String::from("TEMP"));
		readspi2([0x1E, 0x00], &spi, String::from("TIME"));

		readspi([0x04, 0x00, 0x06, 0x00], &spi, String::from("X_GYRO"));
		readspi([0x08, 0x00, 0x0A, 0x00], &spi, String::from("Y_GYRO"));
		readspi([0x0C, 0x00, 0x0E, 0x00], &spi, String::from("Z_GYRO"));

		readspi([0x10, 0x00, 0x12, 0x00], &spi, String::from("X_ACCL"));
		readspi([0x14, 0x00, 0x16, 0x00], &spi, String::from("Y_ACCL"));
		readspi([0x18, 0x00, 0x1A, 0x00], &spi, String::from("Z_ACCL"));

		readspi([0x24, 0x00, 0x26, 0x00], &spi, String::from("X_DELTANG"));
		readspi([0x28, 0x00, 0x2A, 0x00], &spi, String::from("Y_DELTANG"));
		readspi([0x2C, 0x00, 0x2E, 0x00], &spi, String::from("Z_DELTANG"));

		readspi([0x30, 0x00, 0x32, 0x00], &spi, String::from("X_DELTVEL"));
		readspi([0x34, 0x00, 0x36, 0x00], &spi, String::from("Y_DELTVEL"));
		readspi([0x38, 0x00, 0x3A, 0x00], &spi, String::from("Z_DELTVEL"));

		thread::sleep(Duration::from_secs(2));
	}
}

fn readspi(tx_buf: [u8; 4], spi: &Spidev, s: String) {
	let mut rx_buf = [0; 4];
	let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
    let result = spi.transfer(&mut transfer);
	match result {
        Ok(_) => {
            println!("{s}: {:?}", rx_buf);
        }
        Err(err) => println!("{:?}", err),
    }

	// f.write_all(&rx_buf).expect("Failed writing to file");
}

fn readspi2(tx_buf: [u8; 2], spi: &Spidev, s: String) {
	let mut rx_buf = [0; 2];
	let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
    let result = spi.transfer(&mut transfer);
	match result {
        Ok(_) => {
            println!("{s}: {:?}", rx_buf);
        }
        Err(err) => println!("{:?}", err),
    }

	// f.write_all(&rx_buf).expect("Failed writing to file");
}