#![allow(dead_code)]
#![allow(unused_variables)]

use nalgebra::*;
use rand::prelude::*;

struct Clock {
    corners: [i8; 4],
    f_edges: [i8; 4],
    f_centre: i8,
    b_edges: [i8; 4],
    b_centre: i8,
}

// 2n and 2n+1 are the rows to be added
// n = move - 1
const MATRIX_ROWS: [[f64; 14]; 28] = [
    [0., 1., 1., 0., 1., 1., 0., 0., 0., 0., 0., 0., 0., 0.],
    [1., 0., 0., 0., 0., 0., 1., 0., 1., -1., -1., -1., -1., -1.],
    [0., 0., 0., 0., 1., 1., 0., 1., 1., 0., 0., 0., 0., 0.],
    [1., 0., 1., 0., 0., 0., 1., 0., 0., -1., -1., -1., -1., -1.],
    [0., 1., 1., 0., 1., 1., 0., 1., 1., 0., 0., 0., 0., 0.],
    [1., 0., 0., 0., 0., 0., 1., 0., 0., -1., 0., -1., -1., -1.],
    [0., 0., 0., 1., 1., 0., 1., 1., 0., 0., 0., 0., 0., 0.],
    [1., 0., 1., 0., 0., 0., 0., 0., 1., -1., -1., -1., -1., -1.],
    [0., 1., 1., 1., 1., 1., 1., 1., 0., 0., 0., 0., 0., 0.],
    [1., 0., 0., 0., 0., 0., 0., 0., 1., -1., -1., -1., -1., -1.],
    [0., 0., 0., 1., 1., 1., 1., 1., 1., 0., 0., 0., 0., 0.],
    [1., 0., 1., 0., 0., 0., 0., 0., 0., -1., -1., -1., -1., 0.],
    [0., 1., 1., 1., 1., 1., 1., 1., 1., 0., 0., 0., 0., 0.],
    [1., 0., 0., 0., 0., 0., 0., 0., 0., -1., 0., -1., -1., 0.],
    [1., 1., 0., 1., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0.],
    [0., 0., 1., 0., 0., 0., 1., 0., 1., -1., -1., -1., -1., -1.],
    [1., 1., 1., 1., 1., 1., 0., 0., 0., 0., 0., 0., 0., 0.],
    [0., 0., 0., 0., 0., 0., 1., 0., 1., 0., -1., -1., -1., -1.],
    [1., 1., 0., 1., 1., 1., 0., 1., 1., 0., 0., 0., 0., 0.],
    [0., 0., 1., 0., 0., 0., 1., 0., 0., -1., -1., -1., -1., -1.],
    [1., 1., 1., 1., 1., 1., 0., 1., 1., 0., 0., 0., 0., 0.],
    [0., 0., 0., 0., 0., 0., 1., 0., 0., 0., 0., -1., -1., -1.],
    [1., 1., 0., 1., 1., 0., 1., 1., 0., 0., 0., 0., 0., 0.],
    [0., 0., 1., 0., 0., 0., 0., 0., 1., -1., -1., -1., 0., -1.],
    [1., 1., 1., 1., 1., 1., 1., 1., 0., 0., 0., 0., 0., 0.],
    [0., 0., 0., 0., 0., 0., 0., 0., 1., 0., -1., -1., 0., -1.],
    [1., 1., 0., 1., 1., 1., 1., 1., 1., 0., 0., 0., 0., 0.],
    [0., 0., 1., 0., 0., 0., 0., 0., 0., -1., -1., -1., 0., 0.],
];

const PINS: [&'static str; 16] = [
    "NONE", "UR", "DR", "R", "DL", "URDL", "D", "URDRDL", "UL", "U", "ULDR", "ULURDR", "L",
    "ULURDL", "ULDLDR", "ALL",
];

impl Clock {
    fn from_scramble(str: String) -> Self {
        todo!()
    }

    // Moves the up pins' dial by `up`, and the down pins' dial by `down`
    // pins is a bitmask somethonegi ur dr dl ul
    fn apply_move(&mut self, pins: i8, up: i8, down: i8) {
        // WARNING! UGLY CODE!!

        // corners
        todo!()
    }

    // probably bad (along with the apply move function) implementation of an optimal solver!
    // Returns a vector of (pin, up, down) values
    fn optimally_solve(&mut self) -> Vec<(i8, i8, i8)> {
        // Bitmask which reperesents which moves are made
        let min_length = 999;
        let cur_solution: Vec<(i8, i8, i8)> = vec![];
        for move_mask in 0..=0xFFFF {
            todo!()
        }
        cur_solution
    }

