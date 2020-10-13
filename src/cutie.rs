pub struct Cutie {
    length: i32,
    width: i32,
    nw: Leaf,
    ne: Leaf,
    se: Leaf,
    sw: Leaf,
}

impl Cutie {
    pub fn new(length: i32, width: i32) -> Cutie {
        Cutie {
            length,
            width,
            nw: Leaf::new(),
            ne: Leaf::new(),
            se: Leaf::new(),
            sw: Leaf::new(),
        }
    }
}
struct Leaf {
    range: (i32, i32),
    points: Vec<(i32, i32)>,
    subdivided: bool,
    nw: Option<Box<Leaf>>,
    ne: Option<Box<Leaf>>,
    se: Option<Box<Leaf>>,
    sw: Option<Box<Leaf>>,
}

impl Leaf {
    pub fn new(range: (i32, i32)) -> Leaf {
        Leaf {
            range,
            points: Vec::new(),
            subdivided: false,
            nw: None,
            ne: None,
            se: None,
            sw: None,
        }
    }
}
