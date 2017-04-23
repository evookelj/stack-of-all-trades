mod gmatrix;
mod display;
mod draw;
mod parser;

use gmatrix::Gmatrix;
use parser::parse_file;

fn main() {
	parse_file("script");
}