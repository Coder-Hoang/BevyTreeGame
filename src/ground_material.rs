use bevy :: {
    asset :: Asset,
    pbr :: {ExtendedMaterial, MaterialExtension},
    prelude :: *,
    render :: render_resource :: {AsBindGroup, ShaderRef},
};

pub struct GroundMaterialPlugin;

impl Plugin for GroundMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin ::<
            ExtendedMaterial<StandardMaterial, GroundMaterial>,
        > :: default());
    }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct GroundMaterial {
    #[uniform(100)]
    pub scale: f32,
    #[uniform(100)]
    pub noise_scale: f32,
    #[texture(110)]
    #[sampler(111)]
    pub color_textures: Handle<Image>,
}

impl MaterialExtension for GroundMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/ground_material.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/ground_material.wgsl".into()
    }
}