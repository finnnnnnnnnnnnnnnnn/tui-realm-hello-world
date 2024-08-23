use std::time::Duration;

use tuirealm::{
    command::{Cmd, CmdResult}, event::{Key, KeyEvent, KeyModifiers}, props::{Alignment, Color, Style, TextModifiers}, terminal::TerminalBridge, tui::{layout::{Constraint, Direction, Layout, Rect}, widgets::Paragraph}, Application, AttrValue, Attribute, Component, Event, EventListenerCfg, Frame, MockComponent, PollStrategy, Props, State, Sub, SubClause, SubEventClause, Update
};

use crate::{
    AppEvent, Id, Msg
};


// MODEL
pub struct Model {
    quit: bool,
    redraw: bool,
    terminal: TerminalBridge,
    app: Application<Id, Msg, AppEvent>,
}
impl Model {
    pub fn new() -> Self {
        let quit = false;
        let redraw = true;
        let mut terminal = TerminalBridge::new().expect("Cannot create terminal bridge");
        let _ = terminal.enable_raw_mode();
        let _ = terminal.enter_alternate_screen();
        let mut app: Application<Id, Msg, AppEvent> = Application::init(
            EventListenerCfg::default()
                .default_input_listener(Duration::from_millis(10)),
        );
        assert!(app
            .mount(
                Id::Label,
                Box::new(
                    HelloWorld::default()
                        .text("Hello World!")
                        .alignment(Alignment::Left)
                ),
                Vec::default(),
            )
            .is_ok());

        assert!(app
            .mount(
                Id::PhantomListener,
                Box::<PhantomListener>::default(),
                vec![
                    Sub::new(
                        SubEventClause::Keyboard(KeyEvent {
                            code: Key::Esc,
                            modifiers: KeyModifiers::NONE
                        }),
                        SubClause::Always
                    ),
                    Sub::new(
                        SubEventClause::User(AppEvent::ErrorInitialized),
                        SubClause::Always
                    )
                ]
            )
            .is_ok());

        Self {
            quit,
            redraw,
            terminal,
            app,
        }
    }

    pub fn main_loop(&mut self) {
        while !self.quit {
            // Tick
            if let Ok(messages) = self.app.tick(PollStrategy::Once) {
                messages.iter().map(Some).for_each(|msg| {
                    let mut msg = msg.cloned();
                    while msg.is_some() {
                        msg = self.update(msg);
                    }
                });
            }

            // Redraw
            if self.redraw {
                self.view();
                self.redraw = false;
            }
        }
        let _ = self.terminal.leave_alternate_screen();
        let _ = self.terminal.disable_raw_mode();
        let _ = self.terminal.clear_screen();
    }

    fn view(&mut self) {
        let _ = self.terminal.raw_mut().draw(|f| {
            let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Length(1), // Label
                        ]
                        .as_ref(),
                    )
                    .split(f.size());
                self.app.view(&Id::Label, f, chunks[0]);
        });
    }
}

impl Update<Msg> for Model {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        self.redraw = true;
        match msg.unwrap_or(Msg::None) {
            Msg::AppClose => {
                self.quit = true;
                None
            }
            Msg::None => None,        
        }
    }
}

// PHANTOM LISTENER COMPONENT
use tui_realm_stdlib::Phantom;

#[derive(MockComponent, Default)]
pub struct PhantomListener {
    component: Phantom,
}

impl Component<Msg, AppEvent> for PhantomListener {
    fn on(&mut self, ev: Event<AppEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            Event::User(AppEvent::ErrorInitialized) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}


// HELLO WORLD COMPONENT
pub struct HelloWorld {
    props: Props,
}

impl Default for HelloWorld {
    fn default() -> Self {
        Self {
            props: Props::default(),
        }
    }
}

impl HelloWorld {
    pub fn text<S>(mut self, s: S) -> Self
    where
        S: AsRef<str>,
    {
        self.attr(Attribute::Text, AttrValue::String(s.as_ref().to_string()));
        self
    }

    pub fn alignment(mut self, a: Alignment) -> Self {
        self.attr(Attribute::TextAlign, AttrValue::Alignment(a));
        self
    }

    pub fn foreground(mut self, c: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(c));
        self
    }

    pub fn background(mut self, c: Color) -> Self {
        self.attr(Attribute::Background, AttrValue::Color(c));
        self
    }

    pub fn modifiers(mut self, m: TextModifiers) -> Self {
        self.attr(Attribute::TextProps, AttrValue::TextModifiers(m));
        self
    }
}

impl MockComponent for HelloWorld {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        // Check if visible
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            // Get properties
            let text = self
                .props
                .get_or(Attribute::Text, AttrValue::String(String::default()))
                .unwrap_string();
            let alignment = self
                .props
                .get_or(Attribute::TextAlign, AttrValue::Alignment(Alignment::Left))
                .unwrap_alignment();
            let foreground = self
                .props
                .get_or(Attribute::Foreground, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let background = self
                .props
                .get_or(Attribute::Background, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let modifiers = self
                .props
                .get_or(
                    Attribute::TextProps,
                    AttrValue::TextModifiers(TextModifiers::empty()),
                )
                .unwrap_text_modifiers();
            frame.render_widget(
                Paragraph::new(text)
                    .style(
                        Style::default()
                            .fg(foreground)
                            .bg(background)
                            .add_modifier(modifiers),
                    )
                    .alignment(alignment),
                area,
            );
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, _: Cmd) -> CmdResult {
        CmdResult::None
    }
}

impl Component<Msg, AppEvent> for HelloWorld {
    fn on(&mut self, _: Event<AppEvent>) -> Option<Msg> {
        None
    }
}


