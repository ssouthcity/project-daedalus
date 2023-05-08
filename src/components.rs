use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::net;

#[derive(Component)]
pub enum Items {
    StaminaPotion,
    HealthPotion,
    ManaPotion,
}

impl From<&EntityInstance> for Items {
    fn from(entity_instance: &EntityInstance) -> Self {
        match entity_instance.get_enum_field("Item").unwrap().as_str() {
            "Stamina_Potion" => Self::StaminaPotion,
            "Health_Potion" => Self::HealthPotion,
            "Mana_Potion" => Self::ManaPotion,
            _ => unreachable!(),
        }
    }
}

#[derive(Bundle, LdtkEntity)]
pub struct ChestBundle {
    #[from_entity_instance]
    item: Items,
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}

#[derive(Component, Reflect, Default)]
pub struct Patrol(Vec<Vec3>);

impl Patrol {
    pub fn target(&self) -> &Vec3 {
        return self.0.first().unwrap();
    }

    pub fn next(&mut self) {
        self.0.rotate_left(1)
    }
}

impl From<&EntityInstance> for Patrol {
    fn from(entity_instance: &EntityInstance) -> Self {
        let mut patrol_path: Vec<Vec3> = entity_instance
            .iter_points_field("Patrol")
            .unwrap()
            .map(|point| {
                Vec3::new(
                    point.x as f32 * super::PPU,
                    (15 - point.y) as f32 * super::PPU,
                    0.0,
                )
            })
            .collect();

        patrol_path.push(Vec3::new(
            entity_instance.grid.x as f32 * super::PPU,
            (15 - entity_instance.grid.y) as f32 * super::PPU,
            0.0,
        ));

        Self(patrol_path)
    }
}

#[derive(Bundle, LdtkEntity)]
pub struct KnightBundle {
    #[from_entity_instance]
    patrol: Patrol,
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
    velocity: Velocity,
}

#[derive(Component, Deref, DerefMut, Default)]
pub struct Velocity(pub Vec2);

#[derive(Bundle, LdtkEntity)]
pub struct MinotaurBundle {
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
    velocity: Velocity,
    #[with(net::Player::from_entity)]
    character: net::Player,
}

#[derive(Component, Default)]
pub struct Wall;

#[derive(Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Bundle, LdtkEntity)]
pub struct IcarusBundle {
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
    velocity: Velocity,
    #[with(net::Player::from_entity)]
    character: net::Player,
}
