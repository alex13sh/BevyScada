use bevy::prelude::*;

#[derive(Bound)]
pub struct Sensor {
    // tag: Tag read only
    bounds: SensorBounds,
}

enum BoundError {
    WarnLevel,
    ErrorLevel,
}

enum SensorError {
    LowBound(BoundError),
    TopBound(BoundError),
}

pub struct BoundLevel {
    warn: f32,
    error: f32,
}

pub struct SensorBounds {
    /// Ограничение сверху
    top: Option<BoundLevel>,
    /// Ограничение снизу
    low: Option<BoundLevel>,
}

impl SensorBounds {
    pub fn check_value(&self, value: f32) -> Option<SensorError> {
        if let Some(bound) = self.top {
            if bound.error <= value {
                return Some(SensorError::TopBound(BoundError::ErrorLevel)));
            } else if bound.warn <= value {
                return Some(SensorError::TopBound(BoundError::WarnLevel)));
            }
        }

        if let Some(bound) = self.low {
            if bound.error >= value {
                return Some(SensorError::LowBound(BoundError::ErrorLevel)));
            } else if bound.warn >= value {
                return Some(SensorError::LowBound(BoundError::WarnLevel)));
            }
        }
        None
    }
}
