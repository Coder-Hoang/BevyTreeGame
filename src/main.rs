use bevy::{
    audio::{Volume, VolumeLevel}, ecs::system::Spawn, math::vec3, prelude::*
};
use bevy_rapier2d::plugin::RapierPhysicsPlugin;
use bevy_rapier3d::prelude::*;
use bevy_vector_shapes::ShapePlugin;
use no_communication_0::{
    animation_linker::AnimationEntityLinkPlugin,
    background::{setup_space_bg, SpaceMaterial},
    border_material::BorderMaterialPlugin,
    camera::{CameraPlugin, DollyCamera, FollowPlayerCamera, MainCameraTag},
    foliage::FoliagePlugin,
    ground_material::GroundMaterialPlugin,
    health::HealthPlugin,
    inventory::{InventoryPlugin, Item},
    item_pickups::ItemPickupPlugin,
    knockback::KnockbackPlugin,
    map::{MapPlugin, MAP_SIZE_HALF},
    notification::{NotificationEvent, NotificationPlugin},
    pickup::PickupPlugin,
    player::{Body, PlayerPlugin, SpawnPlayerEvent},
    pointer::PointerPlugin,
    projectile::ProjectilePlugin,
    shop::{ShopItemData, ShopItemEffect, ShopPlugin, SpawnShopItemEvent},
    state::{AppState, StatePlugin},
    tower::TowerPlugin,
    tree::{TreePlugin, TriggerSpawnTrees},
    tree_spawner::TreeSpawnerPlugin,
    ui_util::UiUtilPlugin,
    waves::WavePlugin,
    weapon::{AxeSfxCooldownTimer, ProjSfxCooldownTimer, WeaponPlugin, WeaponType},
};
use rand::Rng;
use tree_game::ground_material::GroundMaterial;

// we will fix any error that we occur later

fn main() {
    App :: new ()
        .add_plugins ((
            DefaultPlugins,
            RapierPhysicsPlugin,
            ShapePlugin :: default()
        ))
        .add_plugins ((
            (BorderMaterialPlugin, GroundMaterial),
            (
                UiUtilPlugin,
                CameraPlugin,
                PlayerPlugin,
                WeaponPlugin,
                PickupPlugin,
                HealthPlugin,
                ItemPickupPlugin,
                ProjectilePlugin,
                InventoryPlugin,
                ProjectilePlugin,
                InvetoryPlugin,
                ShopPlugin,
                PointerPlugin,
                MapPlugin,
                NotificationPlugin,
            ),
            (
                TowerPlugin,
                WavePlugin,
                StatePlugin,
                AnimationEntityLinkPlugin,
                KnockbackPlugin,
                TreeSpawnerPlugin,
                MaterialPlugin :: <SpaceMaterial> :: default(),
            ),
        ))
        .add_systems(Startup, (setup, setup_space_bg))
        .run();
}

fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut spawn_player_event: EventWritter<SpawnShopEvent>,
    mut spawn_shop_item_event: EventWritter<SpawnShopItemEvent>,
    mut notification_event: EvenyWritter<NotificationEvent>,
    mut tree_trigger_writer: EventWritter<TriggerSpawnTrees>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(AudioBundle {
        source: asset_server.oad("sounds/8bit-space-shooter.ogg"),
        settings: PlaybackSettings :: LOOP.with_volume(Volume :: Absoluite(VolumeLevel :: new(0.3))),
    });
    tree_trigger_writter.send(TriggerSpawnTrees(0.1));

    rapier_config.gravity = Vec3 :: NEG_Y * 100.0;

    let mut rng = rand :: thread_rng();
    spawn_player_event.send(SpawnPlayerEvent {
        pos : vec3(
            rng.gen_range(-MAP_SIZE_HALF..MAP_SIZE_HALF),
            1.0,
            rng.gen_range(-MAP_SIZE_HALF..MAP_SIZE_HALF),
            ),
            is_main: true,
            body: Body :: Monkey,
            weapon_type: WeaponType :: Bow(asset_server.load("projectiles/bow.projectile.ron")),
        });
        let mut x = MAP_SIZE_HALF + rng.gen_range(10.0..20.0);
        let mut z = MAP_SIZE_HALF + rng.gen_range(10.0..20.0);
        x *= match rng.gen :: <bool>() {
            true => 1.0,
            false => -1.0,
        };

        z *= match rng_gen :: <bool>() {
            true => 1.0,
            false => -1.0,
        };

        spawn_player_event.send(SpawnPlayerEvent {
            pos : vec3(x, 4.0, z),
            is_main: false,
            body: Body :: Robot,
            weapon_type: WeaponType :: Axe,
        });

        {
            spawn_shop_item_event.send(SpawnShopItemEvent {
                item: ShopItemData {
                    cost: vec![(Item :: Log, 1)],
                    effects: vec![(ShopItemEffect :: PlatTree)],
                    permanent: true,
                },
            });
            spawn_shop_item_event.send(SpawnShopItemEvent {
                item: ShopItemData {
                    coat: vec![(Item :: Apple, 2)],
                    effects: vec![(ShopItemEffect :: Heal(10))],
                    permanent : true,
                },
            });
        }
        commands.insert_resource(AmbientLight {
            brightness: 1.0,
            ..default()
        });
        commands.spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 30000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform :: from_xyz(-2.0, 18.5, 25.0).looking_at(Vec3 :: ZERO, Vec3 :: Y),
            ..default()
        });
        let transform = Transform :: from_xyz(-2.0, 18.5, 25.0).looking_at(Vec3 :: ZERO, Vec3 :: Y);
        let pos = transform.translation;
        let rotation = transform.rotation;

        commands.insert_resource(AppState :: Wave(0));
        commands.insert_resource(AxeSfxCooldownTimer(0.0));
        commands.insert_resource(ProjSfxCooldownTimer(0.0));

        commands.spawn ((
            MainCameraTag,
            FollowPlyerCamera,
            DollyyCamera :: new(pos, rotation, 10.0),
            Camera3dBundle {
                transform,
                ..default()
            },
        ));

        notification_event.send(NotificationEvent {
            text: "Protect The Tree!".into(),
            show_for: 7.0,
            color: Color :: WHITE,
        });
        notification_event.send(NotifinationEvent {
            text: "Wave 1!".into(),
            show_for: 3.0,
            color: Color :: BLUE,
        });
}