/// An enum used for the `Surface.draw_straight_line` method.
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

/// Main graphical struct.
pub struct Surface {
    width: usize,
    height: usize,
    data: Vec<Vec<char>>,
}

impl Surface {
    fn distance(x1: isize, y1: isize, x2: isize, y2: isize) -> f64 {
        let x = ((x2-x1).pow(2) + (y2-y1).pow(2)) as f64;
        x.sqrt()
    }

    /// Creates a new `Surface` of width `width` and height `height` filled with spaces.
    /// 
    /// Example
    /// ```
    /// use terminalgl::Surface;
    /// let mut surf = Surface::new(10, 10);
    /// ```
    pub fn new(width: usize, height: usize) -> Surface {
        let v = vec![vec![' '; width]; height];
        Surface {
            width,
            height,
            data: v
        }
    }

    /// Get the width of this surface.
    /// 
    /// Example
    /// ```
    /// use terminalgl::Surface;
    /// let mut surf = Surface::new(10, 10);
    /// println!("{}", surf.width());
    /// ```
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get the height of this surface.
    /// 
    /// Example
    /// ```
    /// use terminalgl::Surface;
    /// let mut surf = Surface::new(10, 10);
    /// println!("{}", surf.height());
    /// ```
    pub fn height(&self) -> usize {
        self.height
    }

    /// Get a reference to the raw vector data of this surface.
    /// 
    /// Example:
    /// ```
    /// use terminalgl::Surface;
    /// let mut surf = Surface::new(10, 10);
    /// println!("{:?}", surf.raw_data());
    /// ```
    pub fn get_raw_data(&self) -> &Vec<Vec<char>> {
        &self.data
    }

    /// Fill this surface with `c`.
    /// 
    /// Example
    /// ```
    /// use terminalgl::Surface;
    /// let mut surf = Surface::new(10, 10);
    /// surf.fill('.');
    /// ```
    pub fn fill(&mut self, c: char) {
        self.data.clear();
        self.data = vec![vec![c; self.width]; self.height];
    }

    /// Display this surface to the terminal.
    /// 
    /// Example
    /// ```
    /// use terminalgl::Surface;
    /// let mut surf = Surface::new(10, 10);
    /// surf.fill('.');
    /// surf.display();
    /// ```
    pub fn display(&self) {
        for row in &self.data {
            let row_str = row.iter().cloned().collect::<String>();
            println!("{}", row_str);
        }
    }

