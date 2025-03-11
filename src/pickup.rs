use bevy :: prelude :: *;
use bevy_rapier3d :: prelude :: *;

use crate :: {inventory :: Item, item_picup :: SpawnItemEvent};
pub const PICKUP_FLY_SPEED: f32 = 10.0;
pub const TIME_TO_FLY: f32 = 0.4;

#[derive(Component)]
pub struct PickupMagnet { // :)
    pub root_entity: Entity,
}

#[derive(Component)]
pub struct PickupTag;

#[derive(Component)]
pub struct FlyToEntity {
    pub entity: Entity,
    pub initial_pos: Vec3,
    pub progress: f32,
}

#[derive(Component)]
pub struct FlyToEntity {
    pub entity: Entity,
    pub initial_pos: Vec3,
    pub progress: f32,
}

#[derive(Event)]
pub struct OnPickedUpEvent {
    pub pickup_entity: Entity,
    pub receiver_entity: Entity,
}

pub struct PickupPlugin;

impl Plugin for PickupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (detect_pickup, fly_to_target))
            .add_systems(Last, destroy_pickups);
    }
}

fn fly_to_target(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut FlyToEntity)>,
    transform: Query<&GlobalTransform>,
    time: Res<Time>,
    mut pickup_event: EventWritter<OnPickedUpEvent>,
    mut spawn_item_event: EventWritter<SpawnItemEvent>,
) {
    for (pickup_entity, mut transform, mut fly_to_entity) in query.iter_mut() {
        let Ok(target_transform) = transform.get(fly_to_entity.entity) else{
            comands.entity(pickup_entity).despawn_recursive();
            spawn_item_event.send(SpawnItemEvent {
                item: Item :: Log,
                pos: transform.translation,
            });

            continue;
        };
        fly_to_entity.progress += time.delta_seconds();
        let percent = (fly_to_entity.process / TIME_TO_FLY).clamp(0.0, 1.0);
        let mut lerped = fly_to_entity
            .initial_pos
            .lerp(target_transform.translation(), percent);

        let jump = percent - (-0.5 + percent * 2.0).max(0.0);
        lerped.y += jump * 3.0;
        transform.translation = lerped;
        
        if percent >= 1.0 {
            pickup_event.send(OnPickUpEvent {
                pickup_entity,
                receiver_entity: fly_to_entity.entity,
            });
        }
    }
}

fn detect_pickup(
    mut events: EventReader<CollisionEvent>,
    pickup_magnet: Query<&PickupMagnet>,
    pickups: Query<(&Entity, &GlobalTransform), With<PickupTag>>,
    mut commands: Commamds
) {
    for event in events.read() {
        let CollisionEvent :: Started(e1, e2, _event_flags) = event else {
            continue;
        };

        let(magnet, (pickup_entity, pickup_transform)) = match (
            pickup_magnent.get(*e1),
            pickups.get(*e2),
            pickup_magnets.get(*e2),
            pickups.get(*e1),
        ) {
            (Ok(m), Ok(p), Err(_), Err(_)) => (m,  p),
            (Err(_), Err(_), Ok(m), Ok(p)) => (m,p),
            _ => continue, 
        };
        commands.entity(pickup_entity)
            .insert(Fly_To_Entity {
                entity: magnent.root_entity,
                initial_pos: pickup_transform. translation(),
                progress: 0.0,
            })
            .remove :: <RigidBody>()
            .remove :: <Collider>()
            .remove :: <PickupTag>();
    }
}