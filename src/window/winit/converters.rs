use crate::input::keyboard::{KeyState, KeyboardInput, VirtualKeyCode};

impl From<&winit::event::KeyboardInput> for KeyboardInput {
    fn from(keyboard_input: &winit::event::KeyboardInput) -> Self {
        KeyboardInput {
            scan_code: keyboard_input.scancode,
            state: keyboard_input.state.into(),
            virtual_key_code: keyboard_input.virtual_keycode.map(|v| v.into()),
        }
    }
}

impl From<winit::event::ElementState> for KeyState {
    fn from(element_state: winit::event::ElementState) -> Self {
        match element_state {
            winit::event::ElementState::Pressed => KeyState::Pressed,
            winit::event::ElementState::Released => KeyState::Released,
        }
    }
}

impl From<winit::event::VirtualKeyCode> for VirtualKeyCode {
    fn from(virtual_key_code: winit::event::VirtualKeyCode) -> Self {
        match virtual_key_code {
            winit::event::VirtualKeyCode::Key1 => VirtualKeyCode::Key1,
            winit::event::VirtualKeyCode::Key2 => VirtualKeyCode::Key2,
            winit::event::VirtualKeyCode::Key3 => VirtualKeyCode::Key3,
            winit::event::VirtualKeyCode::Key4 => VirtualKeyCode::Key4,
            winit::event::VirtualKeyCode::Key5 => VirtualKeyCode::Key5,
            winit::event::VirtualKeyCode::Key6 => VirtualKeyCode::Key6,
            winit::event::VirtualKeyCode::Key7 => VirtualKeyCode::Key7,
            winit::event::VirtualKeyCode::Key8 => VirtualKeyCode::Key8,
            winit::event::VirtualKeyCode::Key9 => VirtualKeyCode::Key9,
            winit::event::VirtualKeyCode::Key0 => VirtualKeyCode::Key0,
            winit::event::VirtualKeyCode::A => VirtualKeyCode::A,
            winit::event::VirtualKeyCode::B => VirtualKeyCode::B,
            winit::event::VirtualKeyCode::C => VirtualKeyCode::C,
            winit::event::VirtualKeyCode::D => VirtualKeyCode::D,
            winit::event::VirtualKeyCode::E => VirtualKeyCode::E,
            winit::event::VirtualKeyCode::F => VirtualKeyCode::F,
            winit::event::VirtualKeyCode::G => VirtualKeyCode::G,
            winit::event::VirtualKeyCode::H => VirtualKeyCode::H,
            winit::event::VirtualKeyCode::I => VirtualKeyCode::I,
            winit::event::VirtualKeyCode::J => VirtualKeyCode::J,
            winit::event::VirtualKeyCode::K => VirtualKeyCode::K,
            winit::event::VirtualKeyCode::L => VirtualKeyCode::L,
            winit::event::VirtualKeyCode::M => VirtualKeyCode::M,
            winit::event::VirtualKeyCode::N => VirtualKeyCode::N,
            winit::event::VirtualKeyCode::O => VirtualKeyCode::O,
            winit::event::VirtualKeyCode::P => VirtualKeyCode::P,
            winit::event::VirtualKeyCode::Q => VirtualKeyCode::Q,
            winit::event::VirtualKeyCode::R => VirtualKeyCode::R,
            winit::event::VirtualKeyCode::S => VirtualKeyCode::S,
            winit::event::VirtualKeyCode::T => VirtualKeyCode::T,
            winit::event::VirtualKeyCode::U => VirtualKeyCode::U,
            winit::event::VirtualKeyCode::V => VirtualKeyCode::V,
            winit::event::VirtualKeyCode::W => VirtualKeyCode::W,
            winit::event::VirtualKeyCode::X => VirtualKeyCode::X,
            winit::event::VirtualKeyCode::Y => VirtualKeyCode::Y,
            winit::event::VirtualKeyCode::Z => VirtualKeyCode::Z,
            winit::event::VirtualKeyCode::Escape => VirtualKeyCode::Escape,
            winit::event::VirtualKeyCode::F1 => VirtualKeyCode::F1,
            winit::event::VirtualKeyCode::F2 => VirtualKeyCode::F2,
            winit::event::VirtualKeyCode::F3 => VirtualKeyCode::F3,
            winit::event::VirtualKeyCode::F4 => VirtualKeyCode::F4,
            winit::event::VirtualKeyCode::F5 => VirtualKeyCode::F5,
            winit::event::VirtualKeyCode::F6 => VirtualKeyCode::F6,
            winit::event::VirtualKeyCode::F7 => VirtualKeyCode::F7,
            winit::event::VirtualKeyCode::F8 => VirtualKeyCode::F8,
            winit::event::VirtualKeyCode::F9 => VirtualKeyCode::F9,
            winit::event::VirtualKeyCode::F10 => VirtualKeyCode::F10,
            winit::event::VirtualKeyCode::F11 => VirtualKeyCode::F11,
            winit::event::VirtualKeyCode::F12 => VirtualKeyCode::F12,
            winit::event::VirtualKeyCode::F13 => VirtualKeyCode::F13,
            winit::event::VirtualKeyCode::F14 => VirtualKeyCode::F14,
            winit::event::VirtualKeyCode::F15 => VirtualKeyCode::F15,
            winit::event::VirtualKeyCode::F16 => VirtualKeyCode::F16,
            winit::event::VirtualKeyCode::F17 => VirtualKeyCode::F17,
            winit::event::VirtualKeyCode::F18 => VirtualKeyCode::F18,
            winit::event::VirtualKeyCode::F19 => VirtualKeyCode::F19,
            winit::event::VirtualKeyCode::F20 => VirtualKeyCode::F20,
            winit::event::VirtualKeyCode::F21 => VirtualKeyCode::F21,
            winit::event::VirtualKeyCode::F22 => VirtualKeyCode::F22,
            winit::event::VirtualKeyCode::F23 => VirtualKeyCode::F23,
            winit::event::VirtualKeyCode::F24 => VirtualKeyCode::F24,
            winit::event::VirtualKeyCode::Snapshot => VirtualKeyCode::Snapshot,
            winit::event::VirtualKeyCode::Scroll => VirtualKeyCode::Scroll,
            winit::event::VirtualKeyCode::Pause => VirtualKeyCode::Pause,
            winit::event::VirtualKeyCode::Insert => VirtualKeyCode::Insert,
            winit::event::VirtualKeyCode::Home => VirtualKeyCode::Home,
            winit::event::VirtualKeyCode::Delete => VirtualKeyCode::Delete,
            winit::event::VirtualKeyCode::End => VirtualKeyCode::End,
            winit::event::VirtualKeyCode::PageDown => VirtualKeyCode::PageDown,
            winit::event::VirtualKeyCode::PageUp => VirtualKeyCode::PageUp,
            winit::event::VirtualKeyCode::Left => VirtualKeyCode::Left,
            winit::event::VirtualKeyCode::Up => VirtualKeyCode::Up,
            winit::event::VirtualKeyCode::Right => VirtualKeyCode::Right,
            winit::event::VirtualKeyCode::Down => VirtualKeyCode::Down,
            winit::event::VirtualKeyCode::Back => VirtualKeyCode::Back,
            winit::event::VirtualKeyCode::Return => VirtualKeyCode::Return,
            winit::event::VirtualKeyCode::Space => VirtualKeyCode::Space,
            winit::event::VirtualKeyCode::Compose => VirtualKeyCode::Compose,
            winit::event::VirtualKeyCode::Caret => VirtualKeyCode::Caret,
            winit::event::VirtualKeyCode::Numlock => VirtualKeyCode::Numlock,
            winit::event::VirtualKeyCode::Numpad0 => VirtualKeyCode::Numpad0,
            winit::event::VirtualKeyCode::Numpad1 => VirtualKeyCode::Numpad1,
            winit::event::VirtualKeyCode::Numpad2 => VirtualKeyCode::Numpad2,
            winit::event::VirtualKeyCode::Numpad3 => VirtualKeyCode::Numpad3,
            winit::event::VirtualKeyCode::Numpad4 => VirtualKeyCode::Numpad4,
            winit::event::VirtualKeyCode::Numpad5 => VirtualKeyCode::Numpad5,
            winit::event::VirtualKeyCode::Numpad6 => VirtualKeyCode::Numpad6,
            winit::event::VirtualKeyCode::Numpad7 => VirtualKeyCode::Numpad7,
            winit::event::VirtualKeyCode::Numpad8 => VirtualKeyCode::Numpad8,
            winit::event::VirtualKeyCode::Numpad9 => VirtualKeyCode::Numpad9,
            winit::event::VirtualKeyCode::AbntC1 => VirtualKeyCode::AbntC1,
            winit::event::VirtualKeyCode::AbntC2 => VirtualKeyCode::AbntC2,
            winit::event::VirtualKeyCode::Add => VirtualKeyCode::Add,
            winit::event::VirtualKeyCode::Apostrophe => VirtualKeyCode::Apostrophe,
            winit::event::VirtualKeyCode::Apps => VirtualKeyCode::Apps,
            winit::event::VirtualKeyCode::At => VirtualKeyCode::At,
            winit::event::VirtualKeyCode::Ax => VirtualKeyCode::Ax,
            winit::event::VirtualKeyCode::Backslash => VirtualKeyCode::Backslash,
            winit::event::VirtualKeyCode::Calculator => VirtualKeyCode::Calculator,
            winit::event::VirtualKeyCode::Capital => VirtualKeyCode::Capital,
            winit::event::VirtualKeyCode::Colon => VirtualKeyCode::Colon,
            winit::event::VirtualKeyCode::Comma => VirtualKeyCode::Comma,
            winit::event::VirtualKeyCode::Convert => VirtualKeyCode::Convert,
            winit::event::VirtualKeyCode::Decimal => VirtualKeyCode::Decimal,
            winit::event::VirtualKeyCode::Divide => VirtualKeyCode::Divide,
            winit::event::VirtualKeyCode::Equals => VirtualKeyCode::Equals,
            winit::event::VirtualKeyCode::Grave => VirtualKeyCode::Grave,
            winit::event::VirtualKeyCode::Kana => VirtualKeyCode::Kana,
            winit::event::VirtualKeyCode::Kanji => VirtualKeyCode::Kanji,
            winit::event::VirtualKeyCode::LAlt => VirtualKeyCode::LAlt,
            winit::event::VirtualKeyCode::LBracket => VirtualKeyCode::LBracket,
            winit::event::VirtualKeyCode::LControl => VirtualKeyCode::LControl,
            winit::event::VirtualKeyCode::LShift => VirtualKeyCode::LShift,
            winit::event::VirtualKeyCode::LWin => VirtualKeyCode::LWin,
            winit::event::VirtualKeyCode::Mail => VirtualKeyCode::Mail,
            winit::event::VirtualKeyCode::MediaSelect => VirtualKeyCode::MediaSelect,
            winit::event::VirtualKeyCode::MediaStop => VirtualKeyCode::MediaStop,
            winit::event::VirtualKeyCode::Minus => VirtualKeyCode::Minus,
            winit::event::VirtualKeyCode::Multiply => VirtualKeyCode::Multiply,
            winit::event::VirtualKeyCode::Mute => VirtualKeyCode::Mute,
            winit::event::VirtualKeyCode::MyComputer => VirtualKeyCode::MyComputer,
            winit::event::VirtualKeyCode::NavigateForward => VirtualKeyCode::NavigateForward,
            winit::event::VirtualKeyCode::NavigateBackward => VirtualKeyCode::NavigateBackward,
            winit::event::VirtualKeyCode::NextTrack => VirtualKeyCode::NextTrack,
            winit::event::VirtualKeyCode::NoConvert => VirtualKeyCode::NoConvert,
            winit::event::VirtualKeyCode::NumpadComma => VirtualKeyCode::NumpadComma,
            winit::event::VirtualKeyCode::NumpadEnter => VirtualKeyCode::NumpadEnter,
            winit::event::VirtualKeyCode::NumpadEquals => VirtualKeyCode::NumpadEquals,
            winit::event::VirtualKeyCode::OEM102 => VirtualKeyCode::OEM102,
            winit::event::VirtualKeyCode::Period => VirtualKeyCode::Period,
            winit::event::VirtualKeyCode::PlayPause => VirtualKeyCode::PlayPause,
            winit::event::VirtualKeyCode::Power => VirtualKeyCode::Power,
            winit::event::VirtualKeyCode::PrevTrack => VirtualKeyCode::PrevTrack,
            winit::event::VirtualKeyCode::RAlt => VirtualKeyCode::RAlt,
            winit::event::VirtualKeyCode::RBracket => VirtualKeyCode::RBracket,
            winit::event::VirtualKeyCode::RControl => VirtualKeyCode::RControl,
            winit::event::VirtualKeyCode::RShift => VirtualKeyCode::RShift,
            winit::event::VirtualKeyCode::RWin => VirtualKeyCode::RWin,
            winit::event::VirtualKeyCode::Semicolon => VirtualKeyCode::Semicolon,
            winit::event::VirtualKeyCode::Slash => VirtualKeyCode::Slash,
            winit::event::VirtualKeyCode::Sleep => VirtualKeyCode::Sleep,
            winit::event::VirtualKeyCode::Stop => VirtualKeyCode::Stop,
            winit::event::VirtualKeyCode::Subtract => VirtualKeyCode::Subtract,
            winit::event::VirtualKeyCode::Sysrq => VirtualKeyCode::Sysrq,
            winit::event::VirtualKeyCode::Tab => VirtualKeyCode::Tab,
            winit::event::VirtualKeyCode::Underline => VirtualKeyCode::Underline,
            winit::event::VirtualKeyCode::Unlabeled => VirtualKeyCode::Unlabeled,
            winit::event::VirtualKeyCode::VolumeDown => VirtualKeyCode::VolumeDown,
            winit::event::VirtualKeyCode::VolumeUp => VirtualKeyCode::VolumeUp,
            winit::event::VirtualKeyCode::Wake => VirtualKeyCode::Wake,
            winit::event::VirtualKeyCode::WebBack => VirtualKeyCode::WebBack,
            winit::event::VirtualKeyCode::WebFavorites => VirtualKeyCode::WebFavorites,
            winit::event::VirtualKeyCode::WebForward => VirtualKeyCode::WebForward,
            winit::event::VirtualKeyCode::WebHome => VirtualKeyCode::WebHome,
            winit::event::VirtualKeyCode::WebRefresh => VirtualKeyCode::WebRefresh,
            winit::event::VirtualKeyCode::WebSearch => VirtualKeyCode::WebSearch,
            winit::event::VirtualKeyCode::WebStop => VirtualKeyCode::WebStop,
            winit::event::VirtualKeyCode::Yen => VirtualKeyCode::Yen,
            winit::event::VirtualKeyCode::Copy => VirtualKeyCode::Copy,
            winit::event::VirtualKeyCode::Paste => VirtualKeyCode::Paste,
            winit::event::VirtualKeyCode::Cut => VirtualKeyCode::Cut,
        }
    }
}
