use lib_simulation as sim;
use wasm_bindgen::prelude::*;

use crate::animal::Animal;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct World {
    pub animals: Vec<Animal>,
}

impl From<&sim::World> for World {
    fn from(world: &sim::World) -> Self {
        let animals = world.animals().iter().map(Animal::from).collect();

        Self { animals }
    }
}
