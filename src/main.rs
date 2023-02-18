use gtk::prelude::*;
use relm4::{send, AppUpdate, Model, RelmApp, Sender, WidgetPlus, Widgets};

#[derive(Default)]
struct AppModel {
    text: gtk::TextBuffer,
    hidden: bool,
}

enum AppMsg {
    Save,
    Close,
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = ();
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, _components: &(), _sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::Save => {
                // Print the value of the text box
                self.hidden = !self.hidden;

                // Open the file chooser and get the path

                // Save the contents of AppModel.text to the file
            }

            AppMsg::Close => {
                self.hidden = false;
            }
        }
        true
    }
}

#[relm4::widget]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        gtk::ApplicationWindow {
            set_title: Some("dNotes app"),
            set_default_width: 600,
            set_default_height: 400,
            set_child = Some(&gtk::Box) {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 5,
                set_spacing: 5,

                append = &gtk::TextView {
                    set_margin_all: 5,
                    set_wrap_mode: gtk::WrapMode::Word,
                    set_buffer: Some(&model.text),
                },

                append = &gtk::Button {
                    set_label: "Save",
                    connect_clicked(sender) => move |_| {
                        send!(sender, AppMsg::Save);
                    },
                },

                append = &gtk::MessageDialog {
                    set_modal: true,
                    set_visible: watch!(model.hidden),
                    set_text: Some("Save?"),
                    add_button: args!("Save", gtk::ResponseType::Accept),
                    add_button: args!("Cancel", gtk::ResponseType::Cancel),
                    connect_response(sender) => move |_, resp| {
                        send!(sender, if resp == gtk::ResponseType::Accept {
                            AppMsg::Close
                        } else {
                            AppMsg::Close
                        });
                    }
                }
            },
        }
    }
}

fn main() {
    let model = AppModel::default();
    let app = RelmApp::new(model);
    app.run();
}
