use ndarray::Array2;
use std::time::SystemTime;
use rand::prelude::*;


pub mod Entity;
pub mod Food;



pub struct Game {
    pub x: i32,
    pub mouse_position: (i32, i32),
    pub entities: Vec<Entity::Entity>,
    pub foods: Vec<Food::Food>,    
    pub delta_time: f32,
    pub last_time: SystemTime,
    pub food_counter: SystemTime,
    pub last_food: u64,
    pub best_entity: usize,
    pub all_entities_created: Vec<Entity::Entity>,
}

impl Game {

    pub fn new() -> Game {
        Game {
            x: 10,
            mouse_position: (0, 0),
            entities: Vec::new(),
            foods: Vec::new(),            
            delta_time: 0.0,
            last_time: SystemTime::now(),
            food_counter: SystemTime::now(),
            last_food: 0,
            best_entity: 0,
            all_entities_created: vec![],
        }
    }

    pub fn update_time(&mut self) {
        self.delta_time =  (SystemTime::now().duration_since(self.last_time)).unwrap().as_micros() as f32 / 500.0 ;
        if self.delta_time > 150.0 {
            self.delta_time = 0.0;
        }
        self.last_time = SystemTime::now();
    }



}








pub fn rand_matrix<A>(outer_length: usize, inner_length: usize) -> Array2<f32> {

    let mut _matrix = Array2::<f32>::default([inner_length, outer_length]);
    let mut rng = rand::thread_rng();

    for i in 0..outer_length {
        for j in 0..inner_length { 
            _matrix.column_mut(i)[j] = rng.gen_range(-1.0..1.0);
            //_matrix
        } 
    }
    _matrix

}

pub fn adjust_weights<A>(mut matrix: Array2<f32>, step_size: f32) -> Array2<f32> {
    let mut rng = rand::thread_rng();
    for mut i in matrix.rows_mut() {
        for j in 0..i.len() {
            i[j] += rng.gen_range(-step_size..step_size);
        }
    }

    matrix


}