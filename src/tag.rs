///! Входной / выходной физический или виртуальный канал
use bevy::prelude::*;

pub mod sensor;

fn tag_plugin(app: &mut App) {
    app.add_systems(Update, sensor::check_bounds);
}

#[derive(Bundle)]
pub struct Tag {
    pub id: TagID,
    pub value: TagValue,
    pub setting: TagSetting
}

#[derive(Component, Debug, PartialEq)]
pub struct TagID(pub String);

impl From<&'static str> for TagID {
    fn from(id: &'static str) -> Self {
        TagID(String::from(id))
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

impl Tag {
    pub fn value(&self) -> &serde_json::Value {
        &self.value
    }
}


#[test]
fn test_tag() {

    // Setup app
    let mut app = App::new();

    let id = app.world
        .spawn(Tag {
            id: "my_id".into(),
            value: TagValue(serde_json::Value::Null),
            setting: TagSetting::default(),
        }).id();

    // Run systems
    app.update();

    // Check resulting changes
    assert!(app.world.get::<TagID>(id).is_some());
    assert_eq!(*app.world.get::<TagID>(id).unwrap(), "my_id".into());
}
