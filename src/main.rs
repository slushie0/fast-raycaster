use std::{f32::{consts::PI}};
use macroquad::{math::*, prelude::*};

const PLAYER_SPEED: f32 = 1.0;

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

fn a2 (y:f32, x:f32) -> f32 {
    return (y.atan2(x) * 180.0) / PI;
}

fn ray (x:f32, y:f32, dir:f32, clip:f32, wall:[f32; 4]) -> f32 {
    let mut closest: f32 = clip;
		
	// This section is Iron Programming's code

	// shortening variables
    let (x1, y1, x2, y2) = (wall[0], wall[1], wall[2], wall[3]);
    let (x3, y3, x4, y4) = (x, y, x+clip*cos_deg(dir), y+clip*-sin_deg(dir));
        
    // denominator
    let den: f32 = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    if den == 0.0 {
        return 0.0;
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
        }
    }
    // This is the end of Iron Programming's code
    if closest == clip {
	    return 0.0;
	}
	return closest;
}

#[macroquad::main("Raycaster")]
async fn main() {

    let width = screen_width();
    let height = screen_height();
    let walls: [[f32; 4]; 5] = [[0.0, 0.0, 400.0, 0.0], [400.0, 0.0, 400.0, 400.0], [400.0, 400.0, 0.0, 400.0], [0.0, 400.0, 0.0, 0.0], [200.0, 40.0, 300.0, 300.0]];
    //println!("Dist: {}", dist(200.0, 200.0, 200.0, 0.0));
    //let gar = (a2(walls[i][1] - 200.0, walls[i][0] - 200.0) + 360.0) % 360.0;
    //println!("Ray : {}", ray(200.0, 200.0, 0.0, 500.0, walls[1]));
    
    let fov: f32 = 66.0;
    //let viewing_plane = width;
    //let focal_length = viewing_plane / 2.0;

    let half_fov: f32 = fov/2.0;

    
    let angle_increment = fov/width;
    let x_increment = width/fov;
    
    let mut wall_lighting: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
    let mut angle: f32 = 0.0;
    let mut x = 20.0;
    let mut y = 20.0;

    let mut timer: i32;
    let mut timer_old: i32 = 0;
    let mut fps: i32 = 0;

    //calculate wall lighting (darkness depends on angle)
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
            x += PLAYER_SPEED*cos_deg(angle);
            y -= PLAYER_SPEED*sin_deg(angle);
        }
        if is_key_down(KeyCode::D) {
            x = x+PLAYER_SPEED*cos_deg(angle-90.0);
            y = y-PLAYER_SPEED*sin_deg(angle-90.0);
        }
        if is_key_down(KeyCode::S) {
            x = x+PLAYER_SPEED*cos_deg(angle+180.0);
            y = y-PLAYER_SPEED*sin_deg(angle+180.0);
        }
        if is_key_down(KeyCode::A) {
            x = x+PLAYER_SPEED*cos_deg(angle+90.0);
            y = y-PLAYER_SPEED*sin_deg(angle+90.0);
        }
        if is_key_down(KeyCode::Left) {
            angle += PLAYER_SPEED;
            if angle > 360.0 {
                angle = 0.0;
            }
        }
        if is_key_down(KeyCode::Right) {
            angle -= PLAYER_SPEED;
            if angle < 0.0 {
                angle = 360.0;
            }
        }
        clear_background(Color { r: 0.0, g: 1.0, b: 1.0, a: 1.0 });
        draw_rectangle(0.0, screen_height()*0.5, screen_width(), screen_height()*0.5, GREEN);

        for i in 0..4 {
            //get angle and distance between the camera and wall points
            let angle1 = (a2(walls[i][1] - y, walls[i][0] - x) + 360.0) % 360.0;
            let angle2 = (a2(walls[i][3] - y, walls[i][2] - x) + 360.0) % 360.0;
            let mut dist1 = dist(x, y, walls[i][0], walls[i][1]);
            let mut dist2 = dist(x, y, walls[i][2], walls[i][3]);

            //shift the angle of wall points to account for player rotation and fov
            let mut shift1 = angle1+angle+33.0;
            let mut shift2 = angle2+angle+33.0;
            while shift1 > 360.0 { shift1 -= 360.0; }
            while shift2 > 360.0 { shift2 -= 360.0; }

            let mut ac1 = shift1 - 33.0;
            let mut ac2 = shift2 - 33.0;
            if ac1 < 0.0 { ac1 = ac1 + 360.0; }
            if ac2 < 0.0 { ac2 = ac2 + 360.0; }

            //frustum culling
            /*if
                (ac1 > 90.0 && ac1 < 270.0) && (ac2 > 90.0 && ac2 < 270.0) ||
                (ac1 > 33.0 && ac1 < 213.0) && (ac2 > 33.0 && ac2 < 213.0) ||
                (ac1 > 147.0 && ac1 < 327.0) && (ac2 > 147.0 && ac2 < 327.0)
            {
                continue;
            }*/
            if
                (ac1 > 90.0 && ac1 < 270.0) && (ac2 > 90.0 && ac2 < 270.0) ||
                (ac1 > 34.0 && ac1 < 212.0) && (ac2 > 34.0 && ac2 < 212.0) ||
                (ac1 > 148.0 && ac1 < 326.0) && (ac2 > 148.0 && ac2 < 326.0)
            {
                //continue;
            }

            let mut x1 = shift1*x_increment;
            let mut x2 = shift2*x_increment;

            //cast ray if on vertex is off-screen
            if ac1 > 33.0 && ac1 < 180.0 {
                x1 = width;
                shift1 = 66.0;
                dist1 = ray(x, y, angle - half_fov, 500.0, walls[i]);
            } else if ac1 > 180.0 && ac1 < 327.0 {
                x1 = 0.0;
                shift1 = 0.0;
                dist1 = ray(x, y, angle + half_fov, 500.0, walls[i]);
            }
            if ac2 > 33.0 && ac2 < 180.0 {
                x2 = width; 
                shift2 = 66.0;
                dist2 = ray(x, y, angle - half_fov, 500.0, walls[i]);
            } else if ac2 > 180.0 && ac2 < 327.0 {
                x2 = 0.0;
                shift2 = 0.0;
                dist2 = ray(x, y, angle + half_fov, 500.0, walls[i]);
            }



            //
            // HERE IS THE PROBLEM!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            //
            let line_height1 = 10.0*width/dist1/cos_deg(shift1*angle_increment);
            let line_height2 = 10.0*width/dist2/cos_deg(shift2*angle_increment);
            let col = Color{r:wall_lighting[i], g:wall_lighting[i], b:wall_lighting[i], a:1.0};

            draw_triangle(
                vec2(x1, (height/2.0)+line_height1),
                vec2(x1, (height/2.0)-line_height1),
                vec2(x2, (height/2.0)+line_height2),
                col
            );
            draw_triangle(
                vec2(x2, (height/2.0)+line_height2),
                vec2(x2, (height/2.0)-line_height2),
                vec2(x1, (height/2.0)-line_height1),
                col
            );
        }
        
        timer = get_time().round() as i32;
        if timer != timer_old {
            fps = get_fps();
        }
        timer_old = timer;

        draw_text_ex(
            &fps.to_string(),
            10.0, 30.0,
            TextParams {
                font_size: (20),
                color: (BLACK),
                ..Default::default()
            }
        );

        next_frame().await;
    }
}
