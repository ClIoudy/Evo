use crow::{Context};
use ndarray::{Array1};
use rand::{Rng};
use crate::setup::{Game, Entity::Entity, Food::Food, adjust_weights};


pub const HIDDEN_NODES: [i32; 3] = [
    10,
    10,
    6
];
    
pub const START_FOODS:u32 = 15;
pub const MAX_FOODS:usize = 60;
pub const STEP_SIZE:f32 = 0.2;
const ENTITY_SPAWN:usize = 6;
const FOOD_SPAWN:u32 = 10;
pub const ENTITY_START_HUNGER: f32 = 20000.0;
pub const FOOD_VAL:(f32, f32) = (6000.0, 7000.0);


pub fn update(mut ctx: &mut Context, game: &mut Game) {
    
    // Drawing Surface
    let mut surface = ctx.surface();
    ctx.clear_color(&mut surface, (0.4, 0.4, 0.8, 1.0)); 







    // ---------------------------------------  ENTITIES  ---------------------------------------

    let mut already_removed_entities = 0;

    for (i, entity) in game.entities.clone().iter_mut().enumerate() {

        // draw entities
        entity.draw(&mut ctx, &mut surface);
        

        // kill entities without sufficient food
        if entity.hunger < 0.0 {     
            entity.clone().drop(i - already_removed_entities, game);
            already_removed_entities += 1;
        }
        

        // let entities with enough food spawn offspring
        if entity.hunger > 1000000.0 {
            game.entities[i].hunger = entity.hunger - ENTITY_START_HUNGER;

            let mut e:Entity = game.entities[i].clone();

            e.pos = (entity.pos.0 - 150, entity.pos.1 - 150);
            e.rotation = entity.rotation - 70.0;
            for i in 0.. e.weights.len() {
                e.weights[i] = adjust_weights::<f32>(entity.weights[i].clone(), STEP_SIZE);
            }
            game.entities.push(e);
        }
    }

    

    // Entities NN logic

    for i in 0..game.entities.len() {
        let mut entities = game.entities.clone();
        let entity = entities[i].clone();
        entities.swap_remove(i);


        // inputs: entity.see()
        let mut inputs = Array1::<f32>::default((entity.vision_arcs * 3) as usize);

        for j in 0..entity.vision_arcs as usize {
            let k = j as f32 - (entity.vision_arcs as f32-1.0)/2.0;
            let sees = entity.see(&entities, game.foods.clone(), k * entity.vision_angle * 1.5);
            inputs[j*3 + 0] = sees.0;
            inputs[j*3 + 1] = sees.1;
            inputs[j*3 + 2] = sees.2;
        }


        
        // input -> hidden layers -> ouptput via matrix multiplication 
        let mut a = inputs;
        for j in 0..game.entities[i].weights.len() {
            a = game.entities[i].weights[j].dot(&a);
        }
        
        
        // output: speed and rotation, clamped 
        let speed = f32::max(f32::min(a[0], entity.max_speed), 0.0);
        let rotation = f32::max(f32::min(a[1], entity.max_rotation_speed), -entity.max_rotation_speed);
        
        game.entities[i].walk(speed, game.delta_time, rotation);

    }



    // always keeping entities on screen
    if game.entities.len() < ENTITY_SPAWN {
        let mut rng = rand::thread_rng();
        Entity::new((rng.gen_range(0..ctx.window_width()) as i32, rng.gen_range(0..ctx.window_height()) as i32 as i32), ctx, game);
    }










    // ---------------------------------------------  FOODS  ---------------------------------------------
    
    let mut already_eaten = 0;

    // kill eaten foods --> loop trough foods, loop trough entities, check for distances
    for (i, food) in &mut game.foods.clone().iter_mut().enumerate() {
        food.draw(&mut ctx, &mut surface);
        for entity in &mut game.entities {
            let x = entity.pos.0 - food.pos.0;
            let y = entity.pos.1 - food.pos.1;
            
            let dist = f32::sqrt((x*x + y*y) as f32);
            if dist <= 30.0 {
                game.foods.swap_remove(i - already_eaten);
                entity.hunger += food.val;
                already_eaten += 1;
                break;
            }
        }

    }    

    // spawn food every second 
    let food_secs = game.food_counter.elapsed().unwrap().as_secs();
    if food_secs > game.last_food && game.foods.len() < MAX_FOODS{
        let mut rng = rand::thread_rng();
        for _i in 0..FOOD_SPAWN {
            let f = Food::new((rng.gen_range(0..ctx.window_width()) as i32, rng.gen_range(0..ctx.window_height()) as i32 as i32), ctx);
            game.foods.push(f);
        }

        game.last_food = food_secs;
    }    


    // update delta_time
    game.update_time();
    
    // draw
    ctx.present(surface).unwrap();
}