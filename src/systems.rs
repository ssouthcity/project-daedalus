use crate::{
    components::{Items, Patrol, Velocity, Wall},
    net,
};
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use bevy_ggrs::{PlayerInputs, Rollback};

pub fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity), Without<Wall>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

pub fn undo_collisions(
    mut query: Query<&mut Transform, (With<Velocity>, Without<Wall>)>,
    wall_query: Query<&Transform, With<Wall>>,
) {
    for mut transform in query.iter_mut() {
        for wall_transform in wall_query.iter() {
            let wall_center = wall_transform.translation + Vec3::splat(super::PPU / 2.0);

            match collide(
                wall_center,
                Vec2::splat(super::PPU),
                transform.translation,
                Vec2::splat(super::PPU),
            ) {
                Some(Collision::Left) => transform.translation.x = wall_center.x + super::PPU,
                Some(Collision::Right) => transform.translation.x = wall_center.x - super::PPU,
                Some(Collision::Top) => transform.translation.y = wall_center.y - super::PPU,
                Some(Collision::Bottom) => transform.translation.y = wall_center.y + super::PPU,
                _ => continue,
            }
        }
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

pub fn apply_input(
    mut query: Query<(&mut Velocity, &net::Player)>,
    inputs: Res<PlayerInputs<net::GGRSConfig>>,
) {
    let speed: f32 = 64.0;

    for (mut velocity, player) in query.iter_mut() {
        let (input, _) = inputs[player.handle];

        let mut new_velocity = Vec2::ZERO;

        if input & net::INPUT_UP != 0 {
            new_velocity.y += 1.0;
        }

        if input & net::INPUT_DOWN != 0 {
            new_velocity.y -= 1.0;
        }

        if input & net::INPUT_LEFT != 0 {
            new_velocity.x -= 1.0;
        }

        if input & net::INPUT_RIGHT != 0 {
            new_velocity.x += 1.0;
        }

        new_velocity = new_velocity.normalize_or_zero() * speed;

        velocity.x = new_velocity.x;
        velocity.y = new_velocity.y;
    }
}

// pub fn camera_follow_player(
//     player_query: Query<&Transform, With<Player>>,
//     mut camera_query: Query<&mut Transform, (With<OrthographicProjection>, Without<Player>)>,
// ) {
//     for player_pos in player_query.iter() {
//         let mut camera_pos = camera_query.single_mut();

//         camera_pos.translation.x = player_pos.translation.x;
//         camera_pos.translation.y = player_pos.translation.y;
//     }
// }
