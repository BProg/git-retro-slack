use colorful::{Color, Colorful};

const MESSAGE_COLOR: Color = Color::DarkGray;
const ERROR_COLOR: Color = Color::DarkRed1;

pub enum Style<S: AsRef<str>> {
    Message(S),
    Important(S),
    #[allow(dead_code)]
    Error(S),
}

pub fn multiple<V, S>(msgs: V)
where
    S: AsRef<str>,
    V: AsRef<[Style<S>]>,
{
    for style in msgs.as_ref() {
        match style {
            Style::Message(msg) => message(msg),
            Style::Important(msg) => important(msg),
            Style::Error(msg) => error(msg),
        }
    }
}

pub fn message<S: AsRef<str>>(msg: S) {
    println!("{}", msg.as_ref().color(MESSAGE_COLOR));
}

pub fn important<S: AsRef<str>>(msg: S) {
    println!("{}", msg.as_ref().bold());
}

pub fn error<S: AsRef<str>>(msg: S) {
    println!("{}", msg.as_ref().color(ERROR_COLOR));
}
