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

    // next_position: The tile the player likes to move on
    // true if not blocked by wall
    pub fn is_blocked_by_a_wall(level: &Level, next_position: &Point) -> bool {
        for b in &level.unmovable_blocks {
            if b == next_position {
                // the tile is blocked
                return true;
            }
        }
        return false;
    }

    // checks im next position is a tile with a box and if this box can be moved
    // FIXME(SRP): also moves box
    pub fn box_is_blocked(level: &mut Level, andi: &Player, next_position: &Point) -> bool {
        if level.movable_blocks.contains(&next_position) {
            let next_block_position =
                (next_position.0 + andi.dir.0, next_position.1 + andi.dir.1);
            if is_block_movable(&level.unmovable_blocks, next_block_position)
                && is_block_movable(&level.movable_blocks, next_block_position)
            {
                //movable_blocks.iter().find(|x: Point | *x == next_position)
                for b in &mut level.movable_blocks {
                    if b == next_position {
                        let next_block_position = (b.0 + andi.dir.0, b.1 + andi.dir.1);
                        *b = next_block_position; // move the box
                    }
                }
            } else {
                return true;
            }
        }
        return false;
    }

    // check if a block can be moved to next_block_position
    fn is_block_movable(unmovable_blocks: &LinkedList<Point>, next_block_position: Point) -> bool {
        for b in unmovable_blocks {
            if *b == next_block_position {
                // cannot move the block
                return false;
            }
        }
        return true;
    }

    // checks if the level is solved
    pub fn all_boxes_on_sinks(movable_blocks: &LinkedList<Point>, sinks: &LinkedList<Point>) -> bool {
        for s in sinks {
            let mut sink_found = false;
            for b in movable_blocks {
                if b == s {
                    sink_found = true;
                }
            }
            if !sink_found {
                return false;
            }
        }
        return true;
    }
}