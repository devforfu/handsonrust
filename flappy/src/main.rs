use bracket_lib::prelude::*;

const PLAYER_X: i32 = 5;
const PLAYER_Y: i32 = 25;
const SCREEN_W: i32 = 80;
const SCREEN_H: i32 = 50;
const FRAME_DURATION: f32 = 75.0;


fn main() -> BError {
    let ctx = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(ctx, State::new(PLAYER_X, PLAYER_Y))
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y, velocity: 0.0 }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle { x, gap_y: random.range(10, 40), size: i32::max(2, 20 - score) }
    }

    fn half_size(&self) -> i32 { self.size / 2 }

    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let d = self.half_size();
        for y in 0..SCREEN_H {
            if y >= (self.gap_y - d) && y < (self.gap_y + d) { continue }
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }

    fn behind(&self, player: &Player) -> bool {
        self.x < player.x
    }

    fn hit(&self, player: &Player) -> bool {
        let d = self.half_size();
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_y - d;
        let player_below_gap = player.y > self.gap_y + d;
        does_x_match && (player_above_gap || player_below_gap)
    }
}

enum GameMode {
    Menu,
    End,
    Playing,
}

struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
    obstacle: Obstacle,
    score: i32,
}

impl State {
    fn new(x: i32, y: i32) -> Self {
        Self {
            player: Player::new(x, y),
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_W, 0),
            mode: GameMode::Menu,
            score: 0,
        }
    }
    
    fn restart(&mut self) {
        self.player = Player::new(PLAYER_X, PLAYER_Y);
        self.frame_time = 0.0;
        self.obstacle = Obstacle::new(SCREEN_W, 0);
        self.mode = GameMode::Playing;
        self.score = 0;
    }
    
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.print_centered(5, "Welcome to the Flappy Dragon!");
        ctx.print_centered(8, "(P) Play the Game");
        ctx.print_centered(9, "(Q) Quit");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");
        ctx.print(0, 1, &format!("Score: {}", self.score));
        self.obstacle.render(ctx, self.player.x);
        if self.obstacle.behind(&self.player) {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_W, self.score);
        }
        if self.player.y > SCREEN_H || self.obstacle.hit(&self.player) {
            self.mode = GameMode::End;
        }
    }
    
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.print_centered(7, &format!("Game Over! Your score: {}", self.score));
        ctx.print_centered(8, "Press (R) to restart, or (Q) to quit.");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::R => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}


