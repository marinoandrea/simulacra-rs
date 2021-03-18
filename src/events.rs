#[macro_export]
macro_rules! bit {
    ($n:expr) => {
        1 << $n
    };
}

pub struct Category;

impl Category {
    pub const APPLICATION: u8 = bit!(0);
    pub const INPUT: u8 = bit!(1);
    pub const KEYBOARD: u8 = bit!(2);
    pub const MOUSE: u8 = bit!(3);
}

#[derive(Debug)]
pub enum Event {
    WindowClose,
    WindowResize { width: u32, height: u32 },
    WindowFocus,
    WindowLostFocus,
    WindowMoved { x: u32, y: u32 },
    AppTick,
    AppUpdate,
    AppRender,
    KeyPressed { key: Key },
    KeyReleased { key: Key },
    MouseButtonPressed { btn: MouseButton },
    MouseButtonReleased { btn: MouseButton },
    MouseMoved { x: u64, y: u64 },
    MouseScrolled { x: u64, y: u64 },
}

impl Event {
    pub fn get_category_flags(&self) -> u8 {
        match &self {
            Event::WindowClose => Category::APPLICATION | Category::INPUT,
            Event::WindowResize { .. } => Category::APPLICATION | Category::INPUT,
            Event::WindowFocus => Category::APPLICATION | Category::INPUT,
            Event::WindowLostFocus => Category::APPLICATION | Category::INPUT,
            Event::WindowMoved { .. } => Category::APPLICATION | Category::INPUT,
            Event::AppTick => Category::APPLICATION,
            Event::AppUpdate => Category::APPLICATION,
            Event::AppRender => Category::APPLICATION,
            Event::KeyPressed { .. } => Category::KEYBOARD | Category::INPUT,
            Event::KeyReleased { .. } => Category::KEYBOARD | Category::INPUT,
            Event::MouseButtonPressed { .. } => Category::MOUSE | Category::INPUT,
            Event::MouseButtonReleased { .. } => Category::MOUSE | Category::INPUT,
            Event::MouseMoved { .. } => Category::MOUSE | Category::INPUT,
            Event::MouseScrolled { .. } => Category::MOUSE | Category::INPUT,
        }
    }

    pub fn is_in_category(&self, c: u8) -> bool {
        (self.get_category_flags() & c) != 0
    }
}

pub trait EventHandler {
    fn handle_events(&mut self, events: &mut Vec<Event>);
}
#[derive(Debug)]
pub enum Key {
    Space,
    Apostrophe,
    Comma,
    Minus,
    Period,
    Slash,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Semicolon,
    Equal,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    LeftBracket,
    Backslash,
    RightBracket,
    GraveAccent,
    World1,
    World2,
    Escape,
    Enter,
    Tab,
    Backspace,
    Insert,
    Delete,
    Right,
    Left,
    Down,
    Up,
    PageUp,
    PageDown,
    Home,
    End,
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    KpDecimal,
    KpDivide,
    KpMultiply,
    KpSubtract,
    KpAdd,
    KpEnter,
    KpEqual,
    LeftShift,
    LeftControl,
    LeftAlt,
    LeftSuper,
    RightShift,
    RightControl,
    RightAlt,
    RightSuper,
    Menu,
}

#[derive(Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    X1,
    X2,
}
