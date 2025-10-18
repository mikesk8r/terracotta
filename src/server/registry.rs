// see https://minecraft.wiki/Java_Edition_protocol/Registry_data#Dimension_Type
#[derive(serde::Serialize)]
pub struct DimensionType {
    // TODO: add fixed_time
    pub has_skylight: bool,
    pub has_ceiling: bool,
    pub ultrawarm: bool,
    pub natural: bool,
    pub coordinate_scale: f64,
    pub bed_works: bool,
    pub respawn_anchor_works: bool,
    pub min_y: i8,
    pub height: i8,
    pub logical_height: i8,
    pub infiniburn: String,
    pub effects: String,
    pub ambient_light: f32,
    pub piglin_safe: bool,
    pub has_raids: bool,
    // TODO: add support for compound tags here
    pub monster_spawn_light_level: i8,
    pub monster_spawn_block_light_limit: i8,
}

#[derive(serde::Serialize)]
pub struct Dimension {
    pub name: String,
    pub id: i32,
    #[serde(rename = "element")]
    pub entry: DimensionType,
}

#[derive(serde::Serialize)]
pub struct DimensionTypeRegistry {
    #[serde(rename = "type")]
    pub registry_type: String,
    #[serde(rename = "value")]
    pub dimensions: Vec<Dimension>,
}
