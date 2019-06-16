//! Graphics primitives

use crate::drawable::Dimensions;

pub mod circle;
pub mod line;
pub mod rect;
pub mod triangle;

/// Primitive trait
pub trait Primitive: Dimensions {}

pub use self::circle::Circle;
pub use self::line::Line;
pub use self::rect::Rect;
pub use self::triangle::Triangle;

/// Create a [`Circle`](./primitives/circle/struct.Circle.html) with optional styling using a
/// convenient macro.
///
/// ```rust
/// use embedded_graphics::{circle, style::Style, primitives::Circle};
///
/// let line_circle: Circle<u8> = circle!((10, 20), 30);
/// let filled_circle: Circle<u8> = circle!((10, 20), 30, stroke = Some(5u8), fill = Some(10u8));
/// let default_style: Circle<u8> = circle!((10, 20), 30, style = Style::default());
/// ```
///
/// Style properties like `stroke` map to the method calls on the
/// [`WithStyle`](style/trait.WithStyle.html) trait. For example, the following code makes two
/// identical circles:
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::{circle, style::Style, primitives::Circle};
///
/// let circle: Circle<u8> = circle!((10, 20), 30, stroke = Some(5u8), fill = Some(10u8));
/// let circle: Circle<u8> = Circle::new(Coord::new(10, 20), 30).stroke(Some(5u8)).fill(Some(10u8));
/// ```
#[macro_export]
macro_rules! circle {
    (($cx:expr, $cy:expr), $r:expr $(, $style_key:ident = $style_value:expr )* $(,)?) => {{
        #[allow(unused_imports)]
        use $crate::style::WithStyle;
        $crate::primitives::Circle::new($crate::coord::Coord::new($cx, $cy), $r)
            $( .$style_key($style_value) )*
    }};
}

/// Create a [`Line`](./primitives/line/struct.Line.html) with optional styling using a
/// convenient macro.
///
/// Note that only the `stroke` property has any effect on lines currently.
///
/// ```rust
/// use embedded_graphics::{line, style::Style, primitives::Line};
///
/// let line: Line<u8> = line!((10, 20), (30, 40));
/// let stroke_line: Line<u8> = line!((10, 20), (30, 40), stroke = Some(5u8));
/// ```
///
/// Style properties like `stroke` map to the method calls on the
/// [`WithStyle`](style/trait.WithStyle.html) trait. For example, the following code makes two
/// identical lines:
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::{line, style::Style, primitives::Line};
///
/// let Line: Line<u8> = line!((10, 20), (30, 40), stroke = Some(5u8), fill = Some(10u8));
/// let Line: Line<u8> = Line::new(Coord::new(10, 20), Coord::new(30, 40))
///     .stroke(Some(5u8))
///     .fill(Some(10u8));
/// ```
#[macro_export]
macro_rules! line {
    (($x1:expr, $y1:expr), ($x2:expr, $y2:expr) $(, $style_key:ident = $style_value:expr )* $(,)?) => {{
        #[allow(unused_imports)]
        use $crate::style::WithStyle;
        $crate::primitives::Line::new($crate::coord::Coord::new($x1, $y1), $crate::coord::Coord::new($x2, $y2))
            $( .$style_key($style_value) )*
    }};
}

