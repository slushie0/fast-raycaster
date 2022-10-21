use std::f32::{consts::PI, INFINITY};
use macroquad::{math::*, models::*, models::Vertex, prelude::*};

fn draw_polygon(x: f32, y: f32, points: Vec<Vec2>, color: Color) {
    let points_length = points.len();
    let mut vertices = Vec::<Vertex>::with_capacity(points_length as usize + 2);
    let mut indices = Vec::<u16>::with_capacity(points_length as usize * 3);

    for (i, point) in points.iter().enumerate() {
        let vertex = Vertex {
            position: Vec3::new(x + point.x, y + point.y, 0.0),
            uv: Vec2::default(),
            color
        };

        vertices.push(vertex);
        indices.extend_from_slice(&[0, i as u16 + 1, i as u16 + 2]);
    }

    let mesh = Mesh {
        vertices,
        indices,
        texture: None,
    };

    draw_mesh(&mesh);
}

fn dist (x1:f32, y1:f32, x2:f32, y2:f32) -> f32 {
    let dx = x1 - x2;
    let dy = y1 - y2;
    return (dx * dx + dy * dy).sqrt();
}

fn cos_deg (deg:f32) -> f32 {
    return (deg * ( PI / 180.0 )).cos();
}

fn sin_deg (deg:f32) -> f32 {
    return (deg * ( PI / 180.0 )).sin();
}

fn tan_deg(deg:f32) -> f32 {
    return (deg * (PI / 180.0)).tan();
}

fn a2 (y:f32, x:f32) -> f32 {
    return (y.atan2(x) * 180.0) / PI;
}

fn intersects(a:f32,b:f32,c:f32,d:f32,p:f32,q:f32,r:f32,s:f32) -> bool {
    let (det, gamma, lambda);
    det = (c - a) * (s - q) - (r - p) * (d - b);
    if det == 0.0 {
        return false;
    }
    lambda = ((s - q) * (r - a) + (p - r) * (s - b)) / det;
    gamma = ((b - d) * (r - a) + (c - a) * (s - b)) / det;
    (0.0 < lambda && lambda < 1.0) && (0.0 < gamma && gamma < 1.0)
}

fn ray (x:f32, y:f32, dir:f32, clip:f32, walls:[[f32; 4]; 5], index:i32) -> (f32, i32) {
    let mut closest: f32 = clip;
    let mut wall_used: i32 = 0;
    for i in 0..4 {
        if index != 1000 && i as i32 != index {
            println!("index: {} | i: {}", index, i);
            continue;
        }
		
		// This section is Iron Programming's code

		// shortening variables
		let wall: [f32; 4] = walls[i];
        let (x1, y1, x2, y2) = (wall[0], wall[1], wall[2], wall[3]);
        let (x3, y3, x4, y4) = (x, y, x+clip*cos_deg(dir), y+clip*-sin_deg(dir));
            
        // returns true if the line from (a,b)->(c,d) intersects with (p,q)->(r,s)
        if intersects(x1, y1, x2, y2, x3, y3, x4, y4) {
        
	        // denominator
	        let den: f32 = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
	        if den == 0.0 {
	            return (0.0, 0);
	        }
	        
	        let t: f32 = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den;
	        let u: f32 = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / den;
	        
	        // Does the ray collide with a wall?
	        if t > 0.0 && t < 1.0 && u > 0.0 {
	            // If true, then where does it intersect?
	            let ptx: f32 = x1 + t * (x2 - x1);
	            let pty: f32 = y1 + t * (y2 - y1);
	            let d = dist(x3, y3, ptx, pty); // distance betwen 2 points
	            if d < closest {
					closest = d;
                    wall_used = i as i32;
				}
	        }
        }
        // This is the end of Iron Programming's code
	}
	//MAYBE IT CONTINUES TO HERE AND RETURNS 0?
    //if closest == clip {
	    //return (0.0, 0);
	//}
	return (closest, wall_used);
}

#[macroquad::main("Raycaster")]
async fn main() {
    let walls: [[f32; 4]; 5] = [[0.0, 0.0, 400.0, 0.0], [400.0, 0.0, 400.0, 400.0], [400.0, 400.0, 0.0, 400.0], [0.0, 400.0, 0.0, 0.0], [200.0, 40.0, 300.0, 300.0]];
    let fov: f32 = 66.0;
    let half_fov: f32 = fov/2.0;

    let width = screen_width();
    let height = screen_height();
    let angle_increment = fov/width;
    
    let mut wall_lighting: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
    let mut angle: f32 = 0.0;
    let mut x = 20.0;
    let mut y = 20.0;

    for i in 0..4 {
        let ex = walls[i][0];
        let ey = walls[i][1];
        let cx = walls[i][2];
        let cy = walls[i][3];
        let dy = ey - cy;
        let dx = ex - cx;
        let mut theta = dy.atan2(dx); // range (-PI, PI]
        theta *= 180.0 / PI; // rads to degs, range (-180, 180]
        if theta < 0.0 {
            //theta = 360.0 + theta; // range [0, 360)
            theta = theta.abs();
        }
        wall_lighting[i] = theta / 360.0;
    }

    loop {
        if is_key_down(KeyCode::W) {
            x += 1.0*cos_deg(angle);
            y -= 1.0*sin_deg(angle);
        }
        if is_key_down(KeyCode::D) {
            x = x+1.0*cos_deg(angle-90.0);
            y = y-1.0*sin_deg(angle-90.0);
        }
        if is_key_down(KeyCode::S) {
            x = x+1.0*cos_deg(angle+180.0);
            y = y-1.0*sin_deg(angle+180.0);
        }
        if is_key_down(KeyCode::A) {
            x = x+1.0*cos_deg(angle+90.0);
            y = y-1.0*sin_deg(angle+90.0);
        }
        if is_key_down(KeyCode::Left) {
            angle += 1.0;
            if angle > 360.0 {
                angle = 0.0;
            }
        }
        if is_key_down(KeyCode::Right) {
            angle -= 1.0;
            if angle < 0.0 {
                angle = 360.0;
            }
        }
        clear_background(Color { r: 0.0, g: 1.0, b: 1.0, a: 1.0 });
        draw_rectangle(0.0, screen_height()*0.5, screen_width(), screen_height()*0.5, GREEN);

        for i in 0..width as i32 {
            let ray_angle = (angle - half_fov) + i as f32 * angle_increment;
            let ray = ray(x, y, ray_angle, 600.0, walls, 1000);
            let dist = ray.0;
             
            let line_height = 10.0*width/dist/cos_deg((i as f32-width/2.0)*angle_increment);
            let col = wall_lighting[ray.1 as usize];

            draw_line(
                width - i as f32, ((height/2.0)+line_height) as f32,
                width - i as f32, ((height/2.0)-line_height) as f32,
                1.0, Color { r: col, g: col, b: col, a: 1.0 }
            );
        }

        for i in 0..4 {
            //get angle and distance between the camera and wall points
            let angle1 = (a2(walls[i][1] - y, walls[i][0] - x) + 360.0) % 360.0;
            let angle2 = (a2(walls[i][3] - y, walls[i][2] - x) + 360.0) % 360.0;
            let mut dist1 = dist(x, y, walls[i][0], walls[i][1]);
            let mut dist2 = dist(x, y, walls[i][2], walls[i][3]);


            //shift the angle of wall points to account for player rotation and fov
            let mut shift1 = angle1+angle+33.0;
            while shift1 > 360.0 { shift1 -= 360.0; }
            let mut shift2 = angle2+angle+33.0;
            while shift2 > 360.0 { shift2 -= 360.0; }

            let mut x1 = shift1/angle_increment;
            let mut x2 = shift2/angle_increment;

            //frustum culling
            //213 degrees (180+33) is the exact back of the camera
            if
                (shift1 > 123.0 && shift1 < 303.0) && (shift2 > 123.0 && shift2 < 303.0) ||
                (shift1 > 66.0 && shift1 < 246.0)  && (shift2 > 66.0 && shift2 < 246.0) ||
                (shift1 > 180.0 && shift1 < 360.0)  && (shift2 > 180.0 && shift2 < 360.0) {
                //println!("{}",  shift1);
                continue;
            }

            if shift1 > 60.0 && shift1 < 213.0 {
                x1 = width;
                let ray = ray(x, y, angle - half_fov, 500.0, walls ,i as i32);
                dist1 = ray.0;
            } else if shift1 > 213.0 {
                let ray = ray(x, y, angle + half_fov, 500.0, walls, i as i32);
                dist1 = ray.0;
                x1 = 0.0;
            }
            if shift2 > 60.0 && shift2 < 213.0 {
                let ray = ray(x, y, angle - half_fov, 500.0, walls, i as i32);
                dist2 = ray.0;
                x2 = width; 
            } else if shift2 > 213.0 {
                let ray = ray(x, y, angle + half_fov, 500.0, walls, i as i32);
                dist2 = ray.0;
                x2 = 0.0;
            }

            let mut line_height1 = 10.0*width/dist1/cos_deg(shift1*angle_increment);
            let mut line_height2 = 10.0*width/dist2/cos_deg(shift2*angle_increment);
            if line_height1 == INFINITY {
                line_height1 = 0.0;
            }
            if line_height2 == INFINITY {
                line_height2 = 0.0;
            }

            println!("x1: {} | x2: {}", x1, x2);
            println!("line1: {} | line2: {}", line_height1, line_height2);
            println!("dist1: {} | dist2: {}", dist1, dist2);

            draw_line(
                x+line_height1*cos_deg(angle1),
                y+line_height1*-sin_deg(angle1),
                x+line_height2*cos_deg(angle2),
                y+line_height2*-sin_deg(angle2),
                2.0,
                GREEN,
            );

            draw_line(
                x1, (height/2.0)+line_height1,
                x1, (height/2.0)-line_height1,
                3.0, RED
            );
            draw_line(
                x2, (height/2.0)+line_height2,
                x2, (height/2.0)-line_height2,
                3.0, RED
            );
            draw_line(
                x1, (height/2.0)+line_height1,
                x2, (height/2.0)+line_height2,
                3.0, RED
            );
            draw_line(
                x1, (height/2.0)-line_height1,
                x2, (height/2.0)-line_height2,
                3.0, RED
            );

            /*
            let mut vec = Vec::new();
            vec.push(vec2(shift1/angle_increment, (height/2.0)+line_height1));
            vec.push(vec2(shift2/angle_increment, (height/2.0)+line_height2));
            vec.push(vec2(shift1/angle_increment, (height/2.0)-line_height1));
            
            vec.push(vec2(shift2/angle_increment, (height/2.0)-line_height2));

            let col = wall_lighting[i];
            draw_polygon(0.0, 0.0, vec, Color { r: col, g: col, b: col, a: 1.0 });
            */
            for i in 0..4 {
                let wall = walls[i];
                draw_line(wall[0], wall[1], wall[2], wall[3], 4.0, BLACK);
                draw_circle(x, y, 5.0, RED);
                draw_triangle(vec2(x, y), vec2(x+10.0*cos_deg(angle-33.0), y+10.0*-sin_deg(angle-33.0)), vec2(x+10.0*cos_deg(angle+33.0), y+10.0*-sin_deg(angle+33.0)), YELLOW);
            }
        }
        
        next_frame().await;
    }
}
