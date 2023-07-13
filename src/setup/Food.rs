use crow::{Context, Texture, DrawConfig};
use crate::update::FOOD_VAL;
use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct Food {
    pub pos: (i32, i32),
    pub f32pos: (f32, f32),
    texture: Texture,
    pub val: f32,
}

impl Food {

    pub fn new(pos:(i32, i32), ctx: &mut Context) -> Food{
        let mut rng = thread_rng();
        let texture = Texture::load(ctx, "./textures/Food.png").unwrap();
        Food{ 
            pos, 
            f32pos: (pos.0 as f32, pos.1 as f32), 
            texture, 
            val: rng.gen_range(FOOD_VAL.0..FOOD_VAL.1),
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, mut surface: &mut crow::WindowSurface)  {
        self.pos = (self.f32pos.0 as i32, self.f32pos.1 as i32);
        ctx.draw(&mut surface, &self.texture, (self.pos.0 - 20, self.pos.1 - 20), &DrawConfig::default());
    }
}