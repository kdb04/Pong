use raylib::core::*;
use raylib::prelude::*;

pub struct Ball{
    pub x: i32,
    pub y: i32,//positions
    pub vx: f32,
    pub vy: f32,//velocities
}

impl Ball{
    pub fn init(x:i32, y:i32, vx:f32, vy:f32)->Self{
        Ball{x,y,vx,vy}
    }

    pub fn change_dir(&mut self, screen_width: i32, screen_height: i32){
        self.x += self.vx as i32;
        self.y += self.vy as i32;

        if (self.x <= 0 || self.x + 10 >= screen_width){
            self.vx = -self.vx; //change direction after collision
        }

        if (self.y <=0 || self.y + 10 >= screen_height){
            self.vy = -self.vy;
        }

    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle){
        d.draw_circle(self.x, self.y, 10, Color::BLUE);
    }
}
