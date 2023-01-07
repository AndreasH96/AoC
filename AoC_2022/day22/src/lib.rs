use std::collections::HashMap;

use num::Complex;
pub fn transition(
    from: &str,
    current_dir: usize,
    current_pos: Complex<i32>,
    block_size: i32,
) -> (usize, &str, Complex<i32>) {
    let (direction, face, position) = match (from, current_dir) {
        ("top", 0) => (3, "right", Complex::new(block_size-1, current_pos.re)),
        ("top", 1) => (1, "front", Complex::new(0, current_pos.im)),
        ("top", 2) => (1, "left", Complex::new(0, current_pos.re)),
        ("top", 3) => (3, "back", Complex::new(block_size - 1, current_pos.im)),

        ("front", 0) => (
            2,
            "right",
            Complex::new(block_size - current_pos.re - 1, block_size - 1),
        ),
        ("front", 1) => (2, "bottom", Complex::new(current_pos.im, block_size - 1)),
        ("front", 2) => (2, "left", Complex::new(current_pos.re, block_size - 1)),
        ("front", 3) => (3, "top", Complex::new(block_size - 1, current_pos.im)),

        ("back", 0) => (0, "right", Complex::new(current_pos.re, 0)),
        ("back", 1) => (1, "top", Complex::new(0, current_pos.im)),
        ("back", 2) => (0, "left", Complex::new(block_size - current_pos.re - 1, 0)),
        ("back", 3) => (0, "bottom", Complex::new(current_pos.im, 0)),

        ("right", 0) => (
            2,
            "front",
            Complex::new(block_size - current_pos.re - 1, block_size - 1),
        ),
        ("right", 1) => (2, "top", Complex::new(current_pos.im, block_size - 1)),
        ("right", 2) => (2, "back", Complex::new(current_pos.re, block_size - 1)),
        ("right", 3) => (3, "bottom", Complex::new(block_size - 1, current_pos.im)),

        ("left", 0) => (0, "front", Complex::new(current_pos.re, 0)),
        ("left", 1) => (1, "bottom", Complex::new(0, current_pos.im)),
        ("left", 2) => (0, "back", Complex::new(block_size - current_pos.re - 1, 0)),
        ("left", 3) => (0, "top", Complex::new(current_pos.im, 0)),

        ("bottom", 0) => (3, "front", Complex::new(block_size - 1, current_pos.re)),
        ("bottom", 1) => (1, "right", Complex::new(0, current_pos.im)),
        ("bottom", 2) => (1, "back", Complex::new(0, current_pos.re)),
        ("bottom", 3) => (3, "left", Complex::new(block_size - 1, current_pos.im)),
        _ => unreachable!(),
    };
    (direction, face, position)
}
pub fn adjust_pos(
    new_pos: &mut Complex<i32>,
    current_block: &mut (i8, i8),
    block_size: usize,
    num_tiles: &(usize, usize),
    map: &HashMap<(i8, i8), Vec<Vec<char>>>,
) -> Complex<i32> {
    if new_pos.re >= block_size as i32 {
        for _ in 0..num_tiles.0 {
            current_block.0 = (current_block.0 + 1).rem_euclid(num_tiles.0 as i8);
            if map.contains_key(&current_block) {
                new_pos.re = 0;
                break;
            }
        }
    } else if new_pos.re < 0 {
        for _ in 0..num_tiles.0 {
            current_block.0 = (current_block.0 as i8 - 1).rem_euclid(num_tiles.0 as i8);

            if map.contains_key(&current_block) {
                new_pos.re = block_size as i32 - 1;
                break;
            }
        }
    } else if new_pos.im >= block_size as i32 {
        for _ in 0..num_tiles.1 {
            current_block.1 = (current_block.1 + 1).rem_euclid(num_tiles.1 as i8);
            if map.contains_key(&current_block) {
                new_pos.im = 0;
                break;
            }
        }
    } else if new_pos.im < 0 {
        for _ in 0..num_tiles.1 {
            current_block.1 = (current_block.1 - 1).rem_euclid(num_tiles.1 as i8);
            if map.contains_key(&current_block) {
                new_pos.im = block_size as i32 - 1;
                break;
            }
        }
    }
    return *new_pos;
}
