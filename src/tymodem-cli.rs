use clap::{App, AppSettings, Arg};

mod at;
use at::{Modem, CMEError, COPSMode, ConnectionType, GSMConnectionInfo, LTEConnectionInfo, ModemState};

fn main() {
	let matches = App::new("TyModem")
		.about("Report Huawei modem status")
		.setting(AppSettings::DisableVersion)
		.arg(
			Arg::with_name("port")
				.help("The device path to a serial port")
				.use_delimiter(false)
				.default_value("/dev/cu.HUAWEIMobile-Pcui")
				.required(true),
		)
		.arg(
			Arg::with_name("baud")
				.help("The baud rate to connect at")
				.use_delimiter(false)
				.default_value("115200")
				.required(true),
		)
		.get_matches();

	let port_name = matches.value_of("port").unwrap().to_owned();
	let baud_rate = matches.value_of("baud").unwrap().to_owned();

	let modem = Modem::new(port_name, baud_rate);

	let rx = modem.run();

	// Print data received
	loop {
		match rx.recv() {
			Ok(state) => {
				eprintln!("State: {}", state);
			}
			Err(err) => {
				eprintln!("Error: {}", err);
			}
		};
	}
}