use lib_simulation as sim;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Animal {
    pub x: f32,
    pub y: f32,
}
// ^ This model is smaller than `lib_simulation::Animal` - that's
// | because a bird's position is all we need on the JavaScript's
// | side at the moment; there's no need to map rest of the fields.

impl From<&sim::Animal> for Animal {
    fn from(animal: &sim::Animal) -> Self {
        Self {
            x: animal.position().x,
            y: animal.position().y,
        }
    }
}
