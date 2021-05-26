mod core;
mod zugzwang;

pub use self::core::*;
pub use zugzwang::*;

pub fn clear() {
    print!("{esc}c", esc = 27 as char);
}