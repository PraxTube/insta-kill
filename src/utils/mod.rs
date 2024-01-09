use bevy::prelude::*;
use bevy_rapier2d::{plugin::RapierTransformPropagateSet, prelude::*};

pub const COLLISION_GROUPS_NONE: CollisionGroups = CollisionGroups::new(Group::NONE, Group::NONE);

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            reset_rotations.before(RapierTransformPropagateSet),
        );
    }
}

#[allow(dead_code)]
pub fn quat_from_vec2(direction: Vec2) -> Quat {
    if direction == Vec2::ZERO {
        return Quat::IDENTITY;
    }
    Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, Vec2::X.angle_between(direction))
}

#[allow(dead_code)]
pub fn quat_from_vec3(direction: Vec3) -> Quat {
    quat_from_vec2(direction.truncate())
}

#[derive(Component, Default)]
pub struct FixedRotation {
    pub offset: Vec3,
    pub rot: Quat,
}

/// there is no way to inherit position but not rotation from the parent entity transform yet
/// see: https://github.com/bevyengine/bevy/issues/1780
fn reset_rotations(
    mut q_transforms: Query<(&Parent, &mut Transform, &FixedRotation)>,
    q_parents: Query<&Transform, Without<FixedRotation>>,
) {
    for (parent, mut transform, fixed_rotation) in q_transforms.iter_mut() {
        if let Ok(parent_transform) = q_parents.get(parent.get()) {
            let rot_inv = parent_transform.rotation.inverse();
            let rot = Quat::from_rotation_z(
                rot_inv.to_euler(EulerRot::ZYX).0 + fixed_rotation.rot.to_euler(EulerRot::ZYX).0,
            );

            transform.rotation = rot;
            transform.translation = rot_inv.mul_vec3(fixed_rotation.offset);
        }
    }
}

pub fn format_time(seconds: f32) -> String {
    let minutes = (seconds / 60.0) as u32;
    let seconds = (seconds % 60.0) as u32;

    format!("{:02}:{:02}", minutes, seconds)
}
