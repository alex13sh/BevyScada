///! Входной / выходной физический или виртуальный канал
use bevy::prelude::*;
use serde::{Serialize, de::DeserializeOwned};
pub mod sensor;
pub mod dozator;

fn tag_plugin(app: &mut App) {
    app.add_systems(Update, sensor::check_bounds);
}

#[derive(Bundle)]
pub struct TagBundle<V: TagExt> {
    pub id: TagID,
    pub json_value: TagValue,
    pub setting: TagSetting,
    pub value: Tag<V>,
}

#[derive(Component, Debug, PartialEq)]
pub struct TagID(pub String);

impl From<&'static str> for TagID {
    fn from(id: &'static str) -> Self {
        TagID(String::from(id))
    }
}

#[derive(Component)]
pub struct Tag<V: TagExt> {
    refs: V::Refs,
    value: V,
}

pub trait TagExt: Serialize + DeserializeOwned + Sync + Send + 'static {
    type Refs: Sync + Send;
    // type Error;
}

impl TagExt for () {
    type Refs = ();
}
impl From<()> for Tag<()> {
    fn from(_:()) -> Self {
        Tag {
            value: (),
            refs: (),
        }
    }
}

#[derive(Component, Default)]
pub struct TagSetting {
    /// Если тэг только на чтение, то в него нельзя записывать
    read_only: bool,
    /// Тэг можно выключить если он не нужен
    enabled: bool,
}

#[derive(Component, Default, Deref, derive_more::From)]
pub struct TagValue(pub serde_json::Value);

impl <V: TagExt> TagBundle<V> {
    pub fn value(&self) -> &serde_json::Value {
        &self.json_value
    }
}


#[test]
fn test_tag() {

    // Setup app
    let mut app = App::new();

    let id = app.world
        .spawn(TagBundle {
            id: "my_id".into(),
            json_value: TagValue(serde_json::Value::Null),
            setting: TagSetting::default(),
            value: ().into(),
        }).id();

    // Run systems
    app.update();

    // Check resulting changes
    assert!(app.world.get::<TagID>(id).is_some());
    assert_eq!(*app.world.get::<TagID>(id).unwrap(), "my_id".into());
}
