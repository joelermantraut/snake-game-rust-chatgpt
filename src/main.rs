extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::Rng;

const GRID_SIZE: f64 = 20.0;
const WINDOW_WIDTH: f64 = 640.0;
const WINDOW_HEIGHT: f64 = 480.0;
const INITIAL_SNAKE_SPEED: f64 = 0.1;  // Velocidad de la serpiente

struct Game {
    snake: Snake,
    food: Food,
}

struct Snake {
    body: Vec<(f64, f64)>,
    direction: Direction,
    speed: f64,
    accumulated_dt: f64,
}

struct Food {
    position: (f64, f64),
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Game {
    fn new() -> Game {
        Game {
            snake: Snake::new(),
            food: Food::new(),
        }
    }
}

impl Snake {
    fn new() -> Snake {
        Snake {
            body: vec![(0.0, 0.0)], // La serpiente comienza en la posición (0, 0)
            direction: Direction::Right,
            speed: INITIAL_SNAKE_SPEED,
            accumulated_dt: 0.0,
        }
    }

    fn move_up(&mut self) {
        if self.direction != Direction::Down {
            self.direction = Direction::Up;
        }
    }

    fn move_down(&mut self) {
        if self.direction != Direction::Up {
            self.direction = Direction::Down;
        }
    }

    fn move_left(&mut self) {
        if self.direction != Direction::Right {
            self.direction = Direction::Left;
        }
    }

    fn move_right(&mut self) {
        if self.direction != Direction::Left {
            self.direction = Direction::Right;
        }
    }

    fn update(&mut self, dt: f64, window_width: f64, window_height: f64) {
        self.accumulated_dt += dt;

        if self.accumulated_dt >= self.speed {
            self.accumulated_dt -= self.speed;

            let (head_x, head_y) = self.body[0];
            let new_head = match self.direction {
                Direction::Up => (head_x, head_y - GRID_SIZE),
                Direction::Down => (head_x, head_y + GRID_SIZE),
                Direction::Left => (head_x - GRID_SIZE, head_y),
                Direction::Right => (head_x + GRID_SIZE, head_y),
            };

            // Verificar si la nueva posición está fuera de los límites
            let (new_head_x, new_head_y) = new_head;
            let new_head_x = if new_head_x < 0.0 {
                window_width - GRID_SIZE
            } else if new_head_x >= window_width {
                0.0
            } else {
                new_head_x
            };
            let new_head_y = if new_head_y < 0.0 {
                window_height - GRID_SIZE
            } else if new_head_y >= window_height {
                0.0
            } else {
                new_head_y
            };

            self.body.insert(0, (new_head_x, new_head_y));
            self.body.pop();
        }
    }

    fn check_collision(&mut self) {
        let (head_x, head_y) = self.body[0];

        /*
        if head_x < 0.0 || head_x >= WINDOW_WIDTH || head_y < 0.0 || head_y >= WINDOW_HEIGHT {
            self.restart_game();
        }
        */
    }

    fn check_collision_with_food(&mut self, food: &mut Food) {
        let (head_x, head_y) = self.body[0];
        let food_x = food.position.0;
        let food_y = food.position.1;
        let collision_margin = GRID_SIZE / 2.0;
    
        if (head_x >= food_x - collision_margin && head_x <= food_x + collision_margin)
            && (head_y >= food_y - collision_margin && head_y <= food_y + collision_margin)
        {
            self.grow();
            food.respawn();
        }
    }
    

    fn restart_game(&mut self) {
        self.body = vec![(0.0, 0.0)];
        self.direction = Direction::Right;
        self.speed = INITIAL_SNAKE_SPEED;
        self.accumulated_dt = 0.0;
    }

    fn grow(&mut self) {
        let last_segment_index = self.body.len() - 1;
        let (last_x, last_y) = self.body[last_segment_index];
        self.body.push((last_x, last_y)); // Añadir un nuevo segmento al final del cuerpo de la serpiente
    }
}

impl Food {
    fn new() -> Food {
        Food {
            position: (0.0, 0.0),
        }
    }

    fn respawn(&mut self) {
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen_range(0.0..WINDOW_WIDTH);
        let y: f64 = rng.gen_range(0.0..WINDOW_HEIGHT);

        self.position = (x, y);
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake Game", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new();

    game.food.respawn();

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Up => game.snake.move_up(),
                Key::Down => game.snake.move_down(),
                Key::Left => game.snake.move_left(),
                Key::Right => game.snake.move_right(),
                _ => {}
            }
        }

        if let Some(update_args) = event.update_args() {
            game.snake.update(update_args.dt, WINDOW_WIDTH, WINDOW_HEIGHT);
            game.snake.check_collision_with_food(&mut game.food);
        }

        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);

            rectangle(
                [1.0, 0.0, 0.0, 1.0], // Color rojo para la comida
                [game.food.position.0, game.food.position.1, GRID_SIZE, GRID_SIZE],
                context.transform,
                graphics,
            );

            for (index, &(x, y)) in game.snake.body.iter().enumerate() {
                let color = if index == 0 {
                    [0.0, 0.0, 1.0, 1.0] // Color azul para la cabeza de la serpiente
                } else {
                    [0.0, 0.5, 0.0, 1.0] // Color verde para el cuerpo de la serpiente
                };
        
                rectangle(
                    color,
                    [x, y, GRID_SIZE, GRID_SIZE],
                    context.transform,
                    graphics,
                );
        
                if index == 0 {
                    let eye_size = GRID_SIZE * 0.2;
                    let eye_offset_x = GRID_SIZE * 0.25;
                    let eye_offset_y = GRID_SIZE * 0.25;
        
                    ellipse(
                        [0.0, 0.0, 0.0, 1.0], // Color negro para el ojo
                        [x + eye_offset_x, y + eye_offset_y, eye_size, eye_size],
                        context.transform,
                        graphics,
                    );
                }
            }
        });

    }
}
