use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::string::String;

use gmatrix::Gmatrix;
use gmatrix::make_scale;
use gmatrix::make_trans;
use gmatrix::make_rot_x;
use gmatrix::make_rot_y;
use gmatrix::make_rot_z;

use display::disp;
use display::clear_screen;
use display::save_ppm;

use draw::draw_tris;
use draw::draw_lines;
use draw::add_curve;
use draw::add_circle;
use draw::add_box;
use draw::add_sphere;
use draw::add_torus;

pub fn parse_file(name: &str) {
	let mut screen = [[[255; 3]; 500]; 500];
	//let mut transf = Gmatrix::new().identity();
	let mut stack: Vec<Gmatrix> = Vec::new();
	stack.push(Gmatrix::new().identity()); //first comes I
	let f = File::open(name).unwrap();
	let file = BufReader::new(&f);
	let mut last = String::from("");
	let mut l: String;
	for line in file.lines() {
		l = line.unwrap();
		let stack_lpos = stack.len()-1;

		let split = l.split(" ");
 		let vec: Vec<&str> = split.collect();
 		
		match last.trim() {
			"save" => {
				//draw_tris(edges, &mut screen, [255,255,255]);
				println!("Saving as {}..", vec[0]);
				save_ppm(&mut screen, vec[0]);
			 	//clear_screen(&mut screen);
			 	last = String::from("");
			 }
			"line" => {
				let mut edges = Gmatrix::new();
 				edges.add_edge(vec[0].parse().unwrap(), 
 					vec[1].parse().unwrap(), 
 					vec[2].parse().unwrap(),
 					vec[3].parse().unwrap(), 
 					vec[4].parse().unwrap(),
 					vec[5].parse().unwrap());
 				stack[stack_lpos].edit_mult(&mut edges);
 				draw_lines(&mut edges, &mut screen, [0,0,0]);
 				last = String::from("");
			}
			"scale" => {
				let scale = make_scale(
					vec[0].parse().unwrap(),
					vec[1].parse().unwrap(),
					vec[2].parse().unwrap()
					);
				//scale.edit_mult(transf);
				scale.edit_mult(&mut stack[stack_lpos]);
				last = String::from("");
			}
			"move" => {
				let trans = make_trans(
					vec[0].parse().unwrap(),
					vec[1].parse().unwrap(),
					vec[2].parse().unwrap()
				);
				//trans.edit_mult(transf);
				trans.edit_mult(&mut stack[stack_lpos]);
				last = String::from("");
			}
			"rotate" => {
				let mut rot = Gmatrix::new();
				match vec[0].trim() {
					"x" => rot = make_rot_x(vec[1].parse().unwrap()),
					"y" => rot = make_rot_y(vec[1].parse().unwrap()),
					"z" => rot = make_rot_z(vec[1].parse().unwrap()),
					_ => ()
				}
				//rot.edit_mult(transf);
				rot.edit_mult(&mut stack[stack_lpos]);
				last = String::from("");
			}
			"circle" => {
				let mut edges = Gmatrix::new();
				add_circle(&mut edges,
					vec[0].parse().unwrap(),
					vec[1].parse().unwrap(),
					vec[2].parse().unwrap(),
					vec[3].parse().unwrap());
				stack[stack_lpos].edit_mult(&mut edges);
				draw_lines(&mut edges, &mut screen, [0,0,0]);
				last = String::from("");
			}
			"hermite" => {
				let mut edges = Gmatrix::new();
				add_curve(&mut edges,
					vec[0].parse().unwrap(), //x0
					vec[1].parse().unwrap(), //y0
					vec[2].parse().unwrap(), //x1
					vec[3].parse().unwrap(), //y1
					vec[4].parse().unwrap(), //rx0
					vec[5].parse().unwrap(), //ry0
					vec[6].parse().unwrap(), //rx1
					vec[7].parse().unwrap(), //ry1
					"h");
				stack[stack_lpos].edit_mult(&mut edges);
				draw_lines(&mut edges, &mut screen, [0,0,0]);
				last = String::from("");
			}
			"bezier" => {
				let mut edges = Gmatrix::new();
				add_curve(&mut edges,
					vec[0].parse().unwrap(),
					vec[1].parse().unwrap(),
					vec[2].parse().unwrap(),
					vec[3].parse().unwrap(),
					vec[4].parse().unwrap(),
					vec[5].parse().unwrap(),
					vec[6].parse().unwrap(),
					vec[7].parse().unwrap(),
					"b");
				stack[stack_lpos].edit_mult(&mut edges);
				draw_lines(&mut edges, &mut screen, [0,0,0]);
				last = String::from("");
			}
			"box" => {
				let mut tris = Gmatrix::new();
				add_box(&mut tris,
					vec[0].parse().unwrap(), //x
					vec[1].parse().unwrap(), //y
					vec[2].parse().unwrap(), //z
					vec[3].parse().unwrap(), //width
					vec[4].parse().unwrap(), //height
					vec[5].parse().unwrap() //depth
					);
				stack[stack_lpos].edit_mult(&mut tris);
				draw_tris(&mut tris, &mut screen, [0,0,0]);
				last = String::from("");
			}
			"sphere" => {
				let mut tris = Gmatrix::new();
				add_sphere(&mut tris,
					vec[0].parse().unwrap(), //cx
					vec[1].parse().unwrap(), //cy
					vec[2].parse().unwrap(), //cz
					vec[3].parse().unwrap(),
					0.05
					); //r
				stack[stack_lpos].edit_mult(&mut tris);
				draw_tris(&mut tris, &mut screen, [0,0,0]);
				last = String::from("");
			}
			"torus" => {
				let mut tris = Gmatrix::new();
				add_torus(&mut tris,
					vec[0].parse().unwrap(), //cx
					vec[1].parse().unwrap(), //cy
					vec[2].parse().unwrap(), //cz
					vec[3].parse().unwrap(), //r1
					vec[4].parse().unwrap(), //r2
					0.05
					);
				stack[stack_lpos].edit_mult(&mut tris);
				draw_tris(&mut tris, &mut screen, [0,0,0]);
				last = String::from("");
			}
 			_ => {
				match l.trim() {
				"push" => {
					let mut to_copy = Gmatrix::new().identity();
					stack[stack_lpos].copy_into(&mut to_copy);
					stack.push(to_copy);
				}
				"pop" => {
					stack.pop();
				}
				"ident" => {
					/*
					let g = Gmatrix::new().identity();
					for i in 0..g.rlen() {
						for c in 0..g.clen() {
							transf.set_val(i,c,g.get_val(i,c));
						}
					}*/
					1+1;
				}
				"apply" => {
					//transf.edit_mult(edges);
					1+1;
				}
				"display" => {
					// draw_lines(edges, screen, [255,50,50]);
					//draw_tris(edges, &mut screen, [255,50,50]);
					disp(&mut screen);
					//clear_screen(&mut screen);
				}
				"clear" => {
					//edges.clear();
					clear_screen(&mut screen);
				}
				_ => last = String::from(vec[0]),
				}
			}
		}
	}
}