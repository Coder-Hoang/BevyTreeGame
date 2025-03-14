use bevy :: prelude :: *;
use bevy :: reflect :: TypePath;
use bevy :: render :: rander_resource :: {AsBindGroup, ShaderRef};

#[derive(AsBindGroup, Clone, TypePath, Asset)]

pub struct SpaceMaterial {
    #[uniform(0)]
    time: f32,

    #[texture(1, dimension = "2d")]
    #[sampler{2}]
    texture: Handle<Image>,

    #[texture(3, dimension = "2d")]
    #[sampler{4}]
    noise: Handle<Image>,

    alpha_mode: AlphaMode
}

impl Material for SpaceMaterial {
    fn fragment_shader() -> ShaderRef {
        "space.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

pub fn setup_space_bg (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<SpaceMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh :: from(shape :: Plane {
            size: 100.0,
            subdivisions: 10,
        })),
        transform: Transform :: from_xyz(0.0, -0.1, 0.0),
        material: materials.add(SpaceMaterial {
            textures: asset_server.load("textures/water.png"),
            noise: asset_server.load("textures/spacenoise.png"),
            time: 0.0,
            alpha_mode: AlphaMode :: Blend,
        }),
        ..Default :: default()
    });
}