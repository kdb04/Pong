use raylib::prelude::*;
use raylib::consts::KeyboardKey;
use raylib::color::Color;

use crate::ball::Ball;
use crate::paddle::Paddle;

const SCREEN_WIDTH: i32 = 1000;
const SCREEN_HEIGHT: i32 = 800;

pub fn single_player(rl: &mut RaylibHandle, ball: &mut Ball, paddle1: &mut Paddle, paddle2: &mut Paddle, score1: &mut i32, score2: &mut i32, timer: &mut f32, thread: &RaylibThread){
    while !rl.window_should_close(){
        ball.change_dir(SCREEN_WIDTH, SCREEN_HEIGHT);
        paddle2.update(SCREEN_HEIGHT, rl.is_key_down(KeyboardKey::KEY_UP), rl.is_key_down(KeyboardKey::KEY_DOWN));
        //auto-controlling paddle1
        if ball.y<paddle1.y{
            paddle1.speed = -5.0;
        }
        else if ball.y>(paddle1.y+100){
            paddle1.speed = 5.0;
        }
        else{
            paddle1.speed = 0.0;
        }
        paddle1.update(SCREEN_HEIGHT, false, false);

        *timer -= rl.get_frame_time();
        if *timer<=0.0{
            break;
        }

        if (ball.x <= paddle1.x + 10) && (ball.y >= paddle1.y) && (ball.y <= paddle1.y + 100) {
            ball.vx = -ball.vx;
        }
        if (ball.x + 10 >= paddle2.x) && (ball.y >= paddle2.y) && (ball.y <= paddle2.y + 100) {
            ball.vx = -ball.vx;
        }

        if (ball.x+10) >= SCREEN_WIDTH{
            *score1 += 1;
            *ball = Ball::init(SCREEN_WIDTH/2, SCREEN_HEIGHT/2, -5.0, -5.0);
        }
        if ball.x<=0{
            *score2 += 1;
            *ball = Ball::init(SCREEN_WIDTH/2, SCREEN_HEIGHT/2, 5.0, 5.0);
        }
    }
}

pub fn multi_player(rl: &mut RaylibHandle, ball: &mut Ball, paddle1: &mut Paddle, paddle2: &mut Paddle, score1: &mut i32, score2: &mut i32, timer: &mut f32, thread: &RaylibThread){
    while(!rl.window_should_close()){
        ball.change_dir(SCREEN_WIDTH, SCREEN_HEIGHT);
        paddle1.update(SCREEN_HEIGHT, rl.is_key_down(KeyboardKey::KEY_W), rl.is_key_down(KeyboardKey::KEY_S));
        paddle2.update(SCREEN_HEIGHT, rl.is_key_down(KeyboardKey::KEY_UP), rl.is_key_down(KeyboardKey::KEY_DOWN));

        *timer -= rl.get_frame_time();
        if *timer<=0{
            break;
        }

        if (ball.x <= paddle1.x + 10) && (ball.y >= paddle1.y) && (ball.y <= paddle1.y + 100) {
            ball.vx = -ball.vx;
        }
        if (ball.x + 10 >= paddle2.x) && (ball.y >= paddle2.y) && (ball.y <= paddle2.y + 100) {
            ball.vx = -ball.vx;
        }

        if (ball.x+10)>=SCREEN_WIDTH{
            *score1 += 1;
            *ball = Ball::init(SCREEN_WIDTH/2, SCREEN_HEIGHT/2, -5.0, -5.0);
        }
        if ball.x<=0{
            *score2 += 1;
            *ball = Ball::init(SCREEN_WIDTH/2, SCREEN_HEIGHT/2, 5.0, 5.0);
        }
    }
}