    /// Draw character `c` at `(x, y)`.
    /// 
    /// Example
    /// ```
    /// use terminalgl::Surface;
    /// let mut surf = Surface::new(10, 10);
    /// surf.draw_pixel(2, 2, '#');
    /// ```
    pub fn draw_pixel(&mut self, x: isize, y: isize, c: char) -> bool {
        let mut placed = false;
        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            self.data[y as usize][x as usize] = c;
            placed = true;
        }
        placed
    }

    /// Draw a straight line using `c` starting at `(x, y)` with length `length` in direction `dir`.
    /// 
    /// Example
    /// ```
    /// use terminalgl::{Surface, Direction};
    /// let mut surf = Surface::new(10, 10);
    /// surf.draw_straight_line(1, 1, 5, Direction::Right, '#');
    /// ```
    pub fn draw_straight_line(&mut self, mut x: isize, mut y: isize, mut length: isize, dir: Direction, c: char) {
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
            self.draw_pixel(x, y, c);
            x += addx;
            y += addy;
        }
    }

    /// Draw a rectangle using `c` with the top-left corner at `(x, y)` with width `width` and height `height`.
    /// If `fill` is false, only the outline of the rectangle will be drawn. Otherwise the entire rectangle will be filled.
    /// 
    /// Example
    /// ```
    /// use terminalgl::Surface;
    /// let mut surf = Surface::new(10, 10);
    /// surf.draw_rectangle(1, 1, 5, 5, '#', true);
    /// ```
    pub fn draw_rectangle(&mut self, x: isize, y: isize, width: usize, height: usize, c: char, fill: bool) {
        if !fill {
            self.draw_straight_line(x, y, width as isize, Direction::Right, c);
            self.draw_straight_line(x, y+(height as isize)-1, width as isize, Direction::Right, c);
            self.draw_straight_line(x, y+1, (height-2) as isize, Direction::Down, c);
            self.draw_straight_line(x+(width as isize)-1, y, height as isize-2, Direction::Down, c);
        }
        else {
            for i in 0..width {
                self.draw_straight_line(x+i as isize, y, height as isize, Direction::Down, c);
            }
        }
    }

    /// Draw a line using `c` with starting point `(x1, y1)` and ending point `(x2, y2)`.
    /// 
    /// Example
    /// ```
    /// use terminalgl::Surface;
    /// let mut surf = Surface::new(10, 10);
    /// surf.draw_line(1, 1, 5, 5, '#');
    /// ```
    pub fn draw_line(&mut self, x1: isize, y1: isize, x2: isize, y2: isize, c: char) {
        if x1 == x2 {
            self.draw_straight_line(x1, y1, y2-y1, Direction::Down, c);
        }
        if y1 == y2 {
            self.draw_straight_line(x1, y1, x2-x1, Direction::Right, c);
        }
        
        let dist = Surface::distance(x1, y1, x2, y2);
        let dx = (x2-x1) as f64 / dist;
        let dy = (y2-y1) as f64 / dist;
        let mut x = x1 as f64;
        let mut y = y1 as f64;
        for _ in 0..dist.round() as isize {
            self.draw_pixel(x.round() as isize, y.round() as isize, c);
            x += dx;
            y += dy;
        }
        self.draw_pixel(x2, y2, c);
    }

    /// Draw an ellipse using `c` with a center at `(h, k)` with a width of `width` (on both sides) and a height of `height` (on both sides).
    /// If `fill` is false, only the outline of the ellipse will be drawn. Otherwise, the entire ellipse will be filled.
    /// 
    /// Example
    /// ```
    /// use terminalgl::Surface;
    /// let mut surf = Surface::new(10, 10);
    /// surf.draw_ellipse(4, 4, 3, 3, '#', false);
    /// ```
    pub fn draw_ellipse(&mut self, h: isize, k: isize, a: usize, b: usize, c: char, fill: bool) {
        for x in 0..a*2+1 {
            let shiftx: isize = x as isize + h - a as isize;
            let inside = ((a*a) as isize - (shiftx-h).pow(2)).abs() as f64;
            let y: f64 = (b as f64) / (a as f64) * inside.sqrt() + k as f64;
            if fill {
                let ydist: isize = 2 * (k - y.round() as isize).abs();
                self.draw_straight_line(shiftx as isize, y.round() as isize, ydist+1, Direction::Up, c);
                continue;
            }
            let ydist: isize = 2 * (k - y.round() as isize);
            self.draw_pixel(shiftx, y.round() as isize, c);
            self.draw_pixel(shiftx, (y.round() as isize + ydist) as isize, c);
        }
    }

    /// Draw a polygon using the points specified in `points`.
    /// `points` must have a length of at least 2.
    /// Sub-vectors of `points` must be `2` or more elements long.
    /// 
    /// Example
    /// ```
    /// use terminalgl::Surface;
    /// let mut surf = Surface::new(10, 10);
    /// let pts = vec![vec![1, 1], vec![9, 3], vec![5, 7]];
    /// surf.draw_polygon(&pts, '#');
    /// surf.display();
    /// ```
    pub fn draw_polygon(&mut self, points: &Vec<Vec<isize>>, c: char) {
        for i in 0..points.len() {
            let mut next = i+1;
            if next >= points.len() {
                next = 0;
            }
            self.draw_line(points[i][0], points[i][1], points[next][0], points[next][1], c);
        }
    }

    /// Draw a line of text starting at `(x, y)` moving to the right for each character in `text`.
    /// 
    /// Example
    /// ```
    /// use terminalgl::Surface;
    /// let mut surf = Surface::new(10, 10);
    /// let text = String::from("hi there");
    /// surf.draw_text(1, 1, &text);
    /// ```
    pub fn draw_text(&mut self, x: isize, y: isize, text: &String) {
        for (i, c) in text.chars().enumerate() {
            self.draw_pixel(x+i as isize, y, c);
        }
    }

    /// Display a surface `other` on top of this surface.
    /// Any space characters (`' '`) in `other` will be considered transparent and will not be overwritten in this surface.
    /// 
    /// Example
    /// ```
    /// use terminalgl::Surface;
    /// let mut s1 = Surface::new(10, 10);
    /// let mut s2 = Surface::new(5, 5);
    /// s1.fill('.');
    /// s2.fill('#');
    /// s1.blit(1, 1, &s2);
    /// ```
    pub fn blit(&mut self, x: isize, y: isize, other: &Surface) {
        for (i, row) in other.data.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if *c == ' ' {
                    continue;
                }
                self.draw_pixel(x+j as isize, y+i as isize, *c);
            }
        }
    }
}
