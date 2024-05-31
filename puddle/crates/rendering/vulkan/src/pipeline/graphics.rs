use std::sync::Arc;

use application::log::trace;
use vulkano::{
    pipeline::{
        graphics::{
            color_blend::{ColorBlendAttachmentState, ColorBlendState},
            input_assembly::InputAssemblyState,
            multisample::MultisampleState,
            rasterization::RasterizationState,
            vertex_input::{Vertex, VertexDefinition},
            viewport::{Viewport, ViewportState},
            GraphicsPipelineCreateInfo,
        },
        layout::PipelineDescriptorSetLayoutCreateInfo,
        PipelineLayout,
    },
    render_pass::{RenderPass, Subpass},
};

pub struct GraphicsPipeline(pub Arc<vulkano::pipeline::GraphicsPipeline>);

pub struct GraphicsPipelineDescriber {
    pub shaders: super::shaders::GraphicsShaderSet,
    pub render_pass: Arc<RenderPass>,
    pub extent: [u32; 2],
}

impl GraphicsPipeline {
    pub fn new(device: &crate::Device, desc: &GraphicsPipelineDescriber) -> Self {
        let vertex_input_state = utils::Vertex::per_vertex()
            .definition(&desc.shaders.vertex)
            .unwrap();

        let subpass = Subpass::from(desc.render_pass.clone(), 0).unwrap();

        let layout = PipelineLayout::new(
            device.device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages(&desc.shaders.to_desc())
                .into_pipeline_layout_create_info(device.device.clone())
                .unwrap(),
        )
        .unwrap();

        let pipeline = vulkano::pipeline::GraphicsPipeline::new(
            device.device.clone(),
            None,
            GraphicsPipelineCreateInfo {
                stages: desc.shaders.to_desc().into(),

                vertex_input_state: Some(vertex_input_state),

                input_assembly_state: Some(InputAssemblyState::default()),

                viewport_state: Some(ViewportState {
                    viewports: [Viewport {
                        offset: [0.0, 0.0],
                        extent: [desc.extent[0] as f32, desc.extent[1] as f32],
                        depth_range: 0.0..=1.0,
                    }]
                    .into_iter()
                    .collect(),
                    ..Default::default()
                }),

                rasterization_state: Some(RasterizationState::default()),

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

        trace!("sucsessfully created new GraphicsPipeline");

        Self(pipeline)
    }
}
