use std::cell::Cell;

use crate::apple::Apple;
use godot::classes::{Area2D, ColorRect, IArea2D, InputEvent};
use godot::prelude::*;
use rand::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
    None,
}

struct Color(u32, u32, u32);

#[derive(GodotClass)]
#[class(base=Area2D)]
struct Snake {
    head_position: Vector2,
    segments: Vec<Gd<ColorRect>>,
    segment_position: Vec<Vector2>,
    direction: Direction,
    old_direction: Direction,
    velocity: Vector2,
    time_since_last_move: f64,
    move_interval: f64,
    base: Base<Area2D>,
}

const CELL_SIZE: f32 = 60.0;

#[godot_api]
impl IArea2D for Snake {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            head_position: Default::default(),
            segments: Vec::new(),
            segment_position: Vec::new(),
            direction: Direction::None,
            old_direction: Direction::None,
            velocity: Default::default(),
            time_since_last_move: 0.,
            move_interval: 0.3,
            base,
        }
    }

    fn ready(&mut self) {
        let mut rng: ThreadRng = rand::rng();

        let x: f32 = rng.random_range(0..=9) as f32;
        let y: f32 = rng.random_range(0..=9) as f32;

        let head_position = Vector2::new(x, y);
        let gd = self.to_gd();

        self.head_position = head_position;

        self.base_mut()
            .signals()
            .area_entered()
            .connect_other(&gd, Self::_on_area_entered);

        self.base_mut().set_global_position(Vector2::new(x * CELL_SIZE, y * CELL_SIZE));
        self.segment_position.push(head_position);
        godot_print!("{}", head_position);
        self.base_mut().set_physics_process(false);
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        let direction = if event.is_action_pressed("ui_up") {
            Direction::Up
        } else if event.is_action_pressed("ui_down") {
            Direction::Down
        } else if event.is_action_pressed("ui_left") {
            Direction::Left
        } else if event.is_action_pressed("ui_right") {
            Direction::Right
        } else {
            Direction::None
        };

        if event.is_action_pressed("ui_accept") {
            self.direction = Direction::None;
            return;
        }

        if direction == Direction::None {
            return;
        }

        if direction != self.direction && self.direction != Direction::None {
            godot_print!("dir : {:?}", direction);
            self.direction = direction;
            godot_print!("premier if");
        } else if self.direction == Direction::None && direction != Direction::None {
            godot_print!("second if");
            self.direction = direction;
            self.base_mut().set_physics_process(true);
        }
    }

    // ne pas changer la pos de la tete et l'obtenir en grid pos de base
    fn physics_process(&mut self, delta: f64) {
        self.time_since_last_move += delta;

        if self.time_since_last_move < self.move_interval {
            return;
        }

        self.time_since_last_move = 0.;

        if self.direction != self.old_direction {
            self.velocity = match self.direction {
                Direction::Right => Vector2::RIGHT,
                Direction::Left => Vector2::LEFT,
                Direction::Down => Vector2::DOWN,
                Direction::Up => Vector2::UP,
                _ => Default::default(),
            };
            self.old_direction = self.direction;
        }

        self.head_position += self.velocity;

        let velocity = self.head_position * Vector2::splat(CELL_SIZE);

        self.base_mut().set_global_position(velocity);
        // self.update_segment_pos();
    }
}

impl Snake {
    fn add_segment(&mut self) {
       
    }

    // fn update_segment_pos(&mut self) {
    //     let t = self
    //         .segments
    //         .iter()
    //         .map(|x| x.get_global_position())
    //         .collect::<Vec<Vector2>>();
    //     for (i, segment) in self.segments.iter_mut().skip(1).enumerate() {
    //         godot_print!("pos: {}", t[i]);
    //         segment.set_global_position(t[i]);
    //     }
    // }

    fn _on_area_entered(&mut self, area: Gd<Area2D>) {
        if let Ok(obj) = area.try_cast::<Apple>() {
            self.add_segment();
            godot_print!(
                "{} detected",
                obj.get_name().to_string()[.."apple".len()].to_lowercase()
            );
        }
    }
}
