use macroquad::prelude::*;

use std::collections::LinkedList;

const SQUARES: i16 = 16;

type Point = (i16, i16);

struct Player {
    position: Point,
    dir: Point,
}

struct Block {
    position: Point,
}

#[macroquad::main("Kanjiban")]
async fn main() {
    let mut andi = Player {
        position: (4, 4),
        dir: (1, 0),
    };
    let mut unmovable_blocks: LinkedList<Block> = LinkedList::new();
    let mut movable_blocks: LinkedList<Block> = LinkedList::new();
    for i in 0..SQUARES {
        let block = Block {
            position: (0, i),
        };
        let block2 = Block {
            position: (SQUARES, i),
        };
        unmovable_blocks.push_front(block);
        unmovable_blocks.push_front(block2);
    }
    for i in 0..SQUARES {
        let block = Block {
            position: (i, 0),
        };
        let block2 = Block {
            position: (i, SQUARES),
        };
        unmovable_blocks.push_front(block);
        unmovable_blocks.push_front(block2);
    }
    // add some none-movable blocks in the middle
    let block = Block {
        position: (5, 5),
    };
    let block2 = Block {
        position: (7, 7),
    };
    unmovable_blocks.push_front(block);
    unmovable_blocks.push_front(block2);
   
    let block3 = Block {
        position: (5, 9),
    };
    movable_blocks.push_front(block3);


    let mut steps = 0;
    let speed = 0.1;
    let mut last_update = get_time();
    let game_over = false;

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
            } else if is_key_down(KeyCode::Down){
                andi.dir = down;     
            }

            if get_time() - last_update > speed {
                last_update = get_time();
                let next_position : Point = (andi.position.0 + andi.dir.0, andi.position.1 + andi.dir.1);
                let mut player_can_move = true;
                for b in &unmovable_blocks {
                    if b.position == next_position {
                        // player likes to move on this tile
                        player_can_move = false;
                    }
                }
                for b in &mut movable_blocks {
                    // player likes to move on a tile with a movable block
                    if b.position == next_position {
                        let mut block_movable = true;
                        let next_block_position : Point = (b.position.0 + andi.dir.0, b.position.1 + andi.dir.1);
                        // check if the block can be moved
                        for b2 in &unmovable_blocks {
                            if b2.position == next_block_position {
                                // cannot move the block
                                block_movable = false;
                                player_can_move = false;
                            } 
                        }
                        if block_movable == true {
                            // move the block
                            b.position = next_block_position;
                        }
                    }
                }
                if player_can_move {
                   andi.position = next_position;
                   steps += 1;
                }
            }
        }
        if !game_over {
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

            draw_rectangle(
                offset_x + andi.position.0 as f32 * sq_size,
                offset_y + andi.position.1 as f32 * sq_size,
                sq_size,
                sq_size,
                DARKGREEN,
            );

            for b in &unmovable_blocks {
                draw_rectangle(
                    offset_x + b.position.0 as f32 * sq_size,
                    offset_y + b.position.1 as f32 * sq_size,
                    sq_size,
                    sq_size,
                    RED,
                );
            }

            for b in &movable_blocks {
                draw_rectangle(
                    offset_x + b.position.0 as f32 * sq_size,
                    offset_y + b.position.1 as f32 * sq_size,
                    sq_size,
                    sq_size,
                    GOLD,
                );
            }

            draw_text(format!("Steps: {steps}").as_str(), 10., 20., 20., DARKGRAY);
        } else {
            clear_background(WHITE);
            let text = "Game Over. Press [enter] to play again.";
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
