mod ball;
mod paddle;

use std::fmt::format;
use::raylib::core::*;
use::raylib::color::Color;
use::raylib::prelude::*;
use::raylib::consts::KeyboardKey;

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

    let mut score1 = 0;
    let mut score2 = 0;
    let mut timer = 60.0; //game terminates after 60 seconds

    while !rl.window_should_close(){
        ball.change_dir(SCREEN_WIDTH, SCREEN_HEIGHT);
        paddle1.update(SCREEN_HEIGHT, rl.is_key_down(KeyboardKey::KEY_W), rl.is_key_down(KeyboardKey::KEY_S));
        paddle2.update(SCREEN_HEIGHT, rl.is_key_down(KeyboardKey::KEY_UP), rl.is_key_down(KeyboardKey::KEY_DOWN));

        //Countdown timer
        timer -= rl.get_frame_time();
        if timer<=0.0{
            break;
        }

        let mut draw = rl.begin_drawing(&thread);
        draw.clear_background(Color::BLACK);

        //Collisions
        if (ball.x<=paddle1.x+10) && (ball.y>=paddle1.y) && (ball.y<=paddle1.y+100){
            ball.vx = -ball.vx;
        }
        if (ball.x+10>=paddle2.x) && (ball.y>=paddle2.y) && (ball.y<=paddle2.y+100){
            ball.vx = -ball.vx;
        }

        //Scoreboard
        if (ball.x + 10)>=SCREEN_WIDTH{
            score1+=1;
            ball = Ball::init(SCREEN_WIDTH/2, SCREEN_HEIGHT/2, -5.0, -5.0);
        }
        if ball.x<=0{
            score2+=1;
            ball = Ball::init(SCREEN_WIDTH/2, SCREEN_HEIGHT/2, 5.0, 5.0);
        }

        //Display scores
        draw.draw_text(format!("Player1:{}", score1).as_str(), 10, 10, 30, Color::WHITE);
        draw.draw_text(format!("Player2:{}", score2).as_str(), SCREEN_WIDTH as i32 - 150, 10, 30, Color::WHITE);
        draw.draw_text(format!("Timer:{:.0}",timer).as_str(), SCREEN_WIDTH as i32 /2 - 50, 10, 30, Color::WHITE);

        paddle1.draw(&mut draw);
        paddle2.draw(&mut draw);
        ball.draw(&mut draw);
    }

    //Winner
    let mut draw = rl.begin_drawing(&thread);
    draw.clear_background(Color::BLACK);
    if (score1>score2){
        draw.draw_text("Player1 wins!", SCREEN_WIDTH as i32 /2 - 100, SCREEN_HEIGHT as i32 /2, 50, Color::WHITE);
    }
    else if (score2>score1){
        draw.draw_text("Player2 wins!", SCREEN_WIDTH as i32 /2 - 100, SCREEN_HEIGHT as i32 /2, 50, Color::WHITE);
    }
    else{
        draw.draw_text("Tie!", SCREEN_WIDTH as i32 /2 - 100, SCREEN_HEIGHT as i32 /2, 50, Color::WHITE);
    }
}
