use std::fmt::Display;

mod modem;
pub use self::modem::*;

#[derive(Clone)]
pub struct ModemState {
	pub mcc: Option<i64>,
	pub mnc: Option<i64>,
	pub operator_name: Option<String>,
	pub rssi: Option<i64>,
	pub cme_error: Option<CMEError>,
	pub operator_mode: Option<COPSMode>,
	pub connection_type: Option<ConnectionType>,
}

#[derive(Clone)]
pub struct COPSMode(i64);

#[derive(Clone)]
pub struct CMEError(i64);

#[derive(Clone)]
pub struct LTEConnectionInfo {
	pub rssi: i64,
	pub rsrp: i64,
	pub rsrq: i64,
	pub sinr: i64,
}

#[derive(Clone)]
pub struct GSMConnectionInfo {
	pub rssi: i64,
}

#[derive(Clone)]
pub enum ConnectionType {
	GSM(GSMConnectionInfo),
	LTE(LTEConnectionInfo),
}

impl COPSMode {
	pub fn new(code: i64) -> COPSMode {
		COPSMode(code)
	}

	pub fn to_str(&self) -> &'static str {
		match self.0 {
			0 => "automatic",
			1 => "manual",
			2 => "deregister from network",
			_ => "unknown",
		}
	}
}

impl CMEError {
	pub fn new(code: i64) -> CMEError {
		CMEError(code)
	}

	pub fn to_str(&self) -> &'static str {
		match self.0 {
			0 => "phone failure",
			1 => "no connection to phone",
			2 => "phone adaptor link reserved",
			3 => "operation not allowed",
			4 => "operation not supported",
			5 => "PH-SIM PIN required",
			6 => "PH-FSIM PIN required",
			7 => "PH-FSIM PUK required",
			10 => "SIM not inserted",
			11 => "SIM PIN required",
			12 => "SIM PUK required",
			13 => "SIM failure",
			14 => "SIM busy",
			15 => "SIM wrong",

			30 => "no network service",
			31 => "network timeout",
			32 => "network not allowed - emergency calls only",

			50 => "Incorrect parameters",
			_ => "(unknown CME error)",
		}
	}
}

impl ModemState {
	pub fn new() -> ModemState {
		ModemState {
			mcc: None,
			mnc: None,
			operator_name: None,
			rssi: None,
			cme_error: None,
			operator_mode: None,
			connection_type: None,
		}
	}
}

impl LTEConnectionInfo {
	// lte_rssi: 0 = -120 dBm, 96 = -25 dBm, 255 = unknown/undetectable
	pub fn rssi_dbm(&self) -> String {
		if self.rssi == 255 {
			"unknown".to_owned()
		} else {
			format!("{} dBm", -120 + self.rssi)
		}
	}

	// lte_rsrp: 0 = -140 dBm, 97 = -44 dBm, 255 = unknown/undetectable
	pub fn rsrp_dbm(&self) -> String {
		if self.rsrp == 255 {
			"unknown".to_owned()
		} else {
			format!("{} dBm", -140 + self.rssi)
		}
	}
}

impl Display for LTEConnectionInfo {
	// lte_rsrp: 0 = -140 dBm, 97 = -44 dBm, 255 = unknown/undetectable
	// lte_sinr: 0 = -20 dB, 251 = -30 dB, 255 = unknown/undetectable
	// lte_rsrq: 0 = -19.5 dB, 34 = -3 dB, 255 = unknown/undetectable
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(
			f,
			"rssi={}, rsrq={}, rsrp={}, sinr={}",
			self.rssi_dbm(),
			self.rsrq,
			self.rsrp_dbm(),
			self.sinr
		)
	}
}

impl Display for GSMConnectionInfo {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "rssi={}", self.rssi)
	}
}

impl Display for ModemState {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let unknown_operator_name = String::from("?");

		let operator_name = match &self.operator_name {
			Some(opr) => opr,
			_ => &unknown_operator_name,
		};

		let operator_mode = match &self.operator_mode {
			Some(opr) => opr.to_str(),
			_ => "unknown",
		};

		let error_string: &'static str = match &self.cme_error {
			Some(err) => err.to_str(),
			_ => "no error",
		};

		let conn_type = match &self.connection_type {
			Some(ConnectionType::LTE(lte_info)) => format!("LTE ({})", lte_info),
			Some(ConnectionType::GSM(gsm_info)) => format!("GSM ({})", gsm_info),
			None => "Unknown".to_owned(),
		};

		write!(
			f,
			"operator={} ({}, {}) mode={} conn={} err={}",
			operator_name,
			self.mcc.unwrap_or(0),
			self.mnc.unwrap_or(0),
			operator_mode,
			conn_type,
			error_string
		)
	}
}
