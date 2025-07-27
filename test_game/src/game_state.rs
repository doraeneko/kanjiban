pub mod board {
    use std::collections::LinkedList;

    pub const SQUARES: i16 = 16;

    pub type Point = (i16, i16);

    pub struct Player {
        pub position: Point,
        pub dir: Point,
    }

    pub struct Level {
        pub unmovable_blocks: LinkedList<Point>,
        pub movable_blocks: LinkedList<Point>,
        pub sinks: LinkedList<Point>,
    }

    pub fn new_level() -> Level {
        return Level {
            unmovable_blocks: LinkedList::new(),
            movable_blocks: LinkedList::new(),
            sinks: LinkedList::new(),
        };
    }
}