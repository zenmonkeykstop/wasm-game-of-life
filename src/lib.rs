mod utils;

use wasm_bindgen::prelude::*;

use std::fmt;

extern crate js_sys;
extern crate fixedbitset;
use fixedbitset::FixedBitSet;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

#[wasm_bindgen]
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width -1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
     
    fn add_spaceship(&self, cells: &mut FixedBitSet, xoff: u32, yoff: u32) -> FixedBitSet {
    /* Ship starts as a 7x6 grid that looks like:
     * _______
     * __#__#_
     * _#_____
     * _#___#_
     * _####__
     * _______
     *
     */
        let mut next = cells.clone();

        let idx_start = self.get_index(xoff % self.width, yoff %self.height);
        next.set(idx_start,true);
        next.set(idx_start+1, true);
        next.set(idx_start+2, true);
        next

    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        self.add_spaceship(&mut next, 40, 40);

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                next.set(idx, match (cell, live_neighbors) {
                    (true, x) if x < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3 => false,
                    (false, 3) => true,
                    (otherwise, _) => otherwise
                });
            }
        }
        self.cells = next;
    }

    pub fn new() -> Universe {
        let width = 80;
        let height = 80;
        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, js_sys::Math::random() < 0.5);
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let i = self.get_index(row,col);
                let symbol = if self.cells[i] == false { ' ' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
