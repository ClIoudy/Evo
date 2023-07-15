use crow::Context;
use crate::setup::Game;

pub const HIDDEN_NODES: [i32; 5] = [
    10,
    10,
    10,
    10,
    10,
];


pub const START_FOODS:u32 = 20;
const FOOD_SPAWN:u32 = 10;
pub const MAX_FOODS:usize = 60;
pub const FOOD_VAL:(f32, f32) = (6000.0, 7000.0);

pub const STEP_SIZE:f32 = 0.2;

const ENTITY_SPAWN:usize = 11;
pub const ENTITY_START_HUNGER: f32 = 20000.0;





pub fn update(mut ctx: &mut Context, game: &mut Game) {
    
    // Drawing Surface
    let mut surface = ctx.surface();
    ctx.clear_color(&mut surface, (0.4, 0.4, 0.8, 1.0)); 


    
    // ---------------------------------------  ENTITIES  ---------------------------------------

    let mut to_remove = vec![];
    for (i, entity) in game.entities.clone().iter_mut().enumerate() {
        
        entity.draw(&mut ctx, &mut surface);
        
        let mut entities = game.entities.clone();
        entities.swap_remove(i);
        game.entities[i].nn_logic(entities, &game.foods, game.delta_time);

        // let entities with enough food spawn offspring
        if entity.hunger > 1500000.0 {
            game.entities[i].hunger = entity.hunger - ENTITY_START_HUNGER;
            game.offspring_of(entity, STEP_SIZE as f32);
        }

        // kill entities without sufficient food
        if entity.hunger < 0.0 { 
            to_remove.push(i);            
        }     
    }

    to_remove.reverse();
    for i in to_remove {
        game.kill(i);
    }

    // spawn entities
    game.spaw_entities(ENTITY_SPAWN as usize, ctx);



    // ---------------------------------------------  FOODS  ---------------------------------------------

    // kill eaten foods
    let mut to_remove:Vec<usize> = vec![];

    for (i, food) in &mut game.foods.clone().iter_mut().enumerate() {
        food.draw(&mut ctx, &mut surface);
        if game.entities_eat(food, i) {
            to_remove.push(i)
        }
    }

    to_remove.reverse();
    for i in to_remove {
        game.foods.swap_remove(i);
    }

    // spawn food every second 
    game.spawn_foods(ctx, FOOD_SPAWN as u32, MAX_FOODS as u32); 


    // update delta_time
    game.update_time();
    
    // draw
    ctx.present(surface).unwrap();
}