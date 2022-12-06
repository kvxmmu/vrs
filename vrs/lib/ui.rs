use {
    crate::{
        message::index,
        route::UiRoute,
    },
    iced::{
        alignment::{
            Horizontal,
            Vertical,
        },
        widget::{
            self,
            button,
            container,
            pane_grid,
            text,
            PaneGrid,
        },
        Length,
        Sandbox,
    },
    rfd::FileDialog,
    std::path::PathBuf,
    vrs_novel::novel::Novel,
};

enum Pane {
    Index,
}

pub struct InterpreterUi {
    route: UiRoute,
    state: pane_grid::State<Pane>,
}

impl Sandbox for InterpreterUi {
    type Message = index::Message;

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    fn new() -> Self {
        Self {
            route: UiRoute::Index,
            state: pane_grid::State::new(Pane::Index).0,
        }
    }

    fn title(&self) -> String {
        String::from("VRS interpreter")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            index::Message::AddNewNovel => {
                let folder = FileDialog::new()
                    .set_directory(std::env::current_dir().unwrap())
                    .pick_folder();
                if let Some(folder) = folder {
                    self.add_new_novel(folder);
                }
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        PaneGrid::new(&self.state, |_id, pane, _is_maximized| {
            widget::column![
                container(text(match pane {
                    Pane::Index => "Select novel to play",
                }))
                .padding(16),
                container(widget::column![button("Add novel")
                    .on_press(index::Message::AddNewNovel)])
                .width(Length::Fill)
                .height(Length::Fill)
                .align_y(Vertical::Bottom)
                .align_x(Horizontal::Right)
                .padding(16)
            ]
            .into()
        })
        .into()
    }
}

impl InterpreterUi {
    fn add_new_novel(&mut self, path: PathBuf) {
        let novel = Novel::from_directory(path);
        dbg!(novel);
    }
}
