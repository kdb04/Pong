use raylib::prelude::*;
use raylib::consts::KeyboardKey;
use raylib::color::Color;
use crate::ball::Ball;
use crate::paddle::Paddle;

const SCREEN_WIDTH: i32 = 1000;
const SCREEN_HEIGHT: i32 = 800;

pub fn single_player(rl: &mut RaylibHandle, thread: &RaylibThread, ball: &mut Ball, paddle1: &mut Paddle, paddle2: &mut Paddle, score1: &mut i32, score2: &mut i32, timer: &mut f32, font: &WeakFont, difficulty: i32){
    let mut draw = rl.begin_drawing(thread);
    draw.clear_background(Color::AZURE);

    ball.change_dir(SCREEN_WIDTH, SCREEN_HEIGHT);
    paddle2.update(SCREEN_HEIGHT, draw.is_key_down(KeyboardKey::KEY_UP), draw.is_key_down(KeyboardKey::KEY_DOWN));

   //custom difficulty levels
   let ai_speed = match difficulty {
      1 => 3.0,
      2 => 5.0,
      3 => 7.0,
      _ => 3.0
   };

   let target_y = (ball.y - 50).clamp(0, SCREEN_HEIGHT - 100);
   if (paddle1.y - target_y).abs() > ai_speed as i32{
       if paddle1.y < target_y{
           paddle1.y += ai_speed as i32;
       }
       else{
           paddle1.y -= ai_speed as i32;
       }
   }
   else{
       paddle1.y = target_y;
   }

    update_game_state(ball, paddle1, paddle2, score1, score2);

    *timer -= draw.get_frame_time();

    draw_game(&mut draw, ball, paddle1, paddle2, *score1, *score2, *timer, font);
}

pub fn multi_player(rl: &mut RaylibHandle, thread: &RaylibThread, ball: &mut Ball, paddle1: &mut Paddle, paddle2: &mut Paddle, score1: &mut i32, score2: &mut i32, timer: &mut f32, font: &WeakFont, cooldown_time: &mut f32, frame_time: f32){
    let mut draw = rl.begin_drawing(thread);
    draw.clear_background(Color::AZURE);

    if *cooldown_time > 0.0{
        *cooldown_time -= frame_time;
        draw.draw_text(&format!("Cooldown Time: {:.0}", cooldown_time.ceil()), SCREEN_WIDTH/2 - 100, SCREEN_HEIGHT/2, 30, Color::GREEN);
        draw_game(&mut draw, ball, paddle1, paddle2, *score1, *score2, *timer, font);
        return;
    }

    ball.change_dir(SCREEN_WIDTH, SCREEN_HEIGHT);
    paddle1.update(SCREEN_HEIGHT, draw.is_key_down(KeyboardKey::KEY_W), draw.is_key_down(KeyboardKey::KEY_S));
    paddle2.update(SCREEN_HEIGHT, draw.is_key_down(KeyboardKey::KEY_UP), draw.is_key_down(KeyboardKey::KEY_DOWN));

    if (ball.x + 10) >= SCREEN_WIDTH{
        *score1+=1;
        *ball = Ball::init(SCREEN_WIDTH/2, SCREEN_HEIGHT/2, -5.0, -5.0);
        *cooldown_time = 2.0;
    }if(ball.x) <= 0{
        *score2+=1;
        *ball = Ball::init(SCREEN_WIDTH/2, SCREEN_HEIGHT/2, 5.0, 5.0);
        *cooldown_time = 2.0;
    }

    update_game_state(ball, paddle1, paddle2, score1, score2);

    if *cooldown_time <= 0.0{
        *timer -= frame_time;
    }

    draw_game(&mut draw, ball, paddle1, paddle2, *score1, *score2, *timer, font);
}

fn update_game_state(ball: &mut Ball, paddle1: &Paddle, paddle2: &Paddle, score1: &mut i32, score2: &mut i32){
    if (ball.x <= paddle1.x + 10) && (ball.y >= paddle1.y) && (ball.y <= paddle2.y + 100){
        ball.vx = -ball.vx;
    }
    if (ball.x + 10 >= paddle2.x) && (ball.y >= paddle2.y) && (ball.y <= paddle2.y + 100){
        ball.vx = -ball.vx;
    }
    if (ball.x + 10) >= SCREEN_WIDTH{
        *score1 += 1;
        *ball = Ball::init(SCREEN_WIDTH/2, SCREEN_HEIGHT/2, -5.0, -5.0);
    }
    if ball.x <= 0{
        *score2 += 1;
        *ball = Ball::init(SCREEN_WIDTH/2, SCREEN_HEIGHT/2, 5.0, 5.0);
    }
}

fn draw_game(draw: &mut RaylibDrawHandle, ball: &mut Ball, paddle1: &Paddle, paddle2: &Paddle, score1: i32, score2: i32, timer: f32, font: &WeakFont){
    draw.clear_background(Color::AZURE);

    //scoreboard to top-left corner
    let start_x = 20.0;
    let mut y_offset = 20.0;
    let line_height = 40.0;
    let font_size = 36.0;
    let spacing = 2.0;

    draw.draw_text_ex(font, &format!("Player 1: {}", score1), Vector2::new(start_x, y_offset), font_size, spacing, Color::GOLD);

    y_offset += line_height;
    draw.draw_text_ex(font, &format!("Timer: {:.1}", timer), Vector2::new(start_x, y_offset), font_size, spacing, Color::GREEN);

    y_offset += line_height;
    draw.draw_text_ex(font, &format!("Player 2: {}", score2), Vector2::new(start_x, y_offset), font_size, spacing, Color::DARKBLUE);

    paddle1.draw(draw);
    paddle2.draw(draw);
    ball.draw(draw);
}
