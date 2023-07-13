use crow::Context;
use rand::Rng;
use crate::setup::{Game, entity::Entity, food::Food};


pub const HIDDEN_NODES: [i32; 5] = [
    10,
    10,
    10,
    10,
    10
];
    
pub const START_FOODS:u32 = 15;
pub const MAX_FOODS:usize = 60;
pub const STEP_SIZE:f32 = 0.2;
const ENTITY_SPAWN:usize = 7;
const FOOD_SPAWN:u32 = 10;
pub const ENTITY_START_HUNGER: f32 = 20000.0;
pub const FOOD_VAL:(f32, f32) = (6000.0, 7000.0);


pub fn update(mut ctx: &mut Context, game: &mut Game) {
    
    // Drawing Surface
    let mut surface = ctx.surface();
    ctx.clear_color(&mut surface, (0.4, 0.4, 0.8, 1.0)); 







    // ---------------------------------------  ENTITIES  ---------------------------------------

    let mut to_remove = vec![];
    for (i, entity) in game.entities.clone().iter_mut().enumerate() {
        
        if entity.hunger < 0.0 { 
            to_remove.push(i);            
        }
        // draw entities
        entity.draw(&mut ctx, &mut surface);
        
        let mut entities = game.entities.clone();
        entities.swap_remove(i);
        game.entities[i].nn_logic(entities, &game.foods, game.delta_time);

        // let entities with enough food spawn offspring
        if entity.hunger > 1500000.0 {
            game.entities[i].hunger = entity.hunger - ENTITY_START_HUNGER;
            game.offspring_of(entity);
        }


        
    }

    // kill entities without sufficient food
    to_remove.reverse();
    for i in to_remove {
        game.kill(i);
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