use gtk::prelude::*;
use gtk::Box;
use gtk::Button;
use gtk::Label;
use gtk::Orientation;
use gtk::{glib, Application, ApplicationWindow};

const APP_ID: &str = "dev.doodles.dnotes";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a title
    let title = Label::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .label("dNotes")
        .can_focus(false)
        .build();

    // Create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let app_box = Box::builder().orientation(Orientation::Vertical).build();
    app_box.append(&title);
    app_box.append(&button);

    // Connect to "clicked" signal of `button`
    button.connect_clicked(|button| match button.label() {
        Some(label) => match label.as_str() {
            "Press me!" => {
                button.set_label("Pressed!");
            }
            "Pressed!" => {
                button.set_label("Press me again!");
            }
            _ => {
                button.set_label("Press me!");
            }
        },
        None => println!("No label"),
    });

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("dNotes")
        .child(&app_box)
        .build();

    // Present window
    window.present();
}
