use bevy::{app::Plugin, render::{ExtractSchedule, RenderApp, render_resource::SpecializedMeshPipelines}};

use crate::fast_voxels::voxel_pipeline::VoxelPipeline;


pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .init_resource::<VoxelPipeline>()
            .init_resource::<SpecializedMeshPipelines<VoxelPipeline>>()
            // .add_systems(ExtractSchedule, extract_custom_data)
            // .add_systems(RenderSchedule,queue_custom_mesh.in_set(RenderSet::Queue))
            ;
    }
}