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
    direction: Direction,
    old_direction: Direction,
    velocity: Vector2,
    time_since_last_move: f64,
    move_interval: f64,
    base: Base<Area2D>,
}

const CELL_SIZE: u16 = 60;

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
        let x: f32 = (rng.random_range(0..=9) * CELL_SIZE) as f32;
        let y: f32 = (rng.random_range(0..=9) * CELL_SIZE) as f32;
        let head_position: Vector2 = Vector2::new(x, y);

        self.head_position = head_position;
        godot_print!("start pos : {}", head_position);
        self.base_mut().set_global_position(head_position);

        // godot_print!("x: {}, y: {}", x, y);
        let gd = self.to_gd();

        self.base_mut()
            .signals()
            .area_entered()
            .connect_other(&gd, Self::_on_area_entered);
        self.base_mut().set_physics_process(false);
        let mut head: Gd<ColorRect> = self.base().get_node_as("head");
        head.set_global_position(head_position);
        self.segments.push(head);
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
            // godot_print!("dir changed");
        }

        let position = self.base().get_position();
        let velocity = position + (self.velocity * Vector2::splat(CELL_SIZE as f32));

        self.base_mut().set_global_position(velocity);
        self.segments
            .first_mut()
            .unwrap()
            .set_global_position(velocity);
        // self.update_segment_pos();
    }
}

impl Snake {
    fn add_segment(&mut self) {
        // grid coordinate
        let last_segment = self.segments.last().unwrap();
        let mut segment = ColorRect::new_alloc();
        let cell_size = CELL_SIZE as f32;

        fn convert_to_grid(lst_seg: &Vector2) -> Vector2 {
            let cell_size = CELL_SIZE as f32;

            Vector2 {
                x: lst_seg.x / cell_size,
                y: lst_seg.y / cell_size,
            }
        }

        segment.set_size(last_segment.get_size());

        let l_pos = convert_to_grid(&last_segment.get_global_position());


        let cell_size = CELL_SIZE as f32;

        let opp = match self.velocity {
            Vector2::RIGHT => Vector2::LEFT,
            Vector2::LEFT => Vector2::RIGHT,
            Vector2::UP => Vector2::DOWN,
            Vector2::DOWN => Vector2::UP,
            _ => Default::default(),
        };

        let pos = l_pos + (opp * Vector2::splat(cell_size));
        godot_print!("last segment pos: {}", l_pos);
        godot_print!("segment add pos : {}", pos);
        segment.set_position(pos);
        self.base_mut().add_child(&segment);
        segment.set_owner(&self.to_gd());
        self.segments.push(segment);
    }

    fn update_segment_pos(&mut self) {
        let t = self
            .segments
            .iter()
            .map(|x| x.get_global_position())
            .collect::<Vec<Vector2>>();
        for (i, segment) in self.segments.iter_mut().skip(1).enumerate() {
            godot_print!("pos: {}", t[i]);
            segment.set_global_position(t[i]);
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