    // hi
    fn optimal_movecount(&self) -> usize {
        let scramble = self.as_matrix();
        let mut min_cnt = 1000;
        for pin_order in 0..=0xFFFF_u16 {
            if pin_order.count_ones() != 7 || (pin_order & 1 != 0) || (pin_order & 0x8000 != 0) {
                continue;
            }
            let mut vec = Vec::with_capacity(196);

            for i in 1..15 {
                if pin_order & (1 << i) != 0 {
                    let mut temp1 = MATRIX_ROWS[2 * (i - 1)].to_vec();
                    let mut temp2 = MATRIX_ROWS[2 * (i - 1) + 1].to_vec();

                    vec.append(&mut temp1);
                    vec.append(&mut temp2);
                }
            }
            let matrix = SMatrix::<f64, 14, 14>::from_column_slice(&vec);
            let det = matrix.determinant();
            if det != 0. {
                let inv = matrix.try_inverse().unwrap();
                let sol = -(inv * scramble);
                let mut mv_cnt = 7;
                for i in 0..7 {
                    if sol[i * 2] == 0. && sol[i * 2 + 1] == 0. {
                        mv_cnt -= 1;
                    }
                }
                let mut turn_cnt = 0;
                for i in 0..14 {
                    let mut n = sol[i] as i8;
                    n %= 12;
                    if n > 6 {
                        n -= 12;
                    }
                    turn_cnt += n.abs() as usize;
                }
                if mv_cnt < min_cnt {
                    min_cnt = mv_cnt;
                    //println!("{} got {}", pin_order, turn_cnt);
                }
            }
        }
        min_cnt
    }

    fn as_matrix(&self) -> SMatrix<f64, 14, 1> {
        let mut scramble = SMatrix::<f64, 14, 1>::from_row_slice(&[0.; 14]);

        scramble[0] = self.corners[0] as f64;
        scramble[2] = self.corners[1] as f64;
        scramble[6] = self.corners[2] as f64;
        scramble[8] = self.corners[3] as f64;

        scramble[1] = self.f_edges[0] as f64;
        scramble[3] = self.f_edges[1] as f64;
        scramble[5] = self.f_edges[2] as f64;
        scramble[8] = self.f_edges[3] as f64;

        scramble[4] = self.f_centre as f64;

        scramble[9] = self.b_edges[0] as f64;
        scramble[10] = self.b_edges[1] as f64;
        scramble[12] = self.b_edges[2] as f64;
        scramble[13] = self.b_edges[3] as f64;

        scramble[11] = self.b_centre as f64;

        scramble
    }

    fn random() -> Self {
        let mut rng = thread_rng();
        // epic code
        let a: i8 = rng.gen_range(-6..6);
        let b: i8 = rng.gen_range(-6..6);
        let c: i8 = rng.gen_range(-6..6);
        let d: i8 = rng.gen_range(-6..6);
        let e: i8 = rng.gen_range(-6..6);
        let f: i8 = rng.gen_range(-6..6);
        let g: i8 = rng.gen_range(-6..6);
        let h: i8 = rng.gen_range(-6..6);
        let i: i8 = rng.gen_range(-6..6);
        let j: i8 = rng.gen_range(-6..6);
        let k: i8 = rng.gen_range(-6..6);
        let l: i8 = rng.gen_range(-6..6);
        let m: i8 = rng.gen_range(-6..6);
        let n: i8 = rng.gen_range(-6..6);
        Clock {
            corners: [a, b, c, d],
            f_edges: [e, f, g, h],
            f_centre: i,
            b_edges: [j, k, l, m],
            b_centre: n,
        }
    }
}

fn find_pin_orders() {
    for pin_order in 0..=0xFFFF_u16 {
        if pin_order.count_ones() != 7 || (pin_order & 1 != 0) || (pin_order & 0x8000 != 0) {
            continue;
        }
        let mut vec = Vec::with_capacity(196);

        for i in 1..15 {
            if pin_order & (1 << i) != 0 {
                let mut temp1 = MATRIX_ROWS[2 * (i - 1)].to_vec();
                let mut temp2 = MATRIX_ROWS[2 * (i - 1) + 1].to_vec();

                vec.append(&mut temp1);
                vec.append(&mut temp2);
            }
        }
        let matrix = SMatrix::<f64, 14, 14>::from_column_slice(&vec);
        let det = matrix.determinant();
        if det != 0. && pin_order == 6040 {
            for i in 1..15 {
                if pin_order & (1 << i) != 0 {
                    print!("{} ", PINS[i]);
                }
            }
            println!(" id is {}", pin_order);
            let inv = matrix.try_inverse().unwrap();
            let scramble = SMatrix::<f64, 14, 1>::from_row_slice(&[
                -1., 1., 6., -3., 4., 0., 3., -5., 6., -1., -1., 3., 5., 0.,
            ]);
            let sol = -(inv * scramble);
            println!("{:?}", sol);
            break;
        }
    }
}

fn main() {
    find_pin_orders();
    // UR2- DR6+ DL0+ UL4- U0+ R3+ D5- L1+ ALL5- y2 U2+ R4+ D4+ L5- ALL6+ UL
    //let clock = Clock {
    //corners: [-5, -4, 6, -1],
    //f_edges: [-5, -5, 4, -3],
    //f_centre: -1,
    //b_edges: [5, -1, -3, 0],
    //b_centre: 6,
    //};
    //println!("{}", clock.optimal_movecount());

    let mut sum = 0;

    for _ in 0..10000 {
        let clock = Clock::random();
        sum += clock.optimal_movecount();
        //println!("{}", clock.optimal_movecount());
    }
    println!("{}", sum as f64 / 10000.);
}
