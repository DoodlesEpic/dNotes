use crate::AppMsg;
use relm4::gtk::prelude::*;
use relm4::prelude::*;

#[derive(Debug)]
pub(crate) struct FileItem {
    pub(crate) value: std::path::PathBuf,
}

#[derive(Debug)]
pub(crate) enum FileItemMsg {}

#[derive(Debug)]
pub(crate) enum FileItemOutput {
    Open(DynamicIndex),
}

#[relm4::factory(pub(crate))]
impl FactoryComponent for FileItem {
    type Init = std::path::PathBuf;
    type Input = FileItemMsg;
    type Output = FileItemOutput;
    type CommandOutput = ();
    type Widgets = FileItemWidgets;
    type ParentInput = AppMsg;
    type ParentWidget = gtk::Box;

    view! {
        root = gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 10,

            #[name(label)]
            gtk::Label {
                #[watch]
                set_label: &self.value.file_name().unwrap_or_default().to_str().unwrap_or("Untitled"),
                set_width_chars: 3,
            },

            #[name(open_button)]
            gtk::Button {
                set_label: "Open",
                connect_clicked[sender, index] => move |_| {
                    sender.output(FileItemOutput::Open(index.clone()));
                }
            },
        }
    }

    fn init_model(value: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self { value }
    }

    fn output_to_parent_input(output: Self::Output) -> Option<AppMsg> {
        Some(match output {
            FileItemOutput::Open(index) => AppMsg::OpenFile(index),
        })
    }
}
