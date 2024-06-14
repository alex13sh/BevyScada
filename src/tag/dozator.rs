use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use super::{Tag, TagExt, TagValue};
#[derive(Serialize, Deserialize)]
struct Dozator {
    speed: f32,
    direct: Direct,
}

#[derive(Serialize, Deserialize)]
enum Direct {
    // #[default]
    Forward, 
    Backward,
}

struct DozatorRefs {
    speed: Entity,
    direct: Entity,
}
impl TagExt for Dozator {
    type Refs = DozatorRefs;

}

fn system(dozators: Query<(&Tag<Dozator>, &TagValue)>) {
    if let Ok(doz) = dozators.get_single() {
        let value = &doz.0.value;
    }
}