use std::process::exit;

use relm4::gtk::gio::{self};
use relm4::gtk::prelude::*;
use relm4::prelude::*;
struct AppModel {
    text: gtk::TextBuffer,
    settings: gio::Settings,
}

#[derive(Debug)]
enum AppMsg {
    Open,
    Save,
    Quit,
    About,
    Update(String),
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Input = AppMsg;

    type Output = ();
    type Init = (gtk::TextBuffer, gio::Settings);

    // Initialize the UI.
    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let (text, settings) = init;
        let model = AppModel { text, settings };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::Save => {
                // Grab the string from the text view
                let start = self.text.start_iter();
                let end = self.text.end_iter();
                let text = self.text.text(&start, &end, true);

                // Retrieve the location setting
                let location = self.settings.get::<String>("notes-location");

                // Write the text to a gio::File
                let file = gio::File::for_path(location);
                let output_stream = file.append_to(
                    gio::FileCreateFlags::REPLACE_DESTINATION,
                    gio::Cancellable::NONE,
                );
                output_stream
                    .expect("Failed to open gio::File")
                    .write_all(text.as_bytes(), gio::Cancellable::NONE)
                    .expect("Failed to write to gio::File");
            }
            AppMsg::Open => {
                // Create a gio::File chooser dialog
                let dialog = gtk::FileChooserDialog::new(
                    Some("Open gio::File"),
                    Some(&gtk::Window::new()),
                    gtk::FileChooserAction::Open,
                    &[
                        ("Cancel", gtk::ResponseType::Cancel),
                        ("Open", gtk::ResponseType::Accept),
                    ],
                );

                // Display the dialog
                dialog.set_transient_for(Some(&gtk::Window::new()));
                dialog.set_modal(true);
                dialog.present();

                // Get the result from the dialog
                dialog.connect_response(move |dialog, response| {
                    if response == gtk::ResponseType::Accept {
                        let file = &dialog.file().expect("gio::File was not set");
                        let file_path = file.path().expect("gio::File path was not set");
                        let file = gio::File::for_path(file_path);

                        let (contents, _) = file
                            .load_contents(gio::Cancellable::NONE)
                            .expect("Failed to load gio::File");
                        let string =
                            String::from_utf8(contents).expect("Failed to parse gio::File");
                        _sender.input(AppMsg::Update(string));
                    }
                    dialog.close();
                });
            }
            AppMsg::Quit => exit(0),
            AppMsg::About => {
                let dialog = gtk::AboutDialog::new();
                dialog.set_program_name(Some("dNotes"));
                dialog.set_version(Some("0.2.1"));
                dialog.set_comments(Some(" A simple, free and open source cross platform note taking application. Developed with GTK 4."));
                dialog.set_website(Some("https://github.com/DoodlesEpic/dNotes"));
                dialog.set_authors(&["Doodles"]);
                dialog.set_license_type(gtk::License::Gpl30Only);
                dialog.set_transient_for(Some(&gtk::Window::new()));
                dialog.set_modal(true);
                dialog.present();
            }
            AppMsg::Update(text) => {
                self.text.set_text(&text);
            }
        }
    }

    view! {
        gtk::Window {
            set_title: Some("dNotes"),
            set_default_width: 600,
            set_default_height: 400,

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_margin_all: 10,
                    set_spacing: 10,

                    gtk::Button::with_label("Save") {
                        connect_clicked[sender] => move |_| {
                            sender.input(AppMsg::Save);
                        }
                    },

                    gtk::Button::with_label("Open") {
                        connect_clicked[sender] => move |_| {
                            sender.input(AppMsg::Open);
                        }
                    },

                    gtk::Button::with_label("Quit") {
                        connect_clicked[sender] => move |_| {
                            sender.input(AppMsg::Quit);
                        }
                    },

                    gtk::Button::with_label("About") {
                        connect_clicked[sender] => move |_| {
                            sender.input(AppMsg::About);
                        }
                    }
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_hexpand: true,

                    gtk::TextView::with_buffer(&model.text) {
                        set_vexpand: true,
                        set_bottom_margin: 10,
                        set_left_margin: 10,
                        set_right_margin: 10,
                        set_top_margin: 10,
                        set_wrap_mode: gtk::WrapMode::WordChar,
                    },
                }
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("dev.doodles.dnotes");
    let settings = gio::Settings::new("dev.doodles.dnotes");

    app.run::<AppModel>((
        gtk::TextBuffer::new(Some(&gtk::TextTagTable::new())),
        settings,
    ));
}
