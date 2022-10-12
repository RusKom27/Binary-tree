use std::cmp::Ordering;
use std::fmt::Display;
use draw::{Canvas, Point};
use crate::{CanvasHandle, STEP_LENGTH, WIDTH};


#[derive(Debug, Clone)]
pub enum BTree<T: Display + Ord + Copy> {
	Leaf {
		value: T,
		position: Point,
		left: Box<BTree<T>>,
		right: Box<BTree<T>>,
	},
	Empty,
}

impl<T: Display + Ord + Copy> BTree<T> {
	pub fn new() -> BTree<T> {
		BTree::Empty
	}

	pub fn insert(&mut self, nv: T, level: usize, index: usize) {
		match self {
			&mut BTree::Leaf {
				ref value,
				position: _,
				ref mut left,
				ref mut right } => {
				match nv.cmp(value) {
					Ordering::Less => right.insert(nv, level + 1, index * 2 + 1),
					Ordering::Greater => left.insert(nv, level + 1, index * 2),
					_  => return
				}
			}
			&mut BTree::Empty => {
				let mut i = index;
				let mut l = level;
				if i % 2 == 0 {
					i += 1;
				}
				if l == 0 {
					l = 1;
				}
				let mut x = (WIDTH as f32 / 2_i32.pow(l as u32) as f32 * i as f32);
				let y = (level as i32 * STEP_LENGTH) as f32;
				*self = BTree::Leaf {
					value: nv,
					position: Point::new(x, y),
					left: Box::new(BTree::Empty),
					right: Box::new(BTree::Empty)
				}
			},
		};
	}

	pub fn draw(&mut self, canvas: &mut Canvas, parent_point: &Point) {
		match self {
			&mut BTree::Leaf {
				value: _,
				ref position,
				ref mut left,
				ref mut right } => {
				canvas.draw_line(parent_point.clone(), position.clone());
				left.draw(canvas, position);
				right.draw(canvas, position);
			}
			&mut BTree::Empty => {
				()
			},
		};
	}
}