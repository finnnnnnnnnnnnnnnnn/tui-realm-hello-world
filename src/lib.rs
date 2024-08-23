pub mod model;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum Id {
    PhantomListener,
    Label,
}

#[derive(Debug, PartialEq, Clone)]
enum Msg {
    AppClose,
    None,
}

#[derive(PartialEq, Eq, Clone, PartialOrd)]
enum AppEvent {
    ErrorInitialized,
}