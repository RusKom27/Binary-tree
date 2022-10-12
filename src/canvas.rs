use draw::*;
use draw::shape::LinePoint;

pub trait CanvasHandle {
	fn draw_rect(&mut self, position: Point, width: u32, height: u32, weight: u32);
	fn draw_line(&mut self, start: Point, end: Point);
	fn save_svg(&self);
}

impl CanvasHandle for Canvas {
	fn draw_rect(&mut self, position: Point, width: u32, height: u32, weight: u32) {
		let rect = Drawing::new()
			.with_shape(Shape::Rectangle {
				width,
				height,
			})
			.with_xy(position.x, position.y)
			.with_style(Style::stroked(weight, Color::black()));

		self.display_list.add(rect);
	}

	fn draw_line(&mut self, start: Point, end: Point) {
		self.draw_rect(start, 1, 1, 2);
		let line = Drawing::new()
			.with_shape(Shape::Line {
				start,
				points: vec![LinePoint::Straight{point: end}]
			})
			.with_style(Style::stroked(1, Color::random()));
		self.display_list.add(line);
	}

	fn save_svg(&self) {
		render::save(
			self,
			"tests/svg/basic_end_to_end.svg",
			SvgRenderer::new(),
		)
		.expect("Failed to save");
	}

}
