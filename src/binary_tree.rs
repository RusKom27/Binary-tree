use std::cmp::{Ordering};
use std::fmt::Display;
use druid::{Point};


#[derive(Debug, Clone)]
pub enum BTree<T: Display + Ord + Copy + Send> {
	Leaf {
		value: T,
		position: Point,
		level: usize,
		left: Box<BTree<T>>,
		right: Box<BTree<T>>,
	},
	Empty,
}

impl<T: Display + Ord + Copy + Send> BTree<T> {
	pub fn new() -> BTree<T> {
		BTree::Empty
	}

	pub fn insert(&mut self, new_value: T, level: usize, index: usize) {
		match self {
			&mut BTree::Leaf {
				ref value,
				position: _,
				level: _,
				ref mut left,
				ref mut right } => {
				match new_value.cmp(value) {
					Ordering::Less => right.insert(new_value, level + 1, index * 2 + 2),
					Ordering::Greater => left.insert(new_value, level + 1, index * 2),
					_  => return
				}
			}
			&mut BTree::Empty => {
				*self = BTree::Leaf {
					value: new_value,
					position: self.calculate_position(index, level),
					level,
					left: Box::new(BTree::Empty),
					right: Box::new(BTree::Empty)
				}
			},
		};
	}
}