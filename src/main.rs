mod ball;
mod paddle;

use::raylib::prelude::*;
use::raylib::consts::KeyboardKey;
use::raylib::core::color::Color;
use ball::Ball;
use paddle::Paddle;

const SCREEN_WIDTH: i32 = 1000;
const SCREEN_HEIGHT: i32 = 800;

fn main(){
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Ping-Pong")
        .build();
    rl.set_target_fps(60);
    let mut ball = Ball::init(SCREEN_WIDTH/2, SCREEN_HEIGHT, 5.0, 5.0);
    let mut paddle1 = Paddle::init(50, (SCREEN_HEIGHT/2)-50, 5.0); //Relative positions and velocity
    let mut paddle2 = Paddle::init(SCREEN_WIDTH-50-10, (SCREEN_HEIGHT/2)-50, 5.0);

    while !rl.window_should_close(){
        let mut draw = rl.begin_drawing(&thread);
        draw.clear_background(Color::BLACK);

        ball.change_dir(SCREEN_WIDTH, SCREEN_HEIGHT);
        paddle1.update(SCREEN_HEIGHT, rl.is_key_down(KeyboardKey::KEY_W), rl.is_key_down(KeyboardKey::KEY_S));
        paddle2.update(SCREEN_HEIGHT, rl.is_key_down(KeyboardKey::KEY_UP), rl.is_key_down(KeyboardKey::KEY_DOWN));

        //Collisions
        if (ball.x<=paddle1.x+10) && (ball.y>=paddle1.y) && (ball.y<=paddle1.y+100){
            ball.vx = -ball.vx;
        }
        if (ball.x+10>=paddle2.x) && (ball.y>=paddle2.y) && (ball.y<=paddle2.y+100){
            ball.vx = -ball.vx;
        }

        //Scoreboard
        if (ball.x + 10)>=SCREEN_WIDTH{
            println!("Player1 Scores!");
            ball = Ball::init(SCREEN_WIDTH/2, SCREEN_HEIGHT/2, -5.0, -5.0);
        }
        if ball.x<=0{
            println!("Player2 Scores");
            ball = Ball::init(SCREEN_WIDTH/2, SCREEN_HEIGHT/2, 5.0, 5.0);
        }

        paddle1.draw(&mut draw);
        paddle2.draw(&mut draw);
        ball.draw(&mut draw);
    }
}
