mod file_item;
use file_item::FileItem;

use relm4::factory::FactoryVecDeque;
use relm4::gtk::gio::{self};
use relm4::gtk::prelude::*;
use relm4::prelude::*;

struct AppModel {
    text: gtk::TextBuffer,
    filename: gtk::TextBuffer,
    settings: gio::Settings,
    file_items: FactoryVecDeque<FileItem>,
}

#[derive(Debug)]
enum AppMsg {
    Open,
    Save,
    About,
    Update(String, String),
    OpenFile(DynamicIndex),
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Input = AppMsg;

    type Output = ();
    type Init = (gtk::TextBuffer, gtk::TextBuffer, gio::Settings);

    // Initialize the UI.
    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let file_items = FactoryVecDeque::new(gtk::Box::default(), sender.input_sender());
        let (text, filename, settings) = init;
        let model = AppModel {
            text,
            filename,
            settings,
            file_items,
        };
        let files_box = model.file_items.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        // Replace $HOME in the path with the user's home directory
        let path = self.settings.get::<String>("notes-location").replace(
            "/$HOME",
            &std::env::var("HOME").expect("Failed to get $HOME"),
        );
        let files = gio::File::for_path(path);

        // Remove the previously added files
        self.file_items.guard().clear();

        // Create a new file item for each file in the notes directory
        files
            .enumerate_children(
                &gio::FILE_ATTRIBUTE_STANDARD_NAME,
                gio::FileQueryInfoFlags::NONE,
                gio::Cancellable::NONE,
            )
            .expect("Failed to enumerate directory")
            .for_each(|file| {
                self.file_items
                    .guard()
                    .push_back(file.expect("Failed to get gio::File").name());
            });

        match msg {
            AppMsg::Save => {
                // Grab the filename from the text view
                let start_filename = self.filename.start_iter();
                let end_filename = self.filename.end_iter();
                let text_filename = self.filename.text(&start_filename, &end_filename, true);

                // Grab the text from the text view
                let start = self.text.start_iter();
                let end = self.text.end_iter();
                let text = self.text.text(&start, &end, true);

                // Retrieve the location setting
                // Replace $HOME in the path with the user's home directory
                let location = self.settings.get::<String>("notes-location").replace(
                    "/$HOME",
                    &std::env::var("HOME").expect("Failed to get $HOME"),
                );

                // Write the text to a gio::File
                let file = gio::File::for_path(location + &text_filename + ".md");
                let output_stream = file.replace(
                    None,
                    false,
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

                        // Grab the filename and content of the file
                        let filename = file
                            .basename()
                            .expect("Failed to get basename")
                            .to_str()
                            .expect("Failed to convert basename to str")
                            .to_string();
                        let (contents, _) = file
                            .load_contents(gio::Cancellable::NONE)
                            .expect("Failed to load gio::File");
                        let contents =
                            String::from_utf8(contents).expect("Failed to parse gio::File");

                        // Update the text view with the contents and filename of the file
                        _sender.input(AppMsg::Update(contents, filename));
                    }
                    dialog.close();
                });
            }
            AppMsg::About => {
                let dialog = gtk::AboutDialog::new();
                dialog.set_program_name(Some("dNotes"));
                dialog.set_version(Some(env!("CARGO_PKG_VERSION")));
                dialog.set_comments(Some(" A simple, free and open source cross platform note taking application. Developed with GTK 4."));
                dialog.set_website(Some("https://github.com/DoodlesEpic/dNotes"));
                dialog.set_authors(&["Doodles"]);
                dialog.set_license_type(gtk::License::Gpl30Only);
                dialog.set_transient_for(Some(&gtk::Window::new()));
                dialog.set_modal(true);
                dialog.present();
            }
            AppMsg::Update(text, filename) => {
                self.text.set_text(&text);
                self.filename.set_text(&filename);
            }
            AppMsg::OpenFile(index) => {
                let path = self
                    .file_items
                    .guard()
                    .get(index.current_index())
                    .expect("Failed to get FileItem")
                    .value
                    .clone();

                // Retrieve the location setting
                // Replace $HOME in the path with the user's home directory
                let location = self.settings.get::<String>("notes-location").replace(
                    "/$HOME",
                    &std::env::var("HOME").expect("Failed to get $HOME"),
                );

                let file = gio::File::for_path(
                    location + &path.to_str().expect("Failed to convert filename to String"),
                );

                // Grab the filename and content of the file
                let filename = file
                    .basename()
                    .expect("Failed to get basename")
                    .to_str()
                    .expect("Failed to convert basename to str")
                    .to_string()
                    .replace(".md", "");
                let (contents, _) = file
                    .load_contents(gio::Cancellable::NONE)
                    .expect("Failed to load gio::File");
                let contents = String::from_utf8(contents).expect("Failed to parse gio::File");

                // Update the text view with the contents and filename of the file
                _sender.input(AppMsg::Update(contents, filename));
            }
        }
    }

    view! {
        gtk::Window {
            set_title: Some("dNotes"),
            set_default_width: 600,
            set_default_height: 400,

            #[wrap(Some)]
            set_titlebar = &gtk::HeaderBar {
                pack_start = &gtk::Box {
                    gtk::Button::with_label("Save") {
                        set_icon_name: "document-save",
                        connect_clicked[sender] => move |_| {
                            sender.input(AppMsg::Save);
                        }
                    },

                    gtk::Button::with_label("Open") {
                        set_icon_name: "document-open",
                        connect_clicked[sender] => move |_| {
                            sender.input(AppMsg::Open);
                        }
                    },
                },
                pack_end = &gtk::Button::with_label("About") {
                    set_icon_name: "help-about",
                    connect_clicked[sender] => move |_| {
                        sender.input(AppMsg::About);
                    }
                },
            },

            gtk::Paned {
                set_orientation: gtk::Orientation::Horizontal,

                #[wrap(Some)]
                set_start_child = &gtk::ScrolledWindow {
                    #[local_ref]
                    files_box -> gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 10,
                        set_margin_all: 10,
                    }
                },


                #[wrap(Some)]
                set_end_child = &gtk::ScrolledWindow {
                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_hexpand: true,

                        gtk::TextView::with_buffer(&model.filename) {
                            set_bottom_margin: 10,
                            set_left_margin: 10,
                            set_right_margin: 10,
                            set_top_margin: 10,
                        },

                        gtk::Separator {},

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
}

fn main() {
    let app = RelmApp::new("dev.doodles.dnotes");
    let settings = gio::Settings::new("dev.doodles.dnotes");

    app.run::<AppModel>((
        gtk::TextBuffer::new(Some(&gtk::TextTagTable::new())),
        gtk::TextBuffer::new(Some(&gtk::TextTagTable::new())),
        settings,
    ));
}
