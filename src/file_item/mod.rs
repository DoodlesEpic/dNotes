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
        root = gtk::Button {
            set_halign: gtk::Align::Fill,
            set_label: &self.value.file_name().expect("Failed to get file name").to_string_lossy().replace(".md", ""),
            connect_clicked[sender, index] => move |_| {
                sender.output(FileItemOutput::Open(index.clone()));
            }
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
