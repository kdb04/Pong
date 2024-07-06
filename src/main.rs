mod ball;
mod paddle;
mod game_modes;

use::raylib::color::Color;
use::raylib::prelude::*;
use::raylib::consts::KeyboardKey;

use ball::Ball;
use paddle::Paddle;
use game_modes::*;

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

    let mut game_mode = 1; //default: multi-player

    match game_mode{
        1 =>{
            while !rl.window_should_close(){
                match rl.get_key_pressed(){
                    Some(KeyboardKey::KEY_ONE)=>{
                        game_mode = 1;
                    },
                    Some(KeyboardKey::KEY_TWO)=>{
                        game_mode = 2;
                    },
                    _ => {}
                }
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
        },
        2 => {
            single_player(&mut rl, &mut ball, &mut paddle1, &mut paddle2, &mut score1, &mut score2, &mut timer, &thread)
        },
        _ => {
            println!("Invalid game mode!");
            return;
        }
    }

    //Winner
    let mut draw = rl.begin_drawing(&thread);
    draw.clear_background(Color::BLACK);
    if score1>score2{
        draw.draw_text("Player1 wins!", SCREEN_WIDTH as i32 /2 - 100, SCREEN_HEIGHT as i32 /2, 50, Color::WHITE);
    }
    else if score2>score1{
        draw.draw_text("Player2 wins!", SCREEN_WIDTH as i32 /2 - 100, SCREEN_HEIGHT as i32 /2, 50, Color::WHITE);
    }
    else{
        draw.draw_text("Tie!", SCREEN_WIDTH as i32 /2 - 100, SCREEN_HEIGHT as i32 /2, 50, Color::WHITE);
    }
}
