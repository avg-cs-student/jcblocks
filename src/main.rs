pub mod blast;

use blast::rectangle::LONGEST_RECTANGLE_EDGE;
use blast::rectangle::Rectangle;
use blast::tee::Tee;

use crate::blast::canvas::Canvas;

fn main() {
    println!("Hello, world!");

    let canvas = Canvas::new(8, 8);
    println!("{canvas:?}");

    let completed_rows = canvas.check_rows();
    println!("Completed rows:\t{completed_rows:?}");

    let completed_columns = canvas.check_columns();
    println!("Completed columns:\t{completed_columns:?}");

    for width in 1..=LONGEST_RECTANGLE_EDGE {
        for height in 1..=LONGEST_RECTANGLE_EDGE {
            let rectangle = Rectangle::new(height, width);
            println!("Rectangle [{height}x{width}]: {rectangle:?}");
        }
    }

    let tee = Tee::new();
    println!("Tee: {tee:?}");
}
