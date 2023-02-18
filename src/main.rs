use relm4::{
    gtk::{
        self,
        prelude::*,
        traits::{DialogExt, FileChooserExt, TextBufferExt, WidgetExt},
        FileChooserDialog, TextBuffer,
    },
    ComponentParts, ComponentSender, RelmApp, SimpleComponent,
};

use std::{
    fs::File,
    io::{prelude::*, BufWriter},
};

struct AppModel {
    text: TextBuffer,
}

#[derive(Debug)]
enum AppMsg {
    Save,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = TextBuffer;

    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("dNotes"),
            set_default_width: 600,
            set_default_height: 400,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                gtk::TextView::with_buffer(&model.text) {
                    set_vexpand: true,
                    set_bottom_margin: 10,
                    set_left_margin: 10,
                    set_right_margin: 10,
                    set_top_margin: 10,
                    set_wrap_mode: gtk::WrapMode::WordChar,
                },

                gtk::Button::with_label("Save") {
                    connect_clicked[sender] => move |_| {
                        sender.input(AppMsg::Save);
                    }
                },
            }
        }
    }

    // Initialize the UI.
    fn init(
        text: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel { text };

        // Insert the macro code generation here
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
        }
    }
}

fn main() {
    let app = RelmApp::new("dev.doodles.dnotes");
    app.run::<AppModel>(gtk::TextBuffer::new(Some(&gtk::TextTagTable::new())));
}
