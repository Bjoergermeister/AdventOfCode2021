pub struct Line {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl Line {
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        return Self { x1, y1, x2, y2 };
    }

    pub fn len(&self) -> i32 {
        if self.x1 == self.x2 {
            return (self.y2 - self.y1).abs();
        }else{
            return (self.x2 - self.x1).abs();
        }
    }

    pub fn is_diagonal(&self) -> bool {
        return self.x1 != self.x2 && self.y1 != self.y2;
    }
}