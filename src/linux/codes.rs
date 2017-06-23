use Code;
use linux::x11::*;
use linux::x11::xlib::*;
use linux::get_display;
use *;

fn get_code(keysym: u8) -> u8 {
    unsafe{XKeysymToKeycode(get_display(), keysym as _)}
}

pub fn lbutton() -> Code {get_code(0x01)}
pub fn rbutton() -> Code {get_code(0x02)}
pub fn mbutton() -> Code {get_code(0x04)}
pub fn xbutton1() -> Code {get_code(0x05)}
pub fn xbutton2() -> Code {get_code(0x06)}
pub fn backspace() -> Code {get_code(0x08)}
pub fn tab() -> Code {get_code(0x09)}
pub fn enter() -> Code {get_code(0x0D)}
pub fn shift() -> Code {get_code(0x10)}
pub fn control() -> Code {get_code(0x11)}
pub fn escape() -> Code {get_code(0x1B)}
pub fn space() -> Code {get_code(0x20)}
pub fn home() -> Code {get_code(0x24)}
pub fn left() -> Code {get_code(0x25)}
pub fn up() -> Code {get_code(0x26)}
pub fn right() -> Code {get_code(0x27)}
pub fn down() -> Code {get_code(0x28)}
pub fn insert() -> Code {get_code(0x2D)}
pub fn delete() -> Code {get_code(0x2E)}
pub fn num0() -> Code {get_code(0x30)}
pub fn num1() -> Code {get_code(0x31)}
pub fn num2() -> Code {get_code(0x32)}
pub fn num3() -> Code {get_code(0x33)}
pub fn num4() -> Code {get_code(0x34)}
pub fn num5() -> Code {get_code(0x35)}
pub fn num6() -> Code {get_code(0x36)}
pub fn num7() -> Code {get_code(0x37)}
pub fn num8() -> Code {get_code(0x38)}
pub fn num9() -> Code {get_code(0x39)}
pub fn a() -> Code {get_code(0x41)}
pub fn b() -> Code {get_code(0x42)}
pub fn c() -> Code {get_code(0x43)}
pub fn d() -> Code {get_code(0x44)}
pub fn e() -> Code {get_code(0x45)}
pub fn f() -> Code {get_code(0x46)}
pub fn g() -> Code {get_code(0x47)}
pub fn h() -> Code {get_code(0x48)}
pub fn i() -> Code {get_code(0x49)}
pub fn j() -> Code {get_code(0x4A)}
pub fn k() -> Code {get_code(0x4B)}
pub fn l() -> Code {get_code(0x4C)}
pub fn m() -> Code {get_code(0x4D)}
pub fn n() -> Code {get_code(0x4E)}
pub fn o() -> Code {get_code(0x4f)}
pub fn p() -> Code {get_code(0x50)}
pub fn q() -> Code {get_code(0x51)}
pub fn r() -> Code {get_code(0x52)}
pub fn s() -> Code {get_code(0x53)}
pub fn t() -> Code {get_code(0x54)}
pub fn u() -> Code {get_code(0x55)}
pub fn v() -> Code {get_code(0x56)}
pub fn w() -> Code {get_code(0x57)}
pub fn x() -> Code {get_code(0x58)}
pub fn y() -> Code {get_code(0x59)}
pub fn z() -> Code {get_code(0x5A)}
pub fn numpad0() -> Code {get_code(0x60)}
pub fn numpad1() -> Code {get_code(0x61)}
pub fn numpad2() -> Code {get_code(0x62)}
pub fn numpad3() -> Code {get_code(0x63)}
pub fn numpad4() -> Code {get_code(0x64)}
pub fn numpad5() -> Code {get_code(0x65)}
pub fn numpad6() -> Code {get_code(0x66)}
pub fn numpad7() -> Code {get_code(0x67)}
pub fn numpad8() -> Code {get_code(0x68)}
pub fn numpad9() -> Code {get_code(0x69)}
pub fn f1() -> Code {get_code(0x70)}
pub fn f2() -> Code {get_code(0x71)}
pub fn f3() -> Code {get_code(0x72)}
pub fn f4() -> Code {get_code(0x73)}
pub fn f5() -> Code {get_code(0x74)}
pub fn f6() -> Code {get_code(0x75)}
pub fn f7() -> Code {get_code(0x76)}
pub fn f8() -> Code {get_code(0x77)}
pub fn f9() -> Code {get_code(0x78)}
pub fn f10() -> Code {get_code(0x79)}
pub fn f11() -> Code {get_code(0x7A)}
pub fn f12() -> Code {get_code(0x7B)}
pub fn f13() -> Code {get_code(0x7C)}
pub fn f14() -> Code {get_code(0x7D)}
pub fn f15() -> Code {get_code(0x7E)}
pub fn f16() -> Code {get_code(0x7f)}
pub fn f17() -> Code {get_code(0x80)}
pub fn f18() -> Code {get_code(0x81)}
pub fn f19() -> Code {get_code(0x82)}
pub fn f20() -> Code {get_code(0x83)}
pub fn f21() -> Code {get_code(0x84)}
pub fn f22() -> Code {get_code(0x85)}
pub fn f23() -> Code {get_code(0x86)}
pub fn f24() -> Code {get_code(0x87)}
pub fn num_lock() -> Code {get_code(0x90)}
pub fn scroll_lock() -> Code {get_code(0x91)}
pub fn lshift() -> Code {get_code(0xA0)}
pub fn rshift() -> Code {get_code(0xA1)}
pub fn lcontrol() -> Code {get_code(0xA2)}
pub fn rcontrol() -> Code {get_code(0xA3)}