pub mod board {
    pub const SQUARES: i16 = 16;

    pub type Point = (i16, i16);

    pub struct Player {
        pub position: Point,
        pub dir: Point,
    }
}