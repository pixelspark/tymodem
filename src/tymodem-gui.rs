mod at;

use clap::{App, AppSettings, Arg};
use iui::prelude::*;
use iui::controls::{Label, Button, VerticalBox, Group};
use at::{Modem, ConnectionType};
use std::time;

fn main() {
	// Parse cmd line arguments
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

	let ui = UI::init().expect("Couldn't initialize UI library");
	let mut win = Window::new(&ui, "Modem status", 200, 200, WindowType::NoMenubar);

	// Create a vertical layout to hold the controls
	let mut vbox = VerticalBox::new(&ui);
	vbox.set_padded(&ui, true);

	let mut quit_button = Button::new(&ui, "Quit");
	quit_button.on_clicked(&ui, {
		let ui = ui.clone();
		move |_| {
			ui.quit();
		}
	});

	// Connection details
	let mut connection_group = Group::new(&ui, "Connection");
	let mut connection_group_vbox = VerticalBox::new(&ui);
	let operator_name = Label::new(&ui, "");
	let connection_type = Label::new(&ui, "");
	connection_group_vbox.append(&ui, operator_name.clone(), LayoutStrategy::Stretchy);
	connection_group_vbox.append(&ui, connection_type.clone(), LayoutStrategy::Stretchy);
	connection_group.set_child(&ui, connection_group_vbox);
	vbox.append(&ui, connection_group, LayoutStrategy::Compact);
	vbox.append(&ui, quit_button, LayoutStrategy::Compact);

	win.set_child(&ui, vbox);
	win.show(&ui);

	// Set up modem reader
	let port_name = matches.value_of("port").unwrap().to_owned();
	let baud_rate = matches.value_of("baud").unwrap().to_owned();

	let modem = Modem::new(port_name, baud_rate);

	let rx = modem.run();
	
	let mut event_loop = ui.event_loop();
	
	event_loop.on_tick(&ui, {
		let ui = ui.clone();
		let mut operator_name_label = operator_name.clone();
		let mut connection_type_label = connection_type.clone();
		let mut last_update = time::Instant::now();

		move || {
			if last_update.elapsed() > time::Duration::from_millis(1000) {
				eprintln!("on_tick");
				last_update = time::Instant::now();
				match rx.try_recv() {
					Ok(state) => {
						let operator_name = match &state.operator_name {
							Some(operator_name) => operator_name.clone(),
							None => "Unknown".to_owned()
						};

						let connection_type = match &state.connection_type {
							Some(ConnectionType::GSM(gi)) => "GSM".to_owned(),
							Some(ConnectionType::LTE(gi)) => "LTE".to_owned(),
							None => "".to_owned()
						};

						operator_name_label.set_text(&ui, operator_name.as_str());
						connection_type_label.set_text(&ui, connection_type.as_str());
						eprintln!("Updated: {}", &state);
					},
					Err(_err) => {
					}
				}
			}
		}
	});
	
	event_loop.run(&ui);
}