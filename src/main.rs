//!!!!!! directory to execute should be setup
#![windows_subsystem = "console"]
mod update;
mod setup;


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

    

    game.load_entities_from_templates(&mut ctx, "0");


    event_loop.run(
        move |event: Event<()>, _window_target: _, control_flow: &mut ControlFlow| match event {


            // Handle Inputs such as window closed, mouse pressed, etc... 
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    game.close_prints();

                    *control_flow = ControlFlow::Exit
                },
                WindowEvent::CursorMoved { position, .. } => {
                    // update cursor position used for spwaning entities or food
                    game.mouse_position = position.into();
                    game.mouse_position.1 = ctx.window_height() as i32 - game.mouse_position.1; 
                }
                WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, ..} => {
                    // spawn new Entity
                    Entity::new(game.mouse_position, &mut ctx, &mut game);
                }                
                WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Right, ..} => {
                    // spawn new food
                    game.foods.push(Food::new(game.mouse_position, &mut ctx));
                }
                WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Middle, ..} => {
                    // kill all current entities
                    game.foods = vec![];
                    for _i in 0..game.entities.len() {   
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