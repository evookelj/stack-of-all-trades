use gmatrix::Gmatrix;
use gmatrix::get_bezier;
use gmatrix::get_hermite;

use display::plot;

use std::f32::consts::PI;

fn line1(x0: i32, y0: i32, x1: i32, y1: i32, screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3]) {
	let mut x = x0;
	let mut y = y0;
	if x0>x1 { return line1(x1,y1,x0,y0,screen,color); }
	let a = 2*(y1-y0) as i32;
	let b = -2*(x1-x0) as i32;
	let mut d: i32 = 2*a+b;
	while x < x1 {
		plot(x,y, screen, color);
		if d>0 {
			y += 1;
			d += b;
		}
		x += 1;
		d += a;
	}
}

fn line2(x0: i32, y0: i32, x1: i32, y1: i32, screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3]) {
	let mut x = x0;
	let mut y = y0;
	if x0>x1 { return line2(x1,y1,x0,y0,screen,color); }
	let a = 2*(y1-y0) as i32;
	let b = -2*(x1-x0) as i32;
	let mut d: i32 = 2*b+a;
	while y < y1 {
		plot(x,y, screen,color);
		if d<0 {
			x += 1;
			d += a;
		}
		y += 1;
		d += b;
	}
}

fn line7(x0: i32, y0: i32, x1: i32, y1: i32, screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3]) {
	let mut x = x0;
	let mut y = y0;
	if x0>x1 { return line2(x1,y1,x0,y0,screen,color); }
	let a = 2*(y1-y0) as i32;
	let b = -2*(x1-x0) as i32;
	let mut d: i32 = a-(2*b);
	while y > y1 {
		plot(x,y, screen,color);
		if d>0 { //bc deltay = A = negative
			x += 1;
			d += a;
		}
		y -= 1;
		d -= b;
	}
}


fn line8(x0: i32, y0: i32, x1: i32, y1: i32, screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3]) {
	let mut x = x0 as i32;
	let mut y = y0 as i32;
	if x0>x1 { return line8(x1,y1,x0,y0,screen,color); }
	let a = 2*(y1-y0) as i32;
	let b = -2*(x1-x0) as i32;
	let mut d: i32 = 2*a-b;
	while x < x1 {
		plot(x,y,screen,color);
		if d<0 {
			y -= 1;
			d -= b;
		}
		x += 1;
		d += a;
	}
}

fn draw_line(x0: i32, y0: i32, x1: i32, y1: i32, screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3]) {
	let dx: f64 = (x1 as f64)-(x0 as f64) as f64;
	let dy: f64 = (y1 as f64)-(y0 as f64) as f64;
	if dx<0.0 { draw_line(x1,y1,x0,y0,screen,color); }

	let m = dy/dx;

	if (dy==0.0) && (dx==0.0) { return ; }
	if (m >= 0.0) && (m < 1.0) { line1(x0,y0,x1,y1,screen,color); } 
	else if m>=1.0 { line2(x0,y0,x1,y1,screen,color); } 
	else if (m <= 0.0) && (m > -1.0) { line8(x0,y0,x1,y1,screen,color); } 
	else if m<=-1.0 { line7(x0,y0,x1,y1,screen,color); } 
	else { println!("Should never reach this"); }
}

#[allow(dead_code)]
pub fn draw_lines(gm: &mut Gmatrix, screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3]) {
	let mut i = 0;
	if gm.clen()<1 { return; }

	while i<gm.clen()-1 {
		draw_line(
			gm.get_val(0,i) as i32, //x0 
			gm.get_val(1,i) as i32, 
			gm.get_val(0,i+1) as i32, //y0 
			gm.get_val(1,i+1) as i32,
			screen,
			color);
		i += 2;
	}
}

fn draw_tri(x0:i32,y0:i32,z0:i32,x1:i32,y1:i32,z1:i32,x2:i32,y2:i32,z2:i32,screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3]) {
	let a = [x1-x0, y1-y0, z1-z0];
	let b = [x2-x0, y2-y0, z2-z0];
	let n = [(a[1]*b[2])-(a[2]*b[1]),
			(a[2]*b[0])-(a[0]*b[2]),
			(a[0]*b[1])-(a[1]*b[0])];

	if n[2]>0 {
		draw_line(x0,y0,x1,y1,screen,color);
		draw_line(x0,y0,x2,y2,screen,color);
		draw_line(x1,y1,x2,y2,screen,color);
	}
}

pub fn draw_tris(gm: &mut Gmatrix, screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3]) {
	let mut i=0;
	if gm.clen()<1 { return; }

	while i<gm.clen()-2 {
		draw_tri(
			gm.get_val(0,i) as i32,
			gm.get_val(1,i) as i32,
			gm.get_val(2,i) as i32,
			gm.get_val(0,i+1) as i32,
			gm.get_val(1,i+1) as i32,
			gm.get_val(2,i+1) as i32,
			gm.get_val(0,i+2) as i32,
			gm.get_val(1,i+2) as i32,
			gm.get_val(2,i+2) as i32,
			screen,
			color);
		i+=3;
	}
}

fn circle_x(t: f32, cx: f32, r: f32) -> f32 {
	let d = t*360.0;
	return cx+r*d.to_radians().cos()
}

fn circle_y(t: f32, cy: f32, r: f32) -> f32 {
	let d = t*360.0;
	return cy+r*d.to_radians().sin()
}

fn curve_x(t: f32, cx: &Gmatrix) -> f32 {
	let a = cx.get_val(0,0);
	let b = cx.get_val(1,0);
	let c = cx.get_val(2,0);
	let d = cx.get_val(3,0);
	return a*t*t*t+b*t*t+c*t+d;
}

fn curve_y(t: f32, cy: &Gmatrix) -> f32 {
	return curve_x(t, cy);
}

fn paramet_circ(edges: &mut Gmatrix, fx: &Fn(f32,f32,f32) -> f32, fy: &Fn(f32,f32,f32) -> f32, circ: [f32; 4], step: f32) {
	let mut t = 0.0;
	let mut x0 = -1.0;
	let mut y0 = -1.0;
	while t <= 1.001 {
		let x1 = fx(t, circ[0], circ[3]);
		let y1 = fy(t, circ[1], circ[3]);
		if t>0.00 {
			edges.add_edge(x0 as i32,y0 as i32,0,x1 as i32,y1 as i32,0);
		}
		x0 = x1;
		y0 = y1;
		t += step;
	}
}

fn paramet_curve(edges: &mut Gmatrix, cx: &Gmatrix, cy: &Gmatrix, fx: &Fn(f32,&Gmatrix) -> f32, fy: &Fn(f32,&Gmatrix) -> f32, step: f32) {
	let mut t = 0.0;
	let mut x0 = -1.0;
	let mut y0 = -1.0;
	while t <= 1.001 {
		let x1 = fx(t,cx);
		let y1 = fy(t,cy);
		if t>0.0 { edges.add_edge(x0 as i32, y0 as i32, 0, x1 as i32, y1 as i32, 0); }
		x0 = x1;
		y0 = y1;
		t += step;
	}
}

pub fn add_curve(edges: &mut Gmatrix, x0:f32,y0:f32,x1:f32,y1:f32,a5:f32,a6:f32,a7:f32,a8:f32,tp:&str) {
	let mut givx = Gmatrix::new();
	let mut givy = Gmatrix::new();
	let cx;
	let cy;

	givx.add_val(0,x0);
	givx.add_val(1,x1);
	givx.add_val(2,a5);
	givx.add_val(3,a7);

	givy.add_val(0,y0);
	givy.add_val(1,y1);
	givy.add_val(2,a6);
	givy.add_val(3,a8);

	if tp=="h" {
		cx = get_hermite(&givx);
		cy = get_hermite(&givy);
	} else {
		cx = get_bezier(&givx);
		cy = get_bezier(&givy);
	}
	paramet_curve(edges, &cx, &cy, &curve_x, &curve_y, 0.01);
}

pub fn add_circle(edges: &mut Gmatrix, cx: f32, cy: f32, cz: f32, r: f32) {
	paramet_circ(edges, &circle_x, &circle_y, [cx,cy,cz,r], 0.01);
}

pub fn add_box(edges: &mut Gmatrix, x:i32, y:i32, z:i32, w:i32, h:i32, d:i32) {
	//FRONT FACE
	edges.add_tri(x,y,z, x,y-h,z, x+w,y-h,z);
	edges.add_tri(x+w,y-h,z, x+w,y,z, x,y,z);
	//BACK FACE
	edges.add_tri(x,y,z-d, x,y-h,z-d, x+w,y-h,z-d);
	edges.add_tri(x+w,y-h,z-d, x+w,y,z-d, x,y,z-d);
	//LEFT FACE
	edges.add_tri(x,y,z-d, x,y-h,z-d, x,y-h,z);
	edges.add_tri(x,y-h,z, x,y,z, x,y,z-d);
	//RIGHT FACE
	edges.add_tri(x+w,y,z, x+w,y-h,z, x+w,y-h,z-d);
	edges.add_tri(x+w,y-h,z-d, x+w,y,z-d, x+w,y,z);
	//TOP FACE
	edges.add_tri(x,y,z-d, x,y,z, x+w,y,z);
	edges.add_tri(x+w,y,z, x+w,y,z-d, x,y,z-d);
	//BOTTOM FACE
	edges.add_tri(x,y-h,z-d, x,y-h,z, x+w,y-h,z);
	edges.add_tri(x+w,y-h,z, x+w,y-h,z-d, x,y-h,z-d);
}

pub fn add_sphere(edges: &mut Gmatrix, cx: f32, cy: f32, cz: f32, r: f32, step:f32) {
	let mut circ = Gmatrix::new();
	let mut n = (1.0/step+0.1) as usize;
	generate_sphere(&mut circ, cx, cy, cz, r, step);

	let lat_start = 0;
	let lat_stop = n;
	let long_start = 0;
	let long_stop = n;

	let mut i;
	n+=1;

	for lat in lat_start..lat_stop {
		for long in long_start..long_stop {
			i = lat*n+long;
			
			edges.add_tri(
				circ.get_val(0,i) as i32,
				circ.get_val(1,i) as i32,
				circ.get_val(2,i) as i32,

				circ.get_val(0,i+1) as i32,
				circ.get_val(1,i+1) as i32,
				circ.get_val(2,i+1) as i32,

				circ.get_val(0,i+1+n) as i32,
				circ.get_val(1,i+1+n) as i32,
				circ.get_val(2,i+1+n) as i32
				);
			edges.add_tri(
				circ.get_val(0,i) as i32,
				circ.get_val(1,i) as i32,
				circ.get_val(2,i) as i32,

				circ.get_val(0,i+1+n) as i32,
				circ.get_val(1,i+1+n) as i32,
				circ.get_val(2,i+1+n) as i32,

				circ.get_val(0,i+n) as i32,
				circ.get_val(1,i+n) as i32,
				circ.get_val(2,i+n) as i32
				);
		}
	}

}

fn generate_sphere(edges: &mut Gmatrix, cx: f32, cy: f32, cz: f32, r: f32, step: f32) {
	let mut rot = 0.0;
	let mut mrot;
	let mut big_ctr = 0;
	let num_steps = (1.0/step) as i32;
	while big_ctr <= num_steps {
		let mut circ = 0.0;
		let mut mcirc;
		let mut sm_ctr = 0;
		while sm_ctr <= num_steps {
			mrot = rot*2.0*PI;
			mcirc = circ*PI;
			let x = (r * mcirc.cos()+ cx) as i32;
			let y = (r * mcirc.sin() * mrot.cos() + cy) as i32;
			let z = (r * mcirc.sin() * mrot.sin() + cz) as i32;
			edges.add_pt(x,y,z);
			circ += step;
			sm_ctr += 1;
		}
		rot += step;
		big_ctr += 1;
	}
}

pub fn add_torus(edges: &mut Gmatrix, cx:f32, cy:f32, cz:f32, r1: f32, r2:f32, step:f32) {
	let mut circ = Gmatrix::new();
	let mut n = (1.0/step+0.1) as usize;
	generate_torus(&mut circ, cx, cy, cz, r1, r2, step);

	let lat_start = 0;
	let lat_stop = n;
	let long_start = 0;
	let long_stop = n;

	let mut i;
	n+=1;

	for lat in lat_start..lat_stop {
		for long in long_start..long_stop {
			i = lat*n+long;
			edges.add_tri(
				circ.get_val(0,i) as i32,
				circ.get_val(1,i) as i32,
				circ.get_val(2,i) as i32,

				circ.get_val(0,i+1) as i32,
				circ.get_val(1,i+1) as i32,
				circ.get_val(2,i+1) as i32,

				circ.get_val(0,i+1+n) as i32,
				circ.get_val(1,i+1+n) as i32,
				circ.get_val(2,i+1+n) as i32
				);
			edges.add_tri(
				circ.get_val(0,i) as i32,
				circ.get_val(1,i) as i32,
				circ.get_val(2,i) as i32,

				circ.get_val(0,i+1+n) as i32,
				circ.get_val(1,i+1+n) as i32,
				circ.get_val(2,i+1+n) as i32,

				circ.get_val(0,i+n) as i32,
				circ.get_val(1,i+n) as i32,
				circ.get_val(2,i+n) as i32
				);
		}
	}
}

fn generate_torus(edges: &mut Gmatrix, cx:f32, cy:f32, cz:f32, r1: f32, r2:f32, step:f32) {
	let mut rot = 0.0;
	let mut mrot;
	let mut big_ctr = 0;
	let num_steps = (1.0/step) as i32;
	while big_ctr <= num_steps {
		let mut circ = 0.0;
		let mut mcirc;
		let mut smal_ctr = 0;
		while smal_ctr <= num_steps {
			mrot = rot*2.0*PI;
			mcirc = circ*2.0*PI;
			let x = (mrot.cos() * ( mcirc.cos()*r1 + r2 ) + cx) as i32;
			let y = (r1*mcirc.sin() + cy) as i32;
			let z = (-1.0 * mrot.sin() * (r1*mcirc.cos() + r2) + cz) as i32;
			edges.add_pt(x,y,z);
			circ += step;
			smal_ctr += 1;
		}
		rot += step;
		big_ctr+=1;
	}
}