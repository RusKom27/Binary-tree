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
    let mut tree = BTree::<i128>::new();
    for i in 0..100 {
        tree.insert(random::<i128>() % 10000, 0, 0);
    }
	let window = WindowDesc::new(move || tree).title(
        LocalizedString::new("Binary tree"));
    AppLauncher::with_window(window)
        .launch(AppData::default())
        .expect("launch failed");
}