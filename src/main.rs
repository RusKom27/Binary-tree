#![windows_subsystem = "windows"]

mod gui_app;
mod binary_tree;

use druid::{AppLauncher, LocalizedString, Point, WindowDesc};
use rand::random;

use crate::binary_tree::BTree;
use crate::gui_app::AppData;

pub const STEP_LENGTH:i32 = 50;
pub const WIDTH:f64 = 600.;
pub const HEIGHT:f64 = 600.;

#[tokio::main]
async fn main() {
    let mut tree = BTree::<i32>::new();
    for _ in 0..10000 {
        tree.insert(random::<i32>() % 500, 0, 0);
    }
	let window = WindowDesc::new(|| tree).title(
        LocalizedString::new("Binary tree")
    ).with_min_size((20.,30.));
    AppLauncher::with_window(window)
        .launch(AppData {
            mouse_pressed: false,
            start_move_point: Point::ZERO,
        })
        .expect("launch failed");
}