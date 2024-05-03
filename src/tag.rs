///! Входной / выходной физический или виртуальный канал

pub struct Tag {
    pub id: String,
    value: TagValue,
}

pub struct TagSetting {
    /// Если тэг только на чтение, то в него нельзя записывать
    read_only: bool,
    /// Тэг можно выключить если он не нужен
    enabled: bool,
}

pub type TagValue = serde_json::Value;

impl Tag {
    pub fn value(&self) -> &TagValue {
        &self.value
    }
}
