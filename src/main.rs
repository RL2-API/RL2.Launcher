use iced::Task;
use iced::{ window, widget };

const TITLE: &str = "Rogue Legacy 2 Launcher";

fn main() -> iced::Result {
    iced::application(TITLE, App::update, App::view)
        .decorations(false)
        .run_with(App::new)
}

pub struct App { }

#[derive(Debug, Clone)]
pub enum Event {
    Titlebar(TitlebarEvent)
}

impl App {
    pub fn new() -> (App, Task<Event>) {
        (
            App { },
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
        }
    }

    pub fn view(&self) -> iced::Element<Event> { 
        widget::column![
            widget::row![
                widget::text("Rogue Legacy 2 Launcher").width(iced::Length::Fill),
                widget::button("_").on_press(Event::Titlebar(TitlebarEvent::Hide)),
                widget::button("[_]").on_press(Event::Titlebar(TitlebarEvent::Maximize)),
                widget::button("X").on_press(Event::Titlebar(TitlebarEvent::Close)),
            ].width(iced::Length::Fill)
        ].into()
    }
}


#[derive(Debug, Clone)]
pub enum TitlebarEvent { 
    Hide,
    Maximize,
    Close
}
