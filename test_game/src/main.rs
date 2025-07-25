use macroquad::prelude::*;

use std::collections::LinkedList;

const SQUARES: i16 = 16;

type Point = (i16, i16);

struct Player {
    position: Point,
    dir: Point,
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

// render all blocks, player and step counter
fn draw_game(
    unmovable_blocks: &mut LinkedList<Point>,
    movable_blocks: &mut LinkedList<Point>,
    sinks: &mut LinkedList<Point>,
    andi: &Player,
    steps: i16,
) {
    clear_background(LIGHTGRAY);

    let game_size = screen_width().min(screen_height());
    let offset_x = (screen_width() - game_size) / 2. + 10.;
    let offset_y = (screen_height() - game_size) / 2. + 10.;
    let sq_size = (screen_height() - offset_y * 2.) / SQUARES as f32;

    draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., WHITE);

    for i in 1..SQUARES {
        draw_line(
            offset_x,
            offset_y + sq_size * i as f32,
            screen_width() - offset_x,
            offset_y + sq_size * i as f32,
            2.,
            LIGHTGRAY,
        );
    }

    for i in 1..SQUARES {
        draw_line(
            offset_x + sq_size * i as f32,
            offset_y,
            offset_x + sq_size * i as f32,
            screen_height() - offset_y,
            2.,
            LIGHTGRAY,
        );
    }

    for s in sinks {
        draw_rectangle(
            offset_x + s.0 as f32 * sq_size,
            offset_y + s.1 as f32 * sq_size,
            sq_size,
            sq_size,
            GREEN,
        );
    }

    for b in unmovable_blocks {
        draw_rectangle(
            offset_x + b.0 as f32 * sq_size,
            offset_y + b.1 as f32 * sq_size,
            sq_size,
            sq_size,
            RED,
        );
    }

    for b in movable_blocks {
        draw_rectangle(
            offset_x + b.0 as f32 * sq_size,
            offset_y + b.1 as f32 * sq_size,
            sq_size,
            sq_size,
            GOLD,
        );
    }

    draw_rectangle(
        offset_x + andi.position.0 as f32 * sq_size,
        offset_y + andi.position.1 as f32 * sq_size,
        sq_size,
        sq_size,
        DARKGREEN,
    );

    draw_text(format!("Steps: {steps}").as_str(), 10., 20., 20., DARKGRAY);
}

fn all_boxes_on_sinks(movable_blocks: &LinkedList<Point>, sinks: &LinkedList<Point>) -> bool {
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

#[macroquad::main("Kanjiban")]
async fn main() {
    let mut andi = Player {
        position: (10, 4),
        dir: (1, 0),
    };
    let mut unmovable_blocks: LinkedList<Point> = LinkedList::new();
    let mut movable_blocks: LinkedList<Point> = LinkedList::new();
    let mut sinks: LinkedList<Point> = LinkedList::new();
    build_level0(&mut unmovable_blocks, &mut movable_blocks, &mut sinks);

    let mut steps = 0;
    let speed = 0.1;
    let mut last_update = get_time();
    let mut game_over = false;

    let up = (0, -1);
    let down = (0, 1);
    let right = (1, 0);
    let left = (-1, 0);

    loop {
        if !game_over {
            andi.dir = (0, 0);
            if is_key_down(KeyCode::Right) {
                andi.dir = right;
            } else if is_key_down(KeyCode::Left) {
                andi.dir = left;
            } else if is_key_down(KeyCode::Up) {
                andi.dir = up;
            } else if is_key_down(KeyCode::Down) {
                andi.dir = down;
            }

            if get_time() - last_update > speed {
                last_update = get_time();
                // player likes to move on this tile:
                let next_position: Point =
                    (andi.position.0 + andi.dir.0, andi.position.1 + andi.dir.1);
                let mut player_can_move = true;
                for b in &unmovable_blocks {
                    if *b == next_position {
                        // the tile is blocked
                        player_can_move = false;
                    }
                }

                if movable_blocks.contains(&next_position) {
                    let next_block_position =
                        (next_position.0 + andi.dir.0, next_position.1 + andi.dir.1);
                    if is_block_movable(&unmovable_blocks, next_block_position)
                        && is_block_movable(&movable_blocks, next_block_position)
                    {
                        //movable_blocks.iter().find(|x: Point | *x == next_position)
                        for b in &mut movable_blocks {
                            if *b == next_position {
                                let next_block_position = (b.0 + andi.dir.0, b.1 + andi.dir.1);
                                *b = next_block_position;
                            }
                        }
                    } else {
                        player_can_move = false;
                    }
                }

                if player_can_move && next_position != andi.position {
                    andi.position = next_position;
                    steps += 1;
                    game_over = all_boxes_on_sinks(&mut movable_blocks, &mut sinks);
                }
            }
        }
        if !game_over {
            draw_game(
                &mut unmovable_blocks,
                &mut movable_blocks,
                &mut sinks,
                &andi,
                steps,
            );
        } else {
            clear_background(WHITE);
            let text = "Game Over. ";
            let font_size = 30.;
            let text_size = measure_text(text, None, font_size as _, 1.0);

            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. + text_size.height / 2.,
                font_size,
                DARKGRAY,
            );
        }
        next_frame().await;
    }
}
