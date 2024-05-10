use std::sync::Arc;

use super::types::Vertex3D;
use crate::backend::{self, device::RenderDevice};
use bevy_ecs::{
    event::EventReader,
    system::{Commands, Res, Resource},
};
use vulkano::{
    pipeline::{
        graphics::{
            color_blend::{ColorBlendAttachmentState, ColorBlendState},
            multisample::MultisampleState,
            rasterization::RasterizationState,
            vertex_input::{Vertex, VertexDefinition},
            GraphicsPipelineCreateInfo,
        },
        layout::PipelineDescriptorSetLayoutCreateInfo,
        GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo,
    },
    render_pass::{RenderPass, Subpass},
};

#[derive(Resource)]
pub struct PipelineSetup {
    pub pipeline: Arc<GraphicsPipeline>,
    pub render_pass: Arc<RenderPass>,
}

pub fn setup_pipeline(
    device: Res<RenderDevice>,
    swapchain: Res<backend::swapchain::Swapchain>,
    mut event: EventReader<window::events::Resized>,
    mut commands: Commands,
) {
    if event.read().last().is_some() {
        let extent = swapchain.swapchain.image_extent();

        let vs = vs::load(device.device.clone()).expect("failed to create shader module");
        let fs = fs::load(device.device.clone()).expect("failed to create shader module");

        let vs = vs.entry_point("main").unwrap();
        let fs = fs.entry_point("main").unwrap();

        let vertex_input_state = Vertex3D::per_vertex().definition(&vs).unwrap();

        let stages = [
            PipelineShaderStageCreateInfo::new(vs),
            PipelineShaderStageCreateInfo::new(fs),
        ];

        let layout = PipelineLayout::new(
            device.device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                .into_pipeline_layout_create_info(device.device.clone())
                .unwrap(),
        )
        .unwrap();

        let render_pass = vulkano::single_pass_renderpass!(
        device.device.clone(),
        attachments: {
            color: {
                format: swapchain.swapchain.image_format(),
                samples: 1,
                load_op: Clear,
                store_op: Store,
            },
        },
        pass: {
            color: [color],
            depth_stencil: {},
        },
        )
        .unwrap();

        use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
        use vulkano::pipeline::graphics::viewport::{Viewport, ViewportState};
        use vulkano::pipeline::GraphicsPipeline;

        let subpass = Subpass::from(render_pass.clone(), 0).unwrap();

        let pipeline = GraphicsPipeline::new(
            device.device.clone(),
            None,
            GraphicsPipelineCreateInfo {
                stages: stages.into_iter().collect(),
                vertex_input_state: Some(vertex_input_state),
                input_assembly_state: Some(InputAssemblyState::default()),
                viewport_state: Some(ViewportState {
                    viewports: [Viewport {
                        offset: [0.0, 0.0],
                        extent: [extent[0] as f32, extent[1] as f32],
                        depth_range: 0.0..=1.0,
                    }]
                    .into_iter()
                    .collect(),
                    ..Default::default()
                }),
                rasterization_state: Some(RasterizationState {
                    cull_mode: vulkano::pipeline::graphics::rasterization::CullMode::Front,
                    ..Default::default()
                }),
                multisample_state: Some(MultisampleState::default()),
                color_blend_state: Some(ColorBlendState::with_attachment_states(
                    subpass.num_color_attachments(),
                    ColorBlendAttachmentState::default(),
                )),
                subpass: Some(subpass.into()),
                ..GraphicsPipelineCreateInfo::layout(layout)
            },
        )
        .unwrap();

        commands.insert_resource(PipelineSetup {
            pipeline,
            render_pass,
        });
    }
}

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/shaders/vertex.glsl"
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/shaders/fragment.glsl",
    }
}
