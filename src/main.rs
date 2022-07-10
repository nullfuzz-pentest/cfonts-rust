extern crate cfonts;
// By Nullfuzz 
use cfonts::{ say, Options, Align, BgColors, Colors, Env, Fonts, Rgb };

fn main() {
	say(Options {
		text: String::from("Aiko"),
		font: Fonts::Font3d,
		colors: vec![Colors::Red],
		background: BgColors::Black,
		align: Align::Center,
		letter_spacing: 1,
		line_height: 1,
		spaceless: false,
		max_length: 0,
		gradient: Vec::new(),
		independent_gradient: false,
		transition_gradient: false,
		env: Env::Cli,
		..Options::default()
	});
}