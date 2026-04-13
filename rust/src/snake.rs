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

#[derive(GodotClass)]
#[class(base=Area2D)]
struct Snake {
    head_position: Vector2,
    segments: Vec<Gd<ColorRect>>,
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
        let head_pos_coord = Vector2::new(x * CELL_SIZE, y * CELL_SIZE);
        let mut head: Gd<ColorRect> = self.base().get_node_as("head");
        let gd = self.to_gd();

        self.head_position = head_position;

        self.base_mut()
            .signals()
            .area_entered()
            .connect_other(&gd, Self::_on_area_entered);

        self.base_mut().set_global_position(head_pos_coord);

        head.set_global_position(head_pos_coord);

        self.segments.push(head);
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
            self.base_mut().set_physics_process(false);
            return;
        }

        if direction == Direction::None {
            return;
        }

        if direction != self.direction && self.direction != Direction::None {
            self.direction = direction;
        } else if self.direction == Direction::None && direction != Direction::None {
            self.direction = direction;
            self.base_mut().set_physics_process(true);
        }
    }

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
        let old_pos: Vec<Vector2> = self.segments.iter().map(|s| s.get_global_position()).collect();
        self.head_position += self.velocity;

        let velocity = self.head_position * Vector2::splat(CELL_SIZE);

        self.base_mut().set_global_position(velocity);
        self.segments.first_mut().unwrap().set_global_position(velocity);
        self.update_segment_pos(old_pos);
    }
}

impl Snake {
    fn add_segment(&mut self) {
        fn convert_to_grid_coord(global_position: Vector2) -> Vector2 {
            Vector2 {
                x: global_position.x / CELL_SIZE,
                y: global_position.y / CELL_SIZE,
            }
        }

        fn opposite(unit_vector: &Vector2) -> Vector2 {
            match *unit_vector {
                Vector2::RIGHT => Vector2::LEFT,
                Vector2::LEFT => Vector2::RIGHT,
                Vector2::UP => Vector2::DOWN,
                Vector2::DOWN => Vector2::UP,
                _ => Default::default(),
            }
        }

        let last_segment = self
            .segments
            .last()
            .expect("The last segment doesn't exist.");

        let last_segment_position = convert_to_grid_coord(last_segment.get_global_position());


        let mut segment = ColorRect::new_alloc();

        let cell_size = Vector2::splat(CELL_SIZE);
        segment.set_size(cell_size);

        let grid_coord = opposite(&self.velocity) + last_segment_position;

        self.base_mut().add_child(&segment);
        segment.set_global_position(grid_coord * cell_size);
        segment.set_owner(&self.to_gd());

        self.segments.push(segment);
    }

    fn update_segment_pos(&mut self, old_pos: Vec<Vector2>) {
        if self.segments.len() > 1 {
            for (i, seg) in self.segments.iter_mut().skip(1).enumerate() {
                seg.set_global_position(old_pos[i]);
            }
        }
    }

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
