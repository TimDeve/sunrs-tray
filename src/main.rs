#![windows_subsystem = "windows"]

use std::env;
use sunrs::messages::{Command, SetColorMessage, SetPowerMessage};
use sunrs::net::send_message;

fn ok() -> Result<(), systray::Error> {
    Ok::<_, systray::Error>(())
}

#[cfg(target_os = "windows")]
fn main() -> Result<(), systray::Error> {
    let mut app = systray::Application::new().expect("Can't create window!");

    let local_app_data = env::var("LOCALAPPDATA").expect("Can't get %LOCALAPPDATA%");

    app.set_icon_from_file(&format!(
        r"{}\Programs\sunrs-tray\sunrs-tray.ico",
        local_app_data
    ))?;

    let config = sunrs::config::Config::from_default_file();

    app.set_tooltip("Sunrs")?;

    let config_clone = config.clone();
    app.add_menu_item("On", move |_| {
        send_on_message(config_clone.bulbs_addresses());
        ok()
    })?;

    let config_clone = config.clone();
    app.add_menu_item("Off", move |_| {
        send_off_message(config_clone.bulbs_addresses());
        ok()
    })?;

    app.add_menu_separator()?;

    for (i, scene) in config.scenes.iter().enumerate() {
        let config_clone = config.clone();
        app.add_menu_item(&scene.name, move |_| {
            let commands = config_clone
                .scene_by_index(i)
                .expect("Should have matching scene as index pulled from iter");
            send_scene_messages(commands);
            ok()
        })?;
    }

    app.add_menu_separator()?;

    app.add_menu_item("Quit", |window| {
        window.quit();
        ok()
    })?;

    app.wait_for_message()?;
    ok()
}

fn send_on_message(addresses: Vec<[u8; 8usize]>) {
    for address in addresses.iter() {
        send_message(SetPowerMessage::new_on_message(*address))
    }
}

fn send_off_message(addresses: Vec<[u8; 8usize]>) {
    for address in addresses.iter() {
        send_message(SetPowerMessage::new_off_message(*address))
    }
}

fn send_scene_messages(scene: Vec<Command>) {
    for command in scene.iter() {
        send_message(SetColorMessage::new_scene(*command));
        send_message(SetPowerMessage::new_on_message(command.mac_address))
    }
}
