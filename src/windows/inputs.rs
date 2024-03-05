use crate::public::{
    KeybdKey::{self, *},
    MouseButton::{self, *},
};

impl From<KeybdKey> for u64 {
    // https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes?redirectedfrom=MSDN
    fn from(key: KeybdKey) -> u64 {
        match key {
            BackspaceKey => 0x08,
            TabKey => 0x09,
            EnterKey => 0x0D,
            EscapeKey => 0x1B,
            SpaceKey => 0x20,
            PageUpKey => 0x21,
            PageDownKey => 0x22,
            EndKey => 0x23,
            HomeKey => 0x24,
            LeftKey => 0x25,
            UpKey => 0x26,
            RightKey => 0x27,
            DownKey => 0x28,
            InsertKey => 0x2D,
            DeleteKey => 0x2E,
            Numrow0Key => 0x30,
            Numrow1Key => 0x31,
            Numrow2Key => 0x32,
            Numrow3Key => 0x33,
            Numrow4Key => 0x34,
            Numrow5Key => 0x35,
            Numrow6Key => 0x36,
            Numrow7Key => 0x37,
            Numrow8Key => 0x38,
            Numrow9Key => 0x39,
            AKey => 0x41,
            BKey => 0x42,
            CKey => 0x43,
            DKey => 0x44,
            EKey => 0x45,
            FKey => 0x46,
            GKey => 0x47,
            HKey => 0x48,
            IKey => 0x49,
            JKey => 0x4A,
            KKey => 0x4B,
            LKey => 0x4C,
            MKey => 0x4D,
            NKey => 0x4E,
            OKey => 0x4F,
            PKey => 0x50,
            QKey => 0x51,
            RKey => 0x52,
            SKey => 0x53,
            TKey => 0x54,
            UKey => 0x55,
            VKey => 0x56,
            WKey => 0x57,
            XKey => 0x58,
            YKey => 0x59,
            ZKey => 0x5A,
            LSuper => 0x5B,
            RSuper => 0x5C,
            Numpad0Key => 0x60,
            Numpad1Key => 0x61,
            Numpad2Key => 0x62,
            Numpad3Key => 0x63,
            Numpad4Key => 0x64,
            Numpad5Key => 0x65,
            Numpad6Key => 0x66,
            Numpad7Key => 0x67,
            Numpad8Key => 0x68,
            Numpad9Key => 0x69,
            F1Key => 0x70,
            F2Key => 0x71,
            F3Key => 0x72,
            F4Key => 0x73,
            F5Key => 0x74,
            F6Key => 0x75,
            F7Key => 0x76,
            F8Key => 0x77,
            F9Key => 0x78,
            F10Key => 0x79,
            F11Key => 0x7A,
            F12Key => 0x7B,
            F13Key => 0x7C,
            F14Key => 0x7D,
            F15Key => 0x7E,
            F16Key => 0x7F,
            F17Key => 0x80,
            F18Key => 0x81,
            F19Key => 0x82,
            F20Key => 0x83,
            F21Key => 0x84,
            F22Key => 0x85,
            F23Key => 0x86,
            F24Key => 0x87,
            NumLockKey => 0x90,
            ScrollLockKey => 0x91,
            CapsLockKey => 0x14,
            LShiftKey => 0xA0,
            RShiftKey => 0xA1,
            LControlKey => 0xA2,
            RControlKey => 0xA3,
            LAltKey => 0xA4,
            RAltKey => 0xA5,
            BrowserBackKey => 0xA6,
            BrowserForwardKey => 0xA7,
            BrowserRefreshKey => 0xA8,
            VolumeMuteKey => 0xAD,
            VolumeDownKey => 0xAE,
            VolumeUpKey => 0xAF,
            MediaNextTrackKey => 0xB0,
            MediaPrevTrackKey => 0xB1,
            MediaStopKey => 0xB2,
            MediaPlayPauseKey => 0xB3,
            BackquoteKey => 0xC0,
            SlashKey => 0xBF,
            BackslashKey => 0xDC,
            CommaKey => 0xBC,
            PeriodKey => 0xBE,
            MinusKey => 0xBD,
            QuoteKey => 0xDE,
            SemicolonKey => 0xBA,
            LBracketKey => 0xDB,
            RBracketKey => 0xDD,
            EqualKey => 0xBB,
            PauseKey => 0x13,
            PrintScreenKey => 0x2C,
            AppsKey => 0x5D,
            OtherKey(code) => code,
        }
    }
}

