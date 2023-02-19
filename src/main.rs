use relm4::{
    gtk::{
        self,
        gio::{self, Settings},
        prelude::*,
        traits::{DialogExt, FileChooserExt, TextBufferExt, WidgetExt},
        FileChooserDialog, TextBuffer,
    },
    ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent,
};

use std::{
    fs::File,
    io::{prelude::*, BufReader, BufWriter},
    process::exit,
};

struct AppModel {
    text: TextBuffer,
    settings: Settings,
}

#[derive(Debug)]
enum AppMsg {
    Open,
    Save,
    Quit,
    About,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = (TextBuffer, Settings);

    type Input = AppMsg;
    type Output = ();

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
                // Create a file chooser dialog
                let dialog = FileChooserDialog::new(
                    Some("Save file"),
                    Some(&gtk::Window::new()),
                    gtk::FileChooserAction::Save,
                    &[
                        ("Cancel", gtk::ResponseType::Cancel),
                        ("Save", gtk::ResponseType::Accept),
                    ],
                );

                // Grab the string from the text view
                let start = self.text.start_iter();
                let end = self.text.end_iter();
                let text = self.text.text(&start, &end, true);

                // Display the dialog
                dialog.set_transient_for(Some(&gtk::Window::new()));
                dialog.set_modal(true);
                dialog.present();

                // Get the result from the dialog
                dialog.connect_response(move |dialog, response| {
                    if response == gtk::ResponseType::Accept {
                        let file = &dialog.file().expect("File was not set");
                        let mut file = BufWriter::new(File::create(file.path().unwrap()).unwrap());
                        file.write_all(text.as_bytes()).unwrap();
                    }
                    dialog.close();
                });
            }
            AppMsg::Open => {
                // Create a file chooser dialog
                let dialog = FileChooserDialog::new(
                    Some("Open file"),
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
                        let file = &dialog.file().expect("File was not set");
                        let mut file = BufReader::new(File::open(file.path().unwrap()).unwrap());
                        let mut filetext = String::new();
                        file.read_to_string(&mut filetext).unwrap();
                        println!("{}", filetext);
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
        }
    }
}

fn main() {
    let app = RelmApp::new("dev.doodles.dnotes");
    let settings = Settings::new("dev.doodles.dnotes");

    app.run::<AppModel>((
        gtk::TextBuffer::new(Some(&gtk::TextTagTable::new())),
        settings,
    ));
}
