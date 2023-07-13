//!!!!!! directory to execute should be setup


#![windows_subsystem = "console"]
mod update;
mod setup;
mod templates;

use crow::{
    glutin::{
        event::*,
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    Context
};
use update::START_FOODS;
use rand::{thread_rng, Rng};

use crate::setup::{food::Food, entity::Entity};

fn main() -> Result<(), crow::Error> {
    
    let event_loop = EventLoop::new();
    let mut ctx = Context::new(WindowBuilder::new(), &event_loop)?;
    ctx.window().set_maximized(true);
    let mut game = setup::Game::new(); 
    for _i in 0..START_FOODS {
        let mut rng = thread_rng();
        let f = Food::new((rng.gen_range(0..ctx.window_width() as i32), rng.gen_range(0..ctx.window_height() as i32)), &mut ctx);
        game.foods.push(f);
    }

    

    templates::create_from_template(&mut ctx, &mut game);


    event_loop.run(
        move |event: Event<()>, _window_target: _, control_flow: &mut ControlFlow| match event {


            // Handle Inputs such as window closed, mouse pressed, etc... 
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    let mut survival_times: Vec<(u64, usize)> = vec![];
                    for (i, entity) in game.all_entities_created.iter().enumerate() {
                        survival_times.push((entity.surival_time, i));

                    }
                    survival_times.sort_unstable();
                    println!("{:?}", survival_times);
                    println!();
                    println!();

                    for i in 0..survival_times.len() {
                        println!("{}/{}", survival_times.len() - i, survival_times.len());
                        let mut entity = game.all_entities_created[survival_times[i].1].clone();
                        entity.weights.pop();
                        println!("index: {}", entity.all_index);
                        println!("survival time: {}", entity.surival_time);
                        println!("speed: {}", entity.max_speed);
                        println!("rotation: {}", entity.max_rotation_speed);
                        println!("vision: \n    range: {}, \n    angle: {} \n    arcs:  {}", entity.vision_range, entity.vision_angle, entity.vision_arcs);
                        println!();
                        println!();

                        println!("Entity::new_from_template({}, {}, {}, {}, {} \n   vec![", entity.max_speed, entity.max_rotation_speed, entity.vision_range, entity.vision_angle, entity.vision_arcs);
                        for j in 0..entity.weights.len() {
                            println!("arr2(&{}),", entity.weights[j]);
                        }
                        println!("], \n   &mut ctx, &mut game \n);");
                        println!();
                        println!();
                        println!();
                    }

                    *control_flow = ControlFlow::Exit
                },
                WindowEvent::CursorMoved { position, .. } => {
                    game.mouse_position = position.into();
                    game.mouse_position.1 = ctx.window_height() as i32 - game.mouse_position.1; 
                }
                WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, ..} => {
                    Entity::new(game.mouse_position, &mut ctx, &mut game);

                }                
                WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Right, ..} => {
                    game.foods.push(Food::new(game.mouse_position, &mut ctx));
                }
                WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Middle, ..} => {
                    //game.entities.dr
                    game.foods = vec![];
                    for _i in 0..game.entities.len() {   
                        // println!("delted entity ({})", game.entities[0].all_index);
                        // game.entities[0].clone().drop(0, &mut game)
                        game.kill(0);
                    }
                }
                _ => (),
            },

            Event::MainEventsCleared => ctx.window().request_redraw(),
            Event::RedrawRequested(_) => {
                update::update(&mut ctx, &mut game);
            }
            _ => (),
        },
    )
}