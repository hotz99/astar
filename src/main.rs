mod field;

use field::*;
use nannou::prelude::*;

fn main() {
    nannou::app(Field::new).update(Field::update).run();
}