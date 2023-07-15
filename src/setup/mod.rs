use crow::Context;
use ndarray::Array2;

use std::time::SystemTime;
use std::vec;
use rand::prelude::*;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;


pub mod entity;
pub mod food;





pub struct Game {
    pub x: i32,
    pub mouse_position: (i32, i32),
    pub entities: Vec<entity::Entity>,
    pub foods: Vec<food::Food>,    
    pub delta_time: f32,
    pub last_time: SystemTime,
    pub food_counter: SystemTime,
    pub last_food: u64,
    pub best_entity: usize,
    pub all_entities_created: Vec<entity::Entity>,
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


    pub fn kill(&mut self, i: usize) {
        let entity = &self.entities[i];
        self.all_entities_created[entity.all_index] = entity.clone();

        println!("delted entity ({})", entity.all_index);
        
        if entity.surival_time > self.all_entities_created[self.best_entity].surival_time {
            self.best_entity = entity.all_index;
            println!("best entity died! - survival time: {}", entity.surival_time);
        }
        self.entities.swap_remove(i);
    }





    pub fn offspring_of(&mut self, entity: &entity::Entity, step_size: f32) {

        let mut e = entity.clone();

        e.pos = (entity.pos.0 - 150, entity.pos.1 - 150);
        e.rotation = entity.rotation - 70.0;
        for i in 0.. e.weights.len() {
            e.weights[i] = adjust_weights::<f32>(entity.weights[i].clone(), step_size);
        }
        self.entities.push(e);
    }
    


    pub fn spawn_foods(&mut self, ctx: &mut Context, food_spawn: u32, max_foods: u32) {
        let food_secs = self.food_counter.elapsed().unwrap().as_secs();
        if food_secs > self.last_food && self.foods.len() < max_foods as usize{
        let mut rng = rand::thread_rng();
            for _i in 0..food_spawn {
                let f = food::Food::new((rng.gen_range(0..ctx.window_width()) as i32, rng.gen_range(0..ctx.window_height()) as i32 as i32), ctx);
                self.foods.push(f);
            }

        self.last_food = food_secs;
        }   
    }

    pub fn spaw_entities(&mut self, entity_spawn: usize, ctx: &mut Context) {
        if self.entities.len() < entity_spawn {
            let mut rng = rand::thread_rng();
            entity::Entity::new((rng.gen_range(0..ctx.window_width()) as i32, rng.gen_range(0..ctx.window_height()) as i32 as i32), ctx, self);
        }
    }

    pub fn entities_eat(&mut self, food: &mut food::Food, i: usize) -> bool {
        let mut to_remove = vec![];
        for entity in &mut self.entities {
            let dx = entity.pos.0 - food.pos.0;
            let dy = entity.pos.1 - food.pos.1;
            
            let dist = f32::sqrt((dx*dx + dy*dy) as f32);
            if dist <= 30.0 {
                to_remove.push(i);
                entity.hunger += food.val;
                return true
            }
        }
        false
    }



    pub fn close_prints(&self) {
        println!();
        println!();
        println!();
        let mut survival_times: Vec<(u64, usize)> = vec![];
        for (i, entity) in self.all_entities_created.iter().enumerate() {
            survival_times.push((entity.surival_time, i))
        }
        survival_times.sort_unstable();

        for i in 0..survival_times.len() {
            println!();
            println!();
            println!();
            println!("{}/{}", survival_times.len() - i, survival_times.len());
            let entity = self.all_entities_created[survival_times[i].1].clone();
            println!("index: {}", entity.all_index);
            println!("survival time: {}", entity.surival_time);
            println!("speed: {}", entity.max_speed);
            println!("rotation: {}", entity.max_rotation_speed);
            println!("vision: \n    range: {}, \n    angle: {} \n    arcs:  {}", entity.vision_range, entity.vision_angle, entity.vision_arcs);

            if i > survival_times.len() - 4 {
                let path = format!("best_entities{}{}", r"\", survival_times.len() - i - 1);
                let data = self.write_to_file(entity);
                std::fs::write(path, data).unwrap();
            }

        }
        
    }


    pub fn write_to_file(&self, entity: entity::Entity) -> String {
        let mut s:String = String::new();   

        for e in vec![entity.max_speed, entity.max_rotation_speed, entity.vision_range, entity.vision_angle, entity.vision_arcs as f32] {
            s.push_str(&e.to_string());
            s.push_str("\n");
        }
        for e in entity.weights {
            for i in 0..e.shape()[0] {
                for j in 0..e.shape()[1] {
                    // s.push_str(entity.weights[i][[i, j]].to_string())
                    s.push_str("\n");
                    s.push_str(&e[[i, j]].to_string());
                }
                s.push_str("\n")
            }
            s.push_str("new array:")
        }
        s
    }


    pub fn read_from_file(&self, path: &str) -> (Vec<f32>, Vec<Array2<f32>>) {
        
        let mut result: Vec<f32> = vec![];
        
        let path = format!("saved_entities{}{}", r"\", path);
        let file = File::open(Path::new(&path)).unwrap();
        let reader = BufReader::new(&file);
        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

        let mut arrs:Vec<Vec<f32>> = vec![];
        let mut w:Vec<Array2<f32>> = vec![];
        let mut inner = vec![];

        for (i, l) in lines.iter().enumerate() {
            if i < 5 {
                result.push(l.parse::<f32>().unwrap());
                continue;
            }
            if i == 5 {
                continue
            }
            if l == "new array:" {
                arrs.push(inner);

                let inner_len = arrs[0].len();
                let outer_len = arrs.len();

                let mut a:Array2<f32> = Array2::zeros([outer_len, inner_len]);

                for i in 0..outer_len {
                    for j in 0..inner_len {
                        a[[i, j]] = arrs[i][j];
                    }
                }
                w.push(a);

                arrs = vec![];
                inner = vec![];
                continue
            }

            if l == "" {
                arrs.push(inner);
                inner = vec![];
                continue;
            }

            inner.push(l.parse::<f32>().unwrap());
        }
        (result, w)
    }


    // loads entities from a template -> a file in saved_entities with path = file_name, with data in the same format as best_entities saves
    pub fn load_entities_from_templates(&mut self, ctx: &mut Context, path: &str) {

        let v = self.read_from_file(path) ;

        let max_speed = v.0[0];
        let max_rotation_speed = v.0[1];
        let vision_range = v.0[2];
        let vision_angle = v.0[3];
        let vision_arcs = v.0[4];
    
        let weights = v.1;

        entity::Entity::new_from_template(max_speed, max_rotation_speed, vision_range, vision_angle, vision_arcs as i32, weights, ctx, self);
    
    }


}








pub fn rand_matrix<A>(outer_length: &usize, inner_length: &usize) -> Array2<f32> {

    let mut matrix = Array2::<f32>::zeros([*inner_length, *outer_length]);
    let mut rng = rand::thread_rng();

    for i in 0..*outer_length {
        for j in 0..*inner_length { 
            matrix[[j, i]] = rng.gen_range(-1.0..1.0);
        } 
    }
    matrix
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