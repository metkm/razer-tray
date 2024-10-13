#![windows_subsystem = "windows"]

mod battery;

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use battery::get_battery;
use image::GenericImageView;
use tao::event_loop::{ControlFlow, EventLoop};
use tray_icon::{menu::MenuEvent, TrayIconBuilder};

fn main() {
    let icon = include_bytes!("../trayicon.jpg");

    let img = image::load_from_memory(icon).unwrap();
    let img_raw = img.to_rgba8().into_raw();
    let (img_w, img_h) = img.dimensions();

    let quit_item = tray_icon::menu::MenuItem::with_id("quit", "quit", true, None);
    let menu = tray_icon::menu::Menu::with_items(&[&quit_item]).unwrap();

    let tray_icon = TrayIconBuilder::new()
        .with_tooltip("Razer Tray")
        .with_menu(Box::new(menu))
        .with_icon(tray_icon::Icon::from_rgba(img_raw, img_w, img_h).unwrap())
        .build()
        .unwrap();

    let event_loop = EventLoop::new();

    let battery_level = Arc::new(Mutex::new(get_battery().unwrap() as u8));
    let is_battery_level_changed = Arc::new(Mutex::new(true));

    let battery_level_arc_2 = battery_level.clone();
    let is_battery_level_changed_arc_2= is_battery_level_changed.clone();

    std::thread::spawn(move || loop {
        let battery = get_battery().unwrap() as u8;
        let mut battery_level = battery_level.lock().unwrap();

        if battery != *battery_level {
            *battery_level = battery as u8;
            *is_battery_level_changed.lock().unwrap() = true;
        }

        std::thread::sleep(Duration::from_secs(2));
    });

    event_loop.run(move |_, _, control_flow| {
        let target = std::time::Instant::now() + std::time::Duration::from_secs(10); 
        *control_flow = ControlFlow::WaitUntil(target);

        if let Ok(MenuEvent { id }) = MenuEvent::receiver().try_recv() {
            let quit_id = quit_item.id();

            if id == quit_id {
                std::process::exit(0);
            }
        }

        if let Ok(mut is_changed) = is_battery_level_changed_arc_2.try_lock() {
            if *is_changed {
                if let Ok(level) = battery_level_arc_2.try_lock() {
                    
                    let text = match *level {
                        0 => {
                            format!("Battery Level {}% (Disconnected?)", *level)
                        },
                        _ => {
                            format!("Battery Level {}%", *level)
                        }
                    };

                    tray_icon.set_tooltip(Some(text)).ok();
                    *is_changed = false;
                }
            }
        }
    });
}
