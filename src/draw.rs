/// Draw without color.

/// Direction for `straight_line`.
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}


/// Get euclidean distance between two points.
fn distance(x1: isize, y1: isize, x2: isize, y2: isize) -> f64 {
    let x = ((x2-x1).pow(2) + (y2-y1).pow(2)) as f64;
    x.sqrt()
}


/// Draw character `c` at `(x, y)`.
/// 
/// Example
/// ```
/// use terminalgl as tgl;
/// tgl::draw::pixel(1, 1, '#');
/// ```
pub fn pixel(x: isize, y: isize, c: char) {
    //let tsize = super::size();
    if x >= 0 && y >= 0 {
        super::cursorto(x as usize, y as usize);
        print!("{}", c);
    }
}


/// Draw a straight line of `c` starting at `(x, y)` with length `length` in direction `dir`.
/// 
/// Example
/// ```
/// use terminalgl as tgl;
/// use terminalgl::draw::Direction;
/// tgl::draw::straight_line(1, 2, 5, Direction::Right, '#');
/// ```
pub fn straight_line(mut x: isize, mut y: isize, mut length: isize, dir: Direction, c: char) {
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
        pixel(x, y, c);
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
/// tgl::draw::rectangle(1, 1, 7, 4, '#', false);
/// ```
pub fn rectangle(x: isize, y: isize, width: usize, height: usize, c: char, fill: bool) {
    let w = width as isize;
    let h = height as isize;
    if fill {
        for i in 0..width {
            straight_line(x+i as isize, y, height as isize, Direction::Down, c);
        }
        return;
    }
    straight_line(x, y, w, Direction::Right, c);
    straight_line(x, y+h-1, w, Direction::Right, c);
    straight_line(x, y+1, h-2, Direction::Down, c);
    straight_line(x+w-1, y+1, h-2, Direction::Down, c);
}


/// Draw a line of `c` with starting point `(x1, y1)` and ending point (`x2, y2`).
/// 
/// Example
/// ```
/// use terminalgl as tgl;
/// tgl::draw::line(1, 1, 6, 3, '#');
/// ```
pub fn line(x1: isize, y1: isize, x2: isize, y2: isize, c: char) {
    if x1 == x2 {
        straight_line(x1, y1, y2-y1, Direction::Down, c);
    }
    if y1 == y2 {
        straight_line(x1, y1, x2-x1, Direction::Right, c);
    }
    
    let dist = distance(x1, y1, x2, y2);
    let dx = (x2-x1) as f64 / dist;
    let dy = (y2-y1) as f64 / dist;
    let mut x = x1 as f64;
    let mut y = y1 as f64;
    for _ in 0..dist.round() as isize {
        pixel(x.round() as isize, y.round() as isize, c);
        x += dx;
        y += dy;
    }
    pixel(x2, y2, c);
}


/// Draw an ellipse at `(h, k)` with width `a` and height `b`.
/// Use `fill` to specify whether the ellipse is outlined (`false`) or filled (`true`).
/// 
/// Example
/// ```
/// use terminalgl as tgl;
/// tgl::draw::ellipse(5, 5, 4, 3, '#', true);
/// ```
pub fn ellipse(h: isize, k: isize, a: usize, b: usize, c: char, fill: bool) {
    for x in 0..a*2+1 {
        let shiftx: isize = x as isize + h - a as isize;
        let inside_y = ((a*a) as isize - (shiftx-h).pow(2)).abs() as f64;
        let y: f64 = (b as f64) / (a as f64) * inside_y.sqrt() + k as f64;
        if fill {
            let ydist: isize = 2 * (k - y.round() as isize).abs();
            straight_line(shiftx as isize, y.round() as isize, ydist+1, Direction::Up, c);
            continue;
        }
        let ydist: isize = 2 * (k - y.round() as isize);
        pixel(shiftx, y.round() as isize, c);
        pixel(shiftx, (y.round() as isize + ydist) as isize, c);
    }
}


/// Draw `text` starting at `(x, y)`.
/// 
/// Example
/// ```
/// use terminalgl as tgl;
/// let s = String::from("sample text");
/// tgl::draw::text(1, 1, &s);
/// ```
pub fn text(x: isize, y: isize, text: &str) {
    for (i, c) in text.chars().enumerate() {
        pixel(x+i as isize, y, c);
    }
}
