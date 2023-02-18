use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt, TextViewExt};
use relm4::{
    gtk::{self, traits::TextBufferExt, TextBuffer},
    ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent,
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
                set_spacing: 5,
                set_margin_all: 5,

                gtk::TextView::with_buffer(&model.text) {

                gtk::TextView {
                    set_margin_all: 5,
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
                // TODO: Save the text to a file
                let start = self.text.start_iter();
                let end = self.text.end_iter();
                let text = self.text.text(&start, &end, true);
                println!("{}", text);
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("dev.doodles.dnotes");
    app.run::<AppModel>(gtk::TextBuffer::new(Some(&gtk::TextTagTable::new())));
}
