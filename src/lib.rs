mod utils;

use wasm_bindgen::prelude::*;

use std::fmt;

extern crate js_sys;
extern crate fixedbitset;
use fixedbitset::FixedBitSet;

extern crate web_sys;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

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

impl Universe {
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    pub fn set_cells(&mut self, cells:&[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx =self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }
}


#[wasm_bindgen]
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        let size = (self.width * self.height) as usize;
        self.cells = FixedBitSet::with_capacity(size);
    }


    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        let size = (self.width * self.height) as usize;
        self.cells = FixedBitSet::with_capacity(size);
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
     
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

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

    pub fn new(is_random:bool) -> Universe {
        utils::set_panic_hook();
        let width = 64;
        let height = 64;
        log!("The universe is {} by {} cells...", width, height);
        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        if is_random {
            for i in 0..size {
                cells.set(i, js_sys::Math::random() < 0.5);
            }
        } else {
            // add a lonely spaceship and a glider
            let x_off = 40;
            let y_off = 40;
            let ship_defn  = [(0,1), (0,4), (1,0), (2,0), (2,4), (3,0), (3,1), (3,2), (3,3), (11,1), (12,2), (13,0), (13,1), (13,2), (31,1), (32,2), (33,0), (33,1), (33,2)];
            for (row, col) in ship_defn.iter().cloned() {
                let idx = ((row + x_off) * width + (col + y_off)) as usize;
                cells.set(idx % size, true);

            }
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
