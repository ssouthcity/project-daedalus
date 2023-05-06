use crate::components::{Items, Patrol, Velocity};
use bevy::prelude::*;

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

pub fn item_glow(mut query: Query<(&mut TextureAtlasSprite, &Items)>, time: Res<Time>) {
    let alpha = time.elapsed_seconds().sin();

    for (mut sprite, item) in query.iter_mut() {
        let mut color = match item {
            Items::HealthPotion => Color::RED,
            Items::ManaPotion => Color::BLUE,
            Items::StaminaPotion => Color::GREEN,
        };

        color.set_a(alpha.max(0.5));

        sprite.color = color
    }
}

pub fn update_patrol_target(mut query: Query<(&Transform, &mut Patrol)>) {
    for (trans, mut patrol) in query.iter_mut() {
        let target = patrol.target();
        let has_reached_target = (trans.translation.x - target.x - super::PPU / 2.0).abs() < 1.0
            && (trans.translation.y - target.y - super::PPU / 2.0).abs() < 1.0;

        if has_reached_target {
            patrol.next();
        }
    }
}

pub fn update_patrol_velocity(mut query: Query<(&mut Velocity, &Transform, &Patrol)>) {
    for (mut velocity, transform, patrol) in query.iter_mut() {
        let target = patrol.target();

        let direction = Vec2::new(
            target.x - transform.translation.x + super::PPU / 2.0,
            target.y - transform.translation.y + super::PPU / 2.0,
        );

        velocity.0 = direction.normalize() * super::PPU * 4.0;
    }
}
