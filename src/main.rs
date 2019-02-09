use regex::Regex;
use std::error::Error;
use std::io::{self, Read};
use std::str;
use std::time::Duration;

use std::sync::mpsc::channel;
use std::{thread, time};

use clap::{App, AppSettings, Arg};
use serialport::prelude::*;

mod at;
use at::{CMEError, COPSMode, ConnectionType, GSMConnectionInfo, LTEConnectionInfo, ModemState};

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

	let (tx, rx) = channel::<ModemState>();
	thread::spawn(move || {
		let exit_code = match run(&port_name, &baud_rate, &tx) {
			Ok(_) => 0,
			Err(e) => {
				println!("Error: {}", e);
				1
			}
		};

		std::process::exit(exit_code);
	});

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

fn run(
	port_name: &str,
	baud_rate: &str,
	sender: &std::sync::mpsc::Sender<ModemState>,
) -> Result<(), Box<Error>> {
	let mut settings: SerialPortSettings = Default::default();
	settings.timeout = Duration::from_millis(10);

	let rate = baud_rate
		.parse::<u32>()
		.map_err(|_| format!("Invalid baud rate '{}' specified", baud_rate))?;
	settings.baud_rate = rate.into();

	match serialport::open_with_settings(&port_name, &settings) {
		Ok(mut port) => {
			let mut serial_buf: Vec<u8> = vec![0; 1000];
			println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
			let mut current_str: String = String::new();

			let rssi = Regex::new(r"^\^RSSI:(\d+)$").unwrap();
			let hcsq_lte = Regex::new("^\\^HCSQ:\"(LTE)\",(\\d+),(\\d+),(\\d+),(\\d+)$").unwrap();
			let hcsq_gsm = Regex::new("^\\^HCSQ:\"(GSM)\",(\\d+)$").unwrap();

			// ^PLMN: 204,04
			let plmn = Regex::new(r"^\^PLMN: (\d+),(\d+)$").unwrap();

			// +CME ERROR: 50
			let cme_error = Regex::new(r"^\+CME ERROR: (\d+)$").unwrap();

			// +COPS: 0,0,"vodafone NL",7
			// mode, format, oper, AcT
			let cops = Regex::new("^\\+COPS: (\\d+),(\\d+),\"([a-zA-Z0-9 ]+)\",(\\d+)$").unwrap();

			port.write("AT\r\n".as_bytes()).expect("write AT command");
			let mut state = ModemState::new();

			loop {
				match port.read(serial_buf.as_mut_slice()) {
					Ok(t) => {
						let input = str::from_utf8(&serial_buf[..t]).unwrap();
						current_str += &input;

						let mut last: &str = "";
						for s in current_str.split("\r\n") {
							if s.len() > 0 {
								last = "";
								if s == "OK" {
									let wait_duration = time::Duration::from_millis(1000);
									thread::sleep(wait_duration);
									// In response to "AT" command
									port.write("AT+COPS?\r\n".as_bytes())
										.expect("write AT+COPS? command");
								} else if plmn.is_match(s) {
									let caps = plmn.captures(s).unwrap();
									state.mcc =
										Some(caps.get(1).unwrap().as_str().parse().unwrap());
									state.mnc =
										Some(caps.get(2).unwrap().as_str().parse().unwrap());
									port.write("AT^HCSQ?\r\n".as_bytes())
										.expect("write AT^HCSQ command");
								} else if cme_error.is_match(s) {
									let caps = cme_error.captures(s).unwrap();
									state.cme_error = Some(CMEError::new(
										caps.get(1).unwrap().as_str().parse().unwrap(),
									));
								} else if cops.is_match(s) {
									let caps = cops.captures(s).unwrap();
									state.operator_mode = Some(COPSMode::new(
										caps.get(1).unwrap().as_str().parse().unwrap(),
									));
									state.operator_name =
										Some(String::from(caps.get(3).unwrap().as_str()));
									port.write("AT^PLMN?\r\n".as_bytes())
										.expect("write AT^PLMN command");
								} else if rssi.is_match(s) {
									let caps = rssi.captures(s).unwrap();
									state.rssi =
										Some(caps.get(1).unwrap().as_str().parse().unwrap());
								} else if hcsq_lte.is_match(s) {
									let caps = hcsq_lte.captures(s).unwrap();
									state.connection_type =
										Some(ConnectionType::LTE(LTEConnectionInfo {
											rssi: caps.get(2).unwrap().as_str().parse().unwrap(),
											rsrp: caps.get(3).unwrap().as_str().parse().unwrap(),
											sinr: caps.get(4).unwrap().as_str().parse().unwrap(),
											rsrq: caps.get(5).unwrap().as_str().parse().unwrap(),
										}));
								} else if hcsq_gsm.is_match(s) {
									let caps = hcsq_gsm.captures(s).unwrap();
									state.connection_type =
										Some(ConnectionType::GSM(GSMConnectionInfo {
											rssi: caps.get(2).unwrap().as_str().parse().unwrap(),
										}));
								} else {
									eprintln!("Unknown command: {:?}", s);
									last = s;
									break;
								}
							} else {
								last = ""
							}
						}

						sender.send(state.clone()).expect("send state");
						current_str = last.to_string()
					}
					Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
					Err(e) => eprintln!("~~{:?}~~", e),
				}
			}
		}
		Err(e) => {
			eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
			::std::process::exit(1);
		}
	}
}