/// Create a [`Rect`](./primitives/rect/struct.Rect.html) with optional styling using a
/// convenient macro.
///
/// ```rust
/// use embedded_graphics::{rect, style::Style, primitives::Rect};
///
/// let empty_rect: Rect<u8> = rect!((10, 20), (30, 40));
/// let filled_rect: Rect<u8> = rect!((10, 20), (30, 40), stroke = Some(5u8), fill = Some(10u8));
/// let rect_default_style: Rect<u8> = rect!((10, 20), (30, 40), style = Style::default());
/// ```
///
/// Style properties like `stroke` map to the method calls on the
/// [`WithStyle`](style/trait.WithStyle.html) trait. For example, the following code makes two
/// identical rectangles:
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::{rect, style::Style, primitives::Rect};
///
/// let Rect: Rect<u8> = rect!((10, 20), (30, 40), stroke = Some(5u8), fill = Some(10u8));
/// let Rect: Rect<u8> = Rect::new(Coord::new(10, 20), Coord::new(30, 40))
///     .stroke(Some(5u8))
///     .fill(Some(10u8));
/// ```
#[macro_export]
macro_rules! rect {
    (($x1:expr, $y1:expr), ($x2:expr, $y2:expr) $(, $style_key:ident = $style_value:expr )* $(,)?) => {{
        #[allow(unused_imports)]
        use $crate::style::WithStyle;
        $crate::primitives::Rect::new($crate::coord::Coord::new($x1, $y1), $crate::coord::Coord::new($x2, $y2))
            $( .$style_key($style_value) )*
    }};
}

/// Create a [`Triangle`](./primitives/triangle/struct.Triangle.html) with optional styling using a
/// convenient macro.
///
/// ```rust
/// use embedded_graphics::{triangle, style::Style, primitives::Triangle};
///
/// let empty_triangle: Triangle<u8> = triangle!((10, 20), (30, 40), (50, 60));
/// let filled_triangle: Triangle<u8> = triangle!((10, 20), (30, 40), (50, 60), stroke = Some(5u8), fill = Some(10u8));
/// let triangle_default_style: Triangle<u8> = triangle!((10, 20), (30, 40), (50, 60), style = Style::default());
/// ```
///
/// Style properties like `stroke` map to the method calls on the
/// [`WithStyle`](style/trait.WithStyle.html) trait. For example, the following code makes two
/// identical triangles:
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::{triangle, style::Style, primitives::Triangle};
///
/// let Triangle: Triangle<u8> = triangle!((10, 20), (30, 40), (50, 60), stroke = Some(5u8), fill = Some(10u8));
/// let Triangle: Triangle<u8> = Triangle::new(Coord::new(10, 20), Coord::new(30, 40), Coord::new(50, 60))
///     .stroke(Some(5u8))
///     .fill(Some(10u8));
/// ```
#[macro_export]
macro_rules! triangle {
    (($x1:expr, $y1:expr), ($x2:expr, $y2:expr), ($x3:expr, $y3:expr) $(, $style_key:ident = $style_value:expr )* $(,)?) => {{
        #[allow(unused_imports)]
        use $crate::style::WithStyle;
        $crate::primitives::Triangle::new($crate::coord::Coord::new($x1, $y1), $crate::coord::Coord::new($x2, $y2), $crate::coord::Coord::new($x3, $y3))
            $( .$style_key($style_value) )*
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Style;

    #[test]
    fn circle() {
        let _c: Circle<u8> = circle!((10, 20), 30);
        let _c: Circle<u8> = circle!((10, 20), 30, stroke = Some(1u8), fill = Some(10u8));
        let _c: Circle<u8> = circle!((10, 20), 30, style = Style::default());
    }

    #[test]
    fn line() {
        let _l: Line<u8> = line!((10, 20), (30, 40));
        let _l: Line<u8> = line!((10, 20), (30, 40), stroke = Some(1u8), fill = Some(10u8));
        let _l: Line<u8> = line!((10, 20), (30, 40), style = Style::default());
    }

    #[test]
    fn rect() {
        let _r: Rect<u8> = rect!((10, 20), (30, 40));
        let _r: Rect<u8> = rect!((10, 20), (30, 40), stroke = Some(1u8), fill = Some(10u8));
        let _r: Rect<u8> = rect!((10, 20), (30, 40), style = Style::default());
    }

    #[test]
    fn triangle() {
        let _t: Triangle<u8> = triangle!((10, 20), (30, 40), (50, 60));
        let _t: Triangle<u8> = triangle!(
            (10, 20),
            (30, 40),
            (50, 60),
            stroke = Some(1u8),
            fill = Some(10u8)
        );
        let _t: Triangle<u8> = triangle!((10, 20), (30, 40), (50, 60), style = Style::default());
    }
}
