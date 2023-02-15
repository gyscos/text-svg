use rusttype::{Font, Point, Rect, Scale};
use svg::node::element::Path;

pub struct Text {
    pub path: Path,
    pub bounding_box: Rect<f32>,
}

impl Text {
    pub fn new(path: Path, bounding_box: Rect<f32>) -> Self {
        Self { path, bounding_box }
    }

    pub fn builder() -> Builder<'static> {
        Default::default()
    }
}

pub struct Builder<'a> {
    pub fill: &'a str,
    pub size: f32,
    pub start: Point<f32>,
    pub letter_spacing: f32,
}

impl Default for Builder<'static> {
    fn default() -> Self {
        Self {
            fill: "#000",
            size: 32.,
            start: Point { x: 0., y: 0. },
            letter_spacing: 1.,
        }
    }
}

impl Builder<'_> {
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn start(mut self, start: Point<f32>) -> Self {
        self.start = start;
        self
    }

    pub fn build(&self, font: &Font, text: &str) -> Text {
        let mut d = String::new();

        let scale = Scale::uniform(self.size);
        let v_metrics = font.v_metrics(scale);

        let mut bounding_box = Rect::default();

        for glyph in font.layout(
            text,
            scale,
            Point {
                x: self.start.x,
                y: self.start.y + v_metrics.ascent,
            },
        ) {
            // `build_outline` uses coordinates relative to its pixel bounding box.
            // So offset the actual drawing by this bounding box top/left corner.
            let bb = glyph.pixel_bounding_box().unwrap();
            glyph.build_outline(&mut crate::Builder {
                x: bb.min.x as f32,
                y: bb.min.y as f32,
                d: &mut d,
            });

            bounding_box.min.x = f32::min(bounding_box.min.x, bb.min.x as f32);
            bounding_box.min.y = f32::min(bounding_box.min.y, bb.min.y as f32);
            bounding_box.max.x = f32::max(bounding_box.max.x, bb.max.x as f32);
            bounding_box.max.y = f32::max(bounding_box.max.y, bb.max.y as f32);
        }

        Text::new(Path::new().set("d", d).set("fill", "#000"), bounding_box)
    }
}