impl From<u64> for KeybdKey {
    // https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes?redirectedfrom=MSDN
    fn from(code: u64) -> KeybdKey {
        match code {
            0x08 => BackspaceKey,
            0x09 => TabKey,
            0x0D => EnterKey,
            0x1B => EscapeKey,
            0x20 => SpaceKey,
            0x21 => PageUpKey,
            0x22 => PageDownKey,
            0x23 => EndKey,
            0x24 => HomeKey,
            0x25 => LeftKey,
            0x26 => UpKey,
            0x27 => RightKey,
            0x28 => DownKey,
            0x2D => InsertKey,
            0x2E => DeleteKey,
            0x30 => Numrow0Key,
            0x31 => Numrow1Key,
            0x32 => Numrow2Key,
            0x33 => Numrow3Key,
            0x34 => Numrow4Key,
            0x35 => Numrow5Key,
            0x36 => Numrow6Key,
            0x37 => Numrow7Key,
            0x38 => Numrow8Key,
            0x39 => Numrow9Key,
            0x41 => AKey,
            0x42 => BKey,
            0x43 => CKey,
            0x44 => DKey,
            0x45 => EKey,
            0x46 => FKey,
            0x47 => GKey,
            0x48 => HKey,
            0x49 => IKey,
            0x4A => JKey,
            0x4B => KKey,
            0x4C => LKey,
            0x4D => MKey,
            0x4E => NKey,
            0x4F => OKey,
            0x50 => PKey,
            0x51 => QKey,
            0x52 => RKey,
            0x53 => SKey,
            0x54 => TKey,
            0x55 => UKey,
            0x56 => VKey,
            0x57 => WKey,
            0x58 => XKey,
            0x59 => YKey,
            0x5A => ZKey,
            0x5B => LSuper,
            0x5C => RSuper,
            0x60 => Numpad0Key,
            0x61 => Numpad1Key,
            0x62 => Numpad2Key,
            0x63 => Numpad3Key,
            0x64 => Numpad4Key,
            0x65 => Numpad5Key,
            0x66 => Numpad6Key,
            0x67 => Numpad7Key,
            0x68 => Numpad8Key,
            0x69 => Numpad9Key,
            0x70 => F1Key,
            0x71 => F2Key,
            0x72 => F3Key,
            0x73 => F4Key,
            0x74 => F5Key,
            0x75 => F6Key,
            0x76 => F7Key,
            0x77 => F8Key,
            0x78 => F9Key,
            0x79 => F10Key,
            0x7A => F11Key,
            0x7B => F12Key,
            0x7C => F13Key,
            0x7D => F14Key,
            0x7E => F15Key,
            0x7F => F16Key,
            0x80 => F17Key,
            0x81 => F18Key,
            0x82 => F19Key,
            0x83 => F20Key,
            0x84 => F21Key,
            0x85 => F22Key,
            0x86 => F23Key,
            0x87 => F24Key,
            0x90 => NumLockKey,
            0x91 => ScrollLockKey,
            0x14 => CapsLockKey,
            0xA0 => LShiftKey,
            0xA1 => RShiftKey,
            0xA2 => LControlKey,
            0xA3 => RControlKey,
            0xA4 => LAltKey,
            0xA5 => RAltKey,
            0xA6 => BrowserBackKey,
            0xA7 => BrowserForwardKey,
            0xA8 => BrowserRefreshKey,
            0xAD => VolumeMuteKey,
            0xAE => VolumeDownKey,
            0xAF => VolumeUpKey,
            0xB0 => MediaNextTrackKey,
            0xB1 => MediaPrevTrackKey,
            0xB2 => MediaStopKey,
            0xB3 => MediaPlayPauseKey,
            0xC0 => BackquoteKey,
            0xBF => SlashKey,
            0xDC => BackslashKey,
            0xBC => CommaKey,
            0xBE => PeriodKey,
            0xBD => MinusKey,
            0xDE => QuoteKey,
            0xBA => SemicolonKey,
            0xDB => LBracketKey,
            0xDD => RBracketKey,
            0xBB => EqualKey,
            0x13 => PauseKey,
            0x2C => PrintScreenKey,
            0x5D => AppsKey,
            _ => OtherKey(code),
        }
    }
}

impl From<MouseButton> for u32 {
    fn from(button: MouseButton) -> u32 {
        match button {
            LeftButton => 0x01,
            RightButton => 0x02,
            MiddleButton => 0x04,
            X1Button => 0x05,
            X2Button => 0x06,
            MousewheelDown => 0x07,
            MousewheelUp => 0x08,
            OtherButton(code) => code,
        }
    }
}
