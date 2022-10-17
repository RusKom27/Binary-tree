use std::fmt::Display;
use druid::widget::prelude::*;
use druid::{Color, Env, FontDescriptor, FontFamily, PaintCtx, Point, RenderContext, TextLayout};
use druid::kurbo::Line;
use async_recursion::async_recursion;
use tokio::runtime::Handle;
use tokio::task::block_in_place;

use crate::{BTree, HEIGHT, STEP_LENGTH, WIDTH};

const FONT_DESC: FontDescriptor = FontDescriptor::new(FontFamily::SERIF).with_size(21.);

impl<T: Display + Ord + Copy + Send> BTree<T> {
    pub fn draw(&mut self, ctx: &mut PaintCtx, env: &Env, layout: &mut TextLayout<String>, parent_point: Option<&Point>) {
        match self {
            &mut BTree::Leaf {
                ref value,
                ref position,
                ref level,
                ref mut left,
                ref mut right } => {
                if position.x < ctx.window().get_size().width + 200. &&
                    position.y < ctx.window().get_size().height + 200. &&
                    position.x > -200. &&
                    position.y > -200.{
                    layout.set_text(format!("{}", value));
                    layout.set_font(FONT_DESC.clone().with_size((FONT_DESC.size - *level as f64).clamp(11., 24.)));
                    layout.rebuild_if_needed(ctx.text(), env);
                    ctx.with_save(|ctx| {
                        layout.draw(ctx, (position.x - 20. + *level as f64, position.y));
                    });

                    match parent_point {
                        Some(parent_point) => {
                            ctx.stroke(Line::new(parent_point.clone(), position.clone()), &Color::WHITE, 2.);
                        },
                        None => ()
                    }
                }

                left.draw(ctx, env, layout, Some(&Point::new(position.x, position.y + 20.)));
                right.draw(ctx, env,  layout, Some(&Point::new(position.x, position.y + 20.)));
            }
            &mut BTree::Empty => {
                ()
            },
        };
    }

    pub fn calculate_position(&mut self, index: u128, level: u128) -> Point {
        let mut i = index;
        let l = level;
        if (i + 2) % 2 == 0 {
            i += 1;
        }
        let x = WIDTH / 2_i32.pow(l as u32) as f64 * i as f64;
        let y = (l as i32 * STEP_LENGTH) as f64;
        return Point::new(x, y);
    }
    #[async_recursion]
    pub async fn zoom(&mut self, dir: f64, origin: Point) {
        match self {
            &mut BTree::Leaf {
                value: _,
                ref mut position,
                level: _,
                ref mut left,
                ref mut right } => {
                let diff = *position - origin;
                position.x -= diff.x / 4. * dir;
                position.y -= diff.y / 4. * dir;
                left.zoom(dir, origin).await;
                right.zoom(dir, origin).await;
            },
            &mut BTree::Empty => {
                ()
            }
        }
    }
    #[async_recursion]
    pub async fn move_tree(&mut self, dir: Point) {
        match self {
            &mut BTree::Leaf {
                value: _,
                ref mut position,
                level: _,
                ref mut left,
                ref mut right } => {
                position.x -= dir.x;
                position.y -= dir.y;
                left.move_tree(dir).await;
                right.move_tree(dir).await;
            },
            &mut BTree::Empty => {
                ()
            }
        }
    }
}

#[derive(Clone, Data, Default)]
pub struct AppData {
    pub start_move_point: Option<Point>,
}

impl<T: Display + Ord + Copy + Send> Widget<AppData> for BTree<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppData, _env: &Env) {
        match event {
            Event::Wheel(event) => {
                block_in_place(move || {
                    Handle::current().block_on(async move {
                        self.zoom(f64::signum(event.wheel_delta.y), event.pos).await;
                    })
                });
                ctx.request_paint();
            },
            Event::MouseDown(event) => {
                data.start_move_point = Some(event.pos);
            },
            Event::MouseUp(_event) => {
                data.start_move_point = None;
            },
            Event::MouseMove(event) => {
                if data.start_move_point.is_some() {
                    let start_point = data.start_move_point.unwrap();
                    block_in_place(move || {
                        Handle::current().block_on(async move {
                            self.move_tree((start_point - event.pos).to_point()).await;
                        });

                    });
                    data.start_move_point = Some(event.pos);
                    ctx.request_paint();
                }
            },
            _ => {}
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &AppData, _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppData, _data: &AppData, _env: &Env) {}

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppData,
        _env: &Env,
    ) -> Size {
        bc.constrain((WIDTH, HEIGHT))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &AppData, env: &Env) {
        let mut layout = TextLayout::<String>::from_text("");
        layout.set_font(FontDescriptor::new(FontFamily::SERIF).with_size(24.));
        layout.set_text_color(Color::WHITE);
        layout.rebuild_if_needed(ctx.text(), env);
        self.draw(ctx, env, &mut layout, None)
    }
}
