pub mod draw;
pub mod drawc;


/// Moves the cursor to `(x, y)` with the top left corner being `(0, 0)`.
pub fn cursorto(x: usize, y: usize) {
    print!("\x1b[{};{}H", y+1, x+1);
}


/// Clear the terminal.
pub fn clear() {
    print!("\x1b[2J\x1b[0;0H");
}


/// Get terminal size as `(cols, rows)`.
pub fn size() -> (u16, u16) {
    termsize::get().map(|size| {
        (size.cols, size.rows)
    }).unwrap()
}


/// Direction for `straight_line`.
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}


/// Alignment for `text_aligned`.
pub enum TextAlignment {
    Left,
    Right,
    Center
}
