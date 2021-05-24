pub mod core;
pub mod zugzwang;

pub fn clear() {
    print!("{esc}c", esc = 27 as char);
}