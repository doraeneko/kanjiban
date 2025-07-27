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
        let mut level = Level {
            unmovable_blocks: LinkedList::new(),
            movable_blocks: LinkedList::new(),
            sinks: LinkedList::new(),
        };
        build_level0(&mut level.unmovable_blocks, 
                     &mut level.movable_blocks, 
                     &mut level.sinks);

        return level;
    }

    // taken from wikipedia
    fn build_level0(
        unmovable_blocks: &mut LinkedList<Point>,
        movable_blocks: &mut LinkedList<Point>,
        sinks: &mut LinkedList<Point>,
    ) {
        // place none movable blocks
        for i in 0..5 {
            unmovable_blocks.push_front((4, i));
            unmovable_blocks.push_front((12, i));
        }
        for i in 4..13 {
            unmovable_blocks.push_front((i, 0));
            unmovable_blocks.push_front((i, 5));
        }
        for i in 4..7 {
            unmovable_blocks.push_front((i, 1));
        }
        for i in 9..12 {
            unmovable_blocks.push_front((i, 1));
        }
        // add some none-movable blocks in the middle
        unmovable_blocks.push_front((9, 3));
        unmovable_blocks.push_front((9, 4));
        unmovable_blocks.push_front((6, 3));

        // add movable boxes
        movable_blocks.push_front((10, 2));
        movable_blocks.push_front((10, 3));

        // add sinks
        sinks.push_front((8, 4));
        sinks.push_front((6, 4));
    }
}