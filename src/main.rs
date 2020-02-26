#![windows_subsystem = "windows"]

use std::env;
use sunrs::messages::{SetColorMessage, SetPowerMessage};
use sunrs::net::send_message;
use sunrs::scenes::{BRIGHT, CHILL, COMPUTER, DARK, DAYLIGHT, READING};

fn ok() -> Result<(), systray::Error> {
    Ok::<_, systray::Error>(())
}

#[cfg(target_os = "windows")]
fn main() -> Result<(), systray::Error> {
    match systray::Application::new() {
        Err(_) => panic!("Can't create window!"),
        Ok(mut app) => {
            let local_app_data = env::var("LOCALAPPDATA").expect("Can't get %LOCALAPPDATA%");

            app.set_icon_from_file(&format!(
                r"{}\Programs\sunrs-tray\sunrs-tray.ico",
                local_app_data
            ))?;

            app.set_tooltip("Sunrs")?;

            app.add_menu_item("On", |_| {
                send_message(SetPowerMessage::new_on_message());
                ok()
            })?;

            app.add_menu_item("Off", |_| {
                send_message(SetPowerMessage::new_off_message());
                ok()
            })?;

            app.add_menu_separator()?;

            app.add_menu_item("Daylight", |_| {
                send_message(SetColorMessage::new_scene(DAYLIGHT));
                send_message(SetPowerMessage::new_on_message());
                ok()
            })?;

            app.add_menu_item("Warm Bright", |_| {
                send_message(SetColorMessage::new_scene(BRIGHT));
                send_message(SetPowerMessage::new_on_message());
                ok()
            })?;

            app.add_menu_item("Computer", |_| {
                send_message(SetColorMessage::new_scene(COMPUTER));
                send_message(SetPowerMessage::new_on_message());
                ok()
            })?;

            app.add_menu_item("Reading", |_| {
                send_message(SetColorMessage::new_scene(READING));
                send_message(SetPowerMessage::new_on_message());
                ok()
            })?;

            app.add_menu_item("Chill", |_| {
                send_message(SetColorMessage::new_scene(CHILL));
                send_message(SetPowerMessage::new_on_message());
                ok()
            })?;

            app.add_menu_item("Dark", |_| {
                send_message(SetColorMessage::new_scene(DARK));
                send_message(SetPowerMessage::new_on_message());
                ok()
            })?;

            app.add_menu_separator()?;

            app.add_menu_item("Quit", |window| {
                window.quit();
                ok()
            })?;

            app.wait_for_message()?;
            ok()
        }
    }
}
