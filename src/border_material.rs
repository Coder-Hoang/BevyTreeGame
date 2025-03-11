use bevy :: {
    asset :: Asset,
    pbr :: {ExtendedMaterial, MaterialExtension},
    prelude :: *,
    render :: render_resource :: {AsBindGroup, ShaderRef},
};

pub struct BorderMaterialPlugin;

impl Plugin for BorderMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin :: <
            ExtendedMaterial<StandardMaterial, BorderMaterial>,
    > ::default())
    }
}

#[derive(Asset, AsBindGroup, Reflecct, Debug, Clone)]
pub struct BorderMaterial {
    #[uniform(100)]
    pub quantize_steps: u32,
    #[texture(101)]
    #[sampler(102)]
    pub color_texture: Handle<Image>,
}

impl MaterialExtension for BorderMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/border_material.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/border_material.wgsl".into()
    }
}