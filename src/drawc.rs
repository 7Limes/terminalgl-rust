// Draw in color.
use super::{Direction, TextAlignment};


/// Color type for `rgb_to_ccode`. Can be either foreground (`fg`) or background (`bg`).
pub enum ColorKind {
    Fg,
    Bg
}


pub const RESET: &str = "\x1b[0m";
pub const BLACK: &str = "\x1b[30m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[37m";

pub const BRIGHT_BLACK: &str = "\x1b[90m";
pub const BRIGHT_RED: &str = "\x1b[91m";
pub const BRIGHT_GREEN: &str = "\x1b[92m";
pub const BRIGHT_YELLOW: &str = "\x1b[93m";
pub const BRIGHT_BLUE: &str = "\x1b[94m";
pub const BRIGHT_MAGENTA: &str = "\x1b[95m";
pub const BRIGHT_CYAN: &str = "\x1b[96m";
pub const BRIGHT_WHITE: &str = "\x1b[97m";

pub const BLACK_BG: &str = "\x1b[40m";
pub const RED_BG: &str = "\x1b[41m";
pub const GREEN_BG: &str = "\x1b[42m";
pub const YELLOW_BG: &str = "\x1b[43m";
pub const BLUE_BG: &str = "\x1b[44m";
pub const MAGENTA_BG: &str = "\x1b[45m";
pub const CYAN_BG: &str = "\x1b[46m";
pub const WHITE_BG: &str = "\x1b[47m";

pub const BRIGHT_BLACK_BG: &str = "\x1b[100m";
pub const BRIGHT_RED_BG: &str = "\x1b[101m";
pub const BRIGHT_GREEN_BG: &str = "\x1b[102m";
pub const BRIGHT_YELLOW_BG: &str = "\x1b[103m";
pub const BRIGHT_BLUE_BG: &str = "\x1b[104m";
pub const BRIGHT_MAGENTA_BG: &str = "\x1b[105m";
pub const BRIGHT_CYAN_BG: &str = "\x1b[106m";
pub const BRIGHT_WHITE_BG: &str = "\x1b[107m";


fn distance(x1: isize, y1: isize, x2: isize, y2: isize) -> f64 {
    let x = ((x2-x1).pow(2) + (y2-y1).pow(2)) as f64;
    x.sqrt()
}


/// Convert an rgb triplet (`rgb`) into an ANSI color code.
/// Use `kind` to specify either foreground or background text color.
/// 
/// Examples
/// ```
/// use terminalgl::drawc;
/// let col1 = drawc::rgb_to_ccode((0, 255, 255), drawc::ColorKind::Fg);  // Cyan foreground
/// let col2 = drawc::rgb_to_ccode((255, 0, 0), drawc::ColorKind::Fg) + drawc::rgb_to_ccode((0, 255, 0), drawc::ColorKind::Bg);  // Red foreground and green background
/// ```
pub fn rgb_to_ccode(rgb: (u8, u8, u8), kind: ColorKind) -> String {
    let (r, g, b) = rgb;
    match kind {
        ColorKind::Fg => format!("\x1b[38;2;{};{};{}m", r, g, b),
        ColorKind::Bg => format!("\x1b[48;2;{};{};{}m", r, g, b)
    }
}


/// Draw character `c` at `(x, y)`.
/// 
/// Example
/// ```
/// use terminalgl as tgl;
/// tgl::drawc::pixel(1, 1, '#', tgl::drawc::RED);
/// ```
pub fn pixel(x: isize, y: isize, c: char, ccode: &str) {
    let tsize = super::size();
    if x >= 0 && x < tsize.0 as isize && y >= 0 && y < tsize.1 as isize && ccode.starts_with('\x1b') {
        super::cursorto(x as usize, y as usize);
        print!("{}{}", ccode, c);
    }
}


/// Draw a straight line of `c` starting at `(x, y)` with length `length` in direction `dir`.
/// 
/// Example
/// ```
/// use terminalgl as tgl;
/// use terminalgl::drawc::Direction;
/// tgl::drawc::straight_line(1, 2, 5, Direction::Right, '#', tgl::drawc::RED);
/// ```
pub fn straight_line(mut x: isize, mut y: isize, mut length: isize, dir: Direction, c: char, ccode: &str) {
    let mut addx: isize = 0;
    let mut addy: isize = 0;
    match dir {
        Direction::Left => addx = -1,
        Direction::Right => addx = 1,
        Direction::Up => addy = -1,
        Direction::Down => addy = 1
    }
    
    if length < 0 {
        length = -length;
        addx = -addx;
        addy = -addy;
    }
    
    for _ in 0..length {
        pixel(x, y, c, ccode);
        x += addx;
        y += addy;
    }
}


/// Draw a rectangle of `c` at `(x, y)` with width `width` and height `height`.
/// Use `fill` to specify whether the rectangle is outlined (`false`) or filled (`true`).
/// 
/// Example
/// ```
/// use terminalgl as tgl;
/// tgl::drawc::rectangle(1, 1, 7, 4, '#', tgl::drawc::RED, false);
/// ```
pub fn rectangle(x: isize, y: isize, width: usize, height: usize, c: char, ccode: &str, fill: bool) {
    let w = width as isize;
    let h = height as isize;
    if fill {
        for i in 0..width {
            straight_line(x+i as isize, y, height as isize, Direction::Down, c, ccode);
        }
        return;
    }
    straight_line(x, y, w, Direction::Right, c, ccode);
    straight_line(x, y+h-1, w, Direction::Right, c, ccode);
    straight_line(x, y+1, h-2, Direction::Down, c, ccode);
    straight_line(x+w-1, y+1, h-2, Direction::Down, c, ccode);
}


/// Draw a line of `c` with starting point `(x1, y1)` and ending point (`x2, y2`).
/// 
/// Example
/// ```
/// use terminalgl as tgl;
/// tgl::drawc::line(1, 1, 6, 3, '#', tgl::drawc::RED);
/// ```
pub fn line(x1: isize, y1: isize, x2: isize, y2: isize, c: char, ccode: &str) {
    if x1 == x2 {
        straight_line(x1, y1, y2-y1, Direction::Down, c, ccode);
    }
    if y1 == y2 {
        straight_line(x1, y1, x2-x1, Direction::Right, c, ccode);
    }
    
    let dist = distance(x1, y1, x2, y2);
    let dx = (x2-x1) as f64 / dist;
    let dy = (y2-y1) as f64 / dist;
    let mut x = x1 as f64;
    let mut y = y1 as f64;
    for _ in 0..dist.round() as isize {
        pixel(x.round() as isize, y.round() as isize, c, ccode);
        x += dx;
        y += dy;
    }
    pixel(x2, y2, c, ccode);
}


/// Draw an ellipse at `(h, k)` with width `a` and height `b`.
/// Use `fill` to specify whether the ellipse is outlined (`false`) or filled (`true`).
/// 
/// Example
/// ```
/// use terminalgl as tgl;
/// tgl::drawc::ellipse(5, 5, 4, 3, '#', tgl::drawc::RED, true);
/// ```
pub fn ellipse(h: isize, k: isize, a: usize, b: usize, c: char, ccode: &str, fill: bool) {
    for x in 0..a*2+1 {
        let shiftx: isize = x as isize + h - a as isize;
        let inside_y = ((a*a) as isize - (shiftx-h).pow(2)).abs() as f64;
        let y: f64 = (b as f64) / (a as f64) * inside_y.sqrt() + k as f64;
        if fill {
            let ydist: isize = 2 * (k - y.round() as isize).abs();
            straight_line(shiftx as isize, y.round() as isize, ydist+1, Direction::Up, c, ccode);
            continue;
        }
        let ydist: isize = 2 * (k - y.round() as isize);
        pixel(shiftx, y.round() as isize, c, ccode);
        pixel(shiftx, (y.round() as isize + ydist) as isize, c, ccode);
    }
}


/// Draw `text` starting at `(x, y)`.
/// 
/// Example
/// ```
/// use terminalgl as tgl;
/// let s = String::from("sample text");
/// tgl::drawc::text(1, 1, &s, tgl::drawc::RED);
/// ```
pub fn text(x: isize, y: isize, text: &str, ccode: &str) {
    for (i, c) in text.chars().enumerate() {
        pixel(x+i as isize, y, c, ccode);
    }
}


/// Draw `text` starting at `(x, y)` with alignment `align`.
/// 
/// Examples
/// ```
/// use terminalgl as tgl;
/// use tgl::TextAlignment::*;
/// tgl::drawc::text_aligned(1, 1, "sample text", Left, tgl::drawc::RED);
/// tgl::drawc::text_aligned(10, 3, "sample text", Center, tgl::drawc::GREEN);
/// tgl::drawc::text_aligned(15, 5, "sample text", Right, tgl::drawc::BLUE);
/// ```
pub fn text_aligned(x: isize, y: isize, text: &str, align: TextAlignment, ccode: &str) {
    let mut x = x;
    match align {
        TextAlignment::Left => {},
        TextAlignment::Center => x -= text.len() as isize / 2,
        TextAlignment::Right => x -= text.len() as isize
    }
    for (i, c) in text.chars().enumerate() {
        pixel(x+i as isize, y, c, ccode);
    }
}
