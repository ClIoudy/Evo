use crow::{Context, Texture, DrawConfig};
use ndarray::{Array2, Array1};
use std::time::SystemTime;
use crate::setup::{Game, rand_matrix};
use crate::Food;
use rand::prelude::*;
use std::f32::consts::PI;

use crate::game::{ENTITY_START_HUNGER, HIDDEN_NODES};

#[derive(Clone)]
pub struct Entity {

    pub pos: (i32, i32),
    pub f32pos: (f32, f32),
    pub hunger: f32,
    texture: Texture,

    pub rotation: f32,
    pub vision_range: f32,
    pub vision_angle: f32,
    pub vision_arcs: i32,
    pub weights: Vec<Array2<f32>>,
    pub max_speed: f32,
    pub max_rotation_speed: f32,
    pub creation_time: SystemTime,
    pub surival_time: u64,
    pub all_index: usize,
}



// functions for Entity: new, new from template, see, walk, drop

impl Entity {

    pub fn new(pos: (i32, i32), mut ctx: &mut Context, game: &mut Game) {
        let mut rng = thread_rng();

        
        // weights - Vector of Array2s 
        let mut weights:Vec<Array2<f32>> = vec![];

        let mut _a = vec![];
        let vision_arcs = rng.gen_range(2..9);
        // let vision_arcs = 3;
        _a.push(vision_arcs * 3);

        for i in 0..HIDDEN_NODES.len() {
            _a.push(HIDDEN_NODES[i]);
        }
        _a.push(2);
        // let layers = [vision_arcs * 3] + HIDDEN_NODES + [2];
        let layers = Array1::from_vec(_a);

        for i in 1..layers.len() {
            weights.push(rand_matrix::<f32>(layers[i-1] as usize, layers[i] as usize));
        }




        let e = Entity {
            pos,
            f32pos: (pos.0 as f32, pos.1 as f32),
            hunger: ENTITY_START_HUNGER,
            texture: Texture::load(&mut ctx, "./textures/Blob.png").unwrap(),
            rotation: 0.0,
            vision_range: rng.gen_range(100.0..500.0),
            vision_angle: rng.gen_range(5.0..40.0),
            vision_arcs,
            // weights: Array3::<f32>::default([1, 1, 1]),
            weights,
            max_speed: rng.gen_range(0.05..0.6),
            max_rotation_speed: rng.gen_range(0.05..0.4),
            creation_time: SystemTime::now(),
            surival_time: 0,
            all_index: game.all_entities_created.len(),
        };
        
        // make entities accessible to Game
        game.entities.push(e.clone());
        game.all_entities_created.push(e);
    }

    pub fn new_from_template(max_speed: f32, max_rotation_speed: f32, vision_range: f32, vision_angle: f32, vision_arcs: i32, weights: Vec<Array2<f32>>, ctx: &mut Context, game: &mut Game) {
        let mut rng = thread_rng();
        let mut _a = vec![];
        _a.push(vision_arcs * 3);
        for i in 0..HIDDEN_NODES.len() {
            _a.push(HIDDEN_NODES[i]);
        }
        _a.push(2);
        let layers = Array1::from_vec(_a);
        let pos = (rng.gen_range(300..700), rng.gen_range(200..300));
        let e = Entity {
            pos,
            f32pos: (pos.0 as f32, pos.1 as f32),
            hunger: ENTITY_START_HUNGER,
            texture: Texture::load(ctx, "./textures/template_blob.png").unwrap(),
            rotation: 0.0,
            vision_angle,
            vision_arcs,
            vision_range,
            weights,
            max_speed,
            max_rotation_speed,
            creation_time: SystemTime::now(),
            surival_time: 0,
            all_index: game.all_entities_created.len(),
        };
        game.entities.push(e.clone());
        game.all_entities_created.push(e);
    }


    pub fn drop(self, i: usize, game: &mut Game) {
        game.all_entities_created[self.all_index] = self.clone();
        game.entities.swap_remove(i);

        println!("delted entity ({})", self.all_index);
        
        if self.surival_time > game.all_entities_created[game.best_entity].surival_time {
            game.best_entity = self.all_index;
            println!("best entity died! - survival time: {}", self.surival_time);
        }
    }


    pub fn walk(&mut self, mut speed: f32, delta_time: f32, rotation: f32) {
        speed = speed * delta_time;
        let x = -f32::sin(self.rotation / 180.0 * PI);
        let y = f32::cos(self.rotation / 180.0 * PI);

        self.f32pos.0 += x * speed * self.max_speed;
        self.f32pos.1 += y * speed * self.max_speed;
        self.pos = (self.f32pos.0 as i32, self.f32pos.1 as i32);

        // self.rotation += f32::max(f32::min(rotation, self.max_rotation_speed), -self.max_rotation_speed);
        self.rotation += rotation * self.max_rotation_speed;
        self.hunger -= 1.5 + speed*4.0;
        self.surival_time = self.creation_time.elapsed().unwrap().as_secs();
    }


    

    pub fn see(&self, entities: &Vec<Entity>, foods: Vec<Food>, angle_offset: f32) -> (f32, f32, f32) 
    {

        let mut dist = self.vision_range;
        let mut angle:f32 = 0.0;

        let mut is:f32 = 0.0; // -1 for food, 0 for nothing, 1 for entity

        for entity in entities {
            
            let _x = self.f32pos.0 - entity.f32pos.0;
            let _y = self.f32pos.1 - entity.f32pos.1;
            let _dist: f32 = f32::sqrt(_x * _x + _y * _y);
            
            let _angle = self.util_see_angle( _x, _y, _dist, angle_offset);
            
            if _angle.abs() > self.vision_angle {
                continue;
            }

            //dist = f32::min(dist, dist1);
            if _dist < dist {
                dist = _dist;
                angle = _angle;
                is = 1.0;
            }
        }

        for food in foods {
            
            let _x = self.f32pos.0 - food.f32pos.0;
            let _y = self.f32pos.1 - food.f32pos.1;
            let _dist: f32 = f32::sqrt(_x * _x + _y * _y);
            
            let _angle = self.util_see_angle( _x, _y, _dist, angle_offset);
            
            if _angle.abs() > self.vision_angle {
                continue;
            }

            //dist = f32::min(dist, dist1);
            if _dist < dist {
                angle = _angle;
                dist = _dist;
                is = -1.0;
            }
        }               

        // println!("{}", self.rotation%360.0);
        // (dist, angle, is) !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        (dist/self.vision_range, angle/self.vision_angle, is)

    }

    pub fn draw(&mut self, ctx: &mut Context, mut surface: &mut crow::WindowSurface)  {
        ctx.draw(
            &mut surface, 
            &self.texture, 
            (self.pos.0 - 20, self.pos.1 - 20),
            &DrawConfig {
                rotation: self.rotation as i32,
                ..DrawConfig::default()
            }
        );
    }


    fn util_see_angle(&self, x: f32, y: f32, dist: f32, angle_offset: f32) -> f32 {

        let a = (x / dist, y / dist); 
        let b = (
            -f32::sin((self.rotation + angle_offset) / 180.0 * PI),
            f32::cos((self.rotation + angle_offset) / 180.0 * PI)
        );
    
        let dot = a.0 * b.0 + a.1 * b.1;
        let dot2 = a.0 * -b.1 + a.1 * b.0;
        let angle = f32::acos(-dot)/ PI * 180.0 * dot2.signum();
        // println!("{:.2}", angle);
        angle
    
    }

}