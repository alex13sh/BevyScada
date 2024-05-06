use bevy::prelude::*;
use super::{Tag, TagValue};

#[derive(Bundle)]
pub struct Sensor {
    tag: Tag,
    bounds: SensorBounds,
    status: SensorStatus,
}

#[derive(Debug, PartialEq)]
enum BoundError {
    WarnLevel,
    ErrorLevel,
}

#[derive(Debug, PartialEq)]
enum SensorError {
    LowBound(BoundError),
    TopBound(BoundError),
}

pub struct BoundLevel {
    warn: f32,
    error: f32,
}

#[derive(Component, Default)]
pub struct SensorBounds {
    /// Ограничение сверху
    top: Option<BoundLevel>,
    /// Ограничение снизу
    low: Option<BoundLevel>,
}

#[derive(Component, Default, Debug, PartialEq)]
pub struct SensorStatus {
    error: Option<SensorError>,
}

impl SensorBounds {
    pub fn check_value(&self, value: f32) -> Option<SensorError> {
        if let Some(bound) = &self.top {
            if bound.error <= value {
                return Some(SensorError::TopBound(BoundError::ErrorLevel));
            } else if bound.warn <= value {
                return Some(SensorError::TopBound(BoundError::WarnLevel));
            }
        }

        if let Some(bound) = &self.low {
            if bound.error >= value {
                return Some(SensorError::LowBound(BoundError::ErrorLevel));
            } else if bound.warn >= value {
                return Some(SensorError::LowBound(BoundError::WarnLevel));
            }
        }
        None
    }
}

pub(super) fn check_bounds(mut sensors: Query<(&TagValue, &SensorBounds, &mut SensorStatus)>) {
    let mut itr = sensors.iter_mut()
        .filter_map(|(v,b,s)|
            Some((v.as_number()?.as_f64()?, b, s))
        );
    for (value, bounds, mut status) in itr {
        status.error = bounds.check_value(value as f32);
    }
}

#[test]
fn test_sensor_bound() {
    let bounds = SensorBounds {
        top: Some(BoundLevel {
            warn: 60.0,
            error: 80.0,
        }),
        low: None,
    };

    assert_eq!(bounds.check_value(55.0), None);
    assert_eq!(bounds.check_value(65.0), Some(SensorError::TopBound(BoundError::WarnLevel)));
    assert_eq!(bounds.check_value(85.0), Some(SensorError::TopBound(BoundError::ErrorLevel)));

    // Setup app
    let mut app = App::new();

    let id = app.world
        .spawn(Sensor {
            tag: Tag {
                id: "my_id".into(),
                value: serde_json::json!{55.0}.into(),
                setting: Default::default(),
            },
            bounds: bounds,
            status: default(),
        }).id();

    app.add_systems(Update, (check_bounds));

    // Run systems
    app.update();

    // Check resulting changes
    assert!(app.world.get::<SensorBounds>(id).is_some());
    assert_eq!(app.world.get::<SensorStatus>(id).unwrap().error, None);

    *app.world.get_mut::<TagValue>(id).unwrap() = serde_json::json!{65.0}.into();
    app.update();
    assert_eq!(app.world.get::<SensorStatus>(id).unwrap().error, Some(SensorError::TopBound(BoundError::WarnLevel)));

    *app.world.get_mut::<TagValue>(id).unwrap() = serde_json::json!{85.0}.into();
    app.update();
    assert_eq!(app.world.get::<SensorStatus>(id).unwrap().error, Some(SensorError::TopBound(BoundError::ErrorLevel)));
}
