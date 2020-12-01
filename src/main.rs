extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;

use gtk::{AboutDialog, Window, Builder};

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("gui.glade");
    let builder = Builder::from_string(glade_src);

    let window: Window = builder.get_object("main_window").expect("Couldn't get \"main_window\".");

    let about_dialog: AboutDialog = builder.get_object("about_dialog").expect("Couldn't get \"about_dialog\".");

    builder.connect_signals(move |_, handler_name| {
        if handler_name == "show_about_dialog" {
            Box::new(clone!(@weak about_dialog => @default-return None, move |_| {
                about_dialog.show_all();
                None
            }))
        } else {
            panic!("Unknown handler name {}", handler_name)
        }
    });

    window.set_application(Some(application));
    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.telmotrooper.citadel"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
