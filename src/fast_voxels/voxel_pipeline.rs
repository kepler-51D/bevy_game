/*
- receive camera
- receive list of VoxelMeshes
- for each voxel mesh, calculate visible sides and send the visible sides to the gpu
*/
use bevy::{asset::{AssetServer, Handle}, ecs::{resource::Resource, world::FromWorld}, image::BevyDefault, mesh::{Mesh, MeshVertexBufferLayout, MeshVertexBufferLayoutRef}, pbr::MeshPipelineKey, render::{render_resource::{BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, BindingType, BlendState, BufferBindingType, ColorTargetState, ColorWrites, FragmentState, MultisampleState, PrimitiveState, RenderPipelineDescriptor, ShaderStages, SpecializedMeshPipeline, SpecializedMeshPipelineError, TextureFormat, VertexState}, renderer::RenderDevice}, shader::Shader};

#[derive(Resource)]
pub struct VoxelPipeline {
    pub shader: Handle<Shader>
}
impl SpecializedMeshPipeline for VoxelPipeline {
    type Key = MeshPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        // key: MeshPipelineKey,
        layout: &MeshVertexBufferLayoutRef,
    ) -> Result<RenderPipelineDescriptor,SpecializedMeshPipelineError> {
        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
        ])?;
        Ok(RenderPipelineDescriptor {
            label: Some("voxel pipeline".into()),
            layout: vec![],
            vertex: VertexState {
                shader: self.shader.clone(),
                entry_point: Some("vertex".into()),
                shader_defs: vec![],
                buffers: vec![vertex_layout]
            },
            fragment: Some(FragmentState {
                shader: self.shader.clone(),
                entry_point: Some("fragment".into()),
                shader_defs: vec![],
                targets: vec![
                    Some(ColorTargetState {
                        format: TextureFormat::bevy_default(),
                        blend: Some(BlendState::REPLACE),
                        write_mask: ColorWrites::ALL,
                    })
                ],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            push_constant_ranges: vec![],
            zero_initialize_workgroup_memory: false,
        })
    }
}
impl FromWorld for VoxelPipeline {
    fn from_world(world: &mut bevy::ecs::world::World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let asset_server = world.resource::<AssetServer>();
        let shader_handle = asset_server.load("voxel.wgsl");
        let layout = render_device.create_bind_group_layout(
            Some("voxel bind group layout"),
            &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::all(),
                    ty: BindingType::Buffer {
                        has_dynamic_offset: false,
                        min_binding_size: None,
                        ty: BufferBindingType::Uniform,
                    },
                    count: None,
                }
            ]
        );
        Self {
            shader: shader_handle,
            // might need to store layout
        }
    }
}