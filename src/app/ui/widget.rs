use druid::{
    Color,
    RenderContext,
    widget::Painter,
    kurbo::Line,
};

pub struct Border;

impl Border {
    pub fn new(position: BorderPosition, color: Color, width: f64) -> Painter<()> {
        Painter::new(move |ctx, _, env| {
            let line = Line::new((0.0, ctx.size().height - 0.5), (ctx.size().width, ctx.size().height - 0.5));

            ctx.stroke(line, &color, width);
        }) // CHANGE THIS!!! Line's use scale.
    }
}

pub enum BorderPosition {
    Top,
    Bottom
}

// Border::new(BorderPosition::Top, 1)