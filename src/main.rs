use gtk::{prelude::*, Application, ApplicationWindow};
use gtk::{Box, Button};
use std::process::Command;

fn main() {
    let application = Application::builder()
        .application_id("sigma.gtk.powerask")
        .build();
    application.connect_activate(|app| {
        let window = gtk::ApplicationWindowBuilder::default_height(
            ApplicationWindow::builder()
                .application(app)
                .title("Power Button Pressed"),
            200,
        )
        .default_width(200)
        .build();

        let gtk_box = Box::new(gtk::Orientation::Vertical, 10);

        let shutdown = Button::with_label("Shutdown");
        shutdown.connect_clicked(|_| {
            Command::new("bash")
                .arg("-c")
                .arg("loginctl poweroff")
                .output()
                .expect("Failed to Shutdown");
        });
        let logout = Button::with_label("Logout");
        logout.connect_clicked(|_| {
            Command::new("bash")
                .arg("-c")
                .arg("i3-msg exit")
                .output()
                .expect("Failed to logout");
        });
        let reboot = Button::with_label("Reboot");
        reboot.connect_clicked(|_| {
            Command::new("bash")
                .arg("-c")
                .arg("loginctl reboot")
                .output()
                .expect("Failed to Reboot");
        });
        let suspend = Button::with_label("Suspend");
        suspend.connect_clicked(|_| {
            Command::new("bash")
                .arg("-c")
                .arg("loginctl suspend")
                .output()
                .expect("Failed to Suspend");
        });
        gtk_box.add(&shutdown);
        gtk_box.add(&logout);
        gtk_box.add(&reboot);
        gtk_box.add(&suspend);
        window.add(&gtk_box);
        window.show_all();
    });
    application.run();
}
