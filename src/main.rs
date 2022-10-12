mod canvas;
mod binary_tree;

use draw::{Canvas, Point};
use rand::random;

use crate::binary_tree::BTree;
use crate::canvas::CanvasHandle;

pub const STEP_LENGTH:i32 = 50;
pub const WIDTH:u32 = 600;
pub const HEIGHT:u32 = 600;


fn main() {
    let mut tree = BTree::<usize>::new();
    for _ in 0..100 {
		tree.insert(random::<usize>(), 0, 0);
    }
	let mut canvas = Canvas::new(WIDTH, HEIGHT);

	canvas.draw_rect(Point::new(0.,0.), WIDTH, HEIGHT, 1);
	tree.draw(&mut canvas, &Point::new((WIDTH / 2) as f32, 0.));
	canvas.save_svg();
}