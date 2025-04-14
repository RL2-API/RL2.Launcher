use iced::Task;
use iced::{ window, widget };

const TITLE: &str = "Rogue Legacy 2 Launcher";

fn main() -> iced::Result {
    iced::application(TITLE, App::update, App::view)
        .decorations(false)
        .run_with(App::new)
}

pub struct App {
    path_file: String,
    store_file: String,
    rl2_path: String,
    store: Option<Store>,
    stores: widget::combo_box::State<Store>
}

#[derive(Debug, Clone)]
pub enum Store {
    EpicGames,
    Steam
}

impl std::fmt::Display for Store {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Store::EpicGames => "Epic Games Store",
            Store::Steam => "Steam"
        })
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    Titlebar(TitlebarEvent),
    PathChanged(String),
    StoreChanged(Store),
    InfoProvided
}

impl App {
    pub fn new() -> (App, Task<Event>) {
        let mut rl2_path = "".to_string();
        let mut store = None;

        let current_dir = std::env::current_dir().expect("Huh").display().to_string();

        let path_file = current_dir.clone() + "/path.saved";
        let store_file = current_dir.clone() + "/store.saved";

        if let Ok(content) = std::fs::read_to_string(&path_file) {
            rl2_path = content;            
        }

        if let Ok(content) = std::fs::read_to_string(&store_file) {
            if content == "Epic Games Store" {
                store = Some(Store::EpicGames);
            }
            else {
                store = Some(Store::Steam);
            }
        }

        (
            App { 
                path_file,
                store_file,
                rl2_path, 
                store, 
                stores: widget::combo_box::State::<Store>::new(vec![
                    Store::EpicGames,
                    Store::Steam
                ])
            },
            window::get_latest().and_then(move |id| {
                window::maximize(id, true)
            })
        )
    }

    pub fn update(&mut self, event: Event) -> Task<Event> {
        match event {
            Event::Titlebar(titlebar) => match titlebar {
                TitlebarEvent::Hide => window::get_latest().and_then(move |id| { 
                    window::minimize(id, true) 
                }),
                TitlebarEvent::Maximize => window::get_latest().and_then(move |id| { 
                    window::get_maximized(id).then(move |maximized| {
                        window::maximize(id, !maximized)
                    }) 
                }),
                TitlebarEvent::Close => window::get_latest().and_then(move |id| { 
                    window::close(id) 
                }),
            },
            Event::PathChanged(new_path) => {
                self.rl2_path = new_path.clone();
                let _ = std::fs::write(self.path_file.clone(), new_path);
                Task::none()
            },
            Event::StoreChanged(store) => {
                self.store = Some(store.clone());
                let _ = std::fs::write(self.store_file.clone(), format!("{}", store));
                Task::none()
            }
            _ => todo!()
        }
    }

    pub fn view(&self) -> iced::Element<Event> { 
        widget::column![
            widget::row![
                widget::text("Rogue Legacy 2 Launcher").width(iced::Length::Fill),
                widget::button("_").on_press(Event::Titlebar(TitlebarEvent::Hide)),
                widget::button("[_]").on_press(Event::Titlebar(TitlebarEvent::Maximize)),
                widget::button("X").on_press(Event::Titlebar(TitlebarEvent::Close)),
            ].width(iced::Length::Fill).padding(iced::Padding { top: 0.0, right: 0.0, left: 5.0, bottom: 0.0 }),

            widget::row![
                widget::text_input("Rogue Legacy 2 installation path...", &self.rl2_path).on_input(Event::PathChanged).width(iced::Length::FillPortion(8)),
                widget::combo_box(&self.stores, "Select store", self.store.as_ref(), Event::StoreChanged),
                widget::button("Vanilla"),
                widget::button("Modded")
            ].width(iced::Length::Fill).padding(iced::Padding { top: 15.0, right: 0.0, left: 0.0, bottom: 0.0 }).spacing(5)
        ].padding(iced::Padding { top: 5.0, right: 5.0, left: 5.0, bottom: 0.0 }).into()
    }
}


#[derive(Debug, Clone)]
pub enum TitlebarEvent { 
    Hide,
    Maximize,
    Close
}
