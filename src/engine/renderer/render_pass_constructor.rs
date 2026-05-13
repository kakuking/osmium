use std::sync::Arc;

use vulkano::{
    device::Device, 
    format::Format, 
    render_pass::RenderPass,
    image::ImageLayout::{
        Undefined,
        ColorAttachmentOptimal,
        PresentSrc,
        DepthStencilAttachmentOptimal,
        DepthStencilReadOnlyOptimal
    }
};

use crate::engine::config::renderer_config::RendererConfig;

pub struct RenderPassConstructor;

impl RenderPassConstructor {
    pub fn create_render_pass(
        config: &RendererConfig,
        device: Arc<Device>,
        image_format: Format,
        depth_format: Format
    ) -> Arc<RenderPass> {
        match config.render_pass.samples {
            1 => Self::color_only(config, device, image_format, depth_format),
            _ => Self::color_msaa(config, device, image_format, depth_format),
        }
    }

    pub fn create_shadow_render_pass(
        device: Arc<Device>,
        depth_format: Format,
    ) -> Arc<RenderPass> {
        vulkano::single_pass_renderpass!(
            device,
            attachments: {
                depth: {
                    format: depth_format,
                    samples: 1,
                    load_op: Clear,
                    store_op: Store,
                    initial_layout: Undefined,
                    final_layout: DepthStencilReadOnlyOptimal,
                },
            },
            pass: {
                color: [],
                depth_stencil: {depth},
            },
        )
        .unwrap()
    }

    fn color_only(
        config: &RendererConfig,
        device: Arc<Device>,
        image_format: Format,
        depth_format: Format
    ) -> Arc<RenderPass> {
        match (
            config.render_pass.clear_color,
            config.render_pass.store_color,
            config.render_pass.clear_depth,
        ) {
            (true, true, true) => {
                vulkano::single_pass_renderpass!(
                    device,
                    attachments: {
                        color: {
                            format: image_format,
                            samples: 1,
                            load_op: Clear,
                            store_op: Store,
                            initial_layout: PresentSrc,
                            final_layout: PresentSrc,
                        },
                        depth: {
                            format: depth_format,
                            samples: 1,
                            load_op: Clear,
                            store_op: DontCare,
                            initial_layout: Undefined,
                            final_layout: DepthStencilAttachmentOptimal,
                        },
                    },
                    pass: {
                        color: [color],
                        depth_stencil: {depth},
                    },
                )
                .unwrap()
            }

            (true, true, false) => {
                vulkano::single_pass_renderpass!(
                    device,
                    attachments: {
                        color: {
                            format: image_format,
                            samples: 1,
                            load_op: Clear,
                            store_op: Store,
                            initial_layout: PresentSrc,
                            final_layout: PresentSrc,
                        },
                        depth: {
                            format: depth_format,
                            samples: 1,
                            load_op: DontCare,
                            store_op: DontCare,
                            initial_layout: DepthStencilAttachmentOptimal,
                            final_layout: DepthStencilAttachmentOptimal,
                        },
                    },
                    pass: {
                        color: [color],
                        depth_stencil: {depth},
                    },
                )
                .unwrap()
            }

            (false, true, true) => {
                vulkano::single_pass_renderpass!(
                    device,
                    attachments: {
                        color: {
                            format: image_format,
                            samples: 1,
                            load_op: DontCare,
                            store_op: Store,
                            initial_layout: PresentSrc,
                            final_layout: PresentSrc,
                        },
                        depth: {
                            format: depth_format,
                            samples: 1,
                            load_op: Clear,
                            store_op: DontCare,
                            initial_layout: DepthStencilAttachmentOptimal,
                            final_layout: DepthStencilAttachmentOptimal,
                        },
                    },
                    pass: {
                        color: [color],
                        depth_stencil: {depth},
                    },
                )
                .unwrap()
            }

            _ => {
                vulkano::single_pass_renderpass!(
                    device,
                    attachments: {
                        color: {
                            format: image_format,
                            samples: 1,
                            load_op: Clear,
                            store_op: Store,
                            initial_layout: PresentSrc,
                            final_layout: PresentSrc,
                        },
                        depth: {
                            format: depth_format,
                            samples: 1,
                            load_op: Clear,
                            store_op: DontCare,
                            initial_layout: Undefined,
                            final_layout: DepthStencilAttachmentOptimal,
                        },
                    },
                    pass: {
                        color: [color],
                        depth_stencil: {depth},
                    },
                )
                .unwrap()
            }
        }
    }

    fn color_msaa(
        config: &RendererConfig,
        device: Arc<Device>,
        image_format: Format,
        depth_format: Format
    ) -> Arc<RenderPass> {
        let samples = config.render_pass.samples;

        vulkano::ordered_passes_renderpass!(
            device,
            attachments: {
                color_msaa: {
                    format: image_format,
                    samples: samples,
                    load_op: Clear,
                    store_op: DontCare,
                    initial_layout: Undefined,
                    final_layout: ColorAttachmentOptimal,
                },
                color_resolve: {
                    format: image_format,
                    samples: 1,
                    load_op: DontCare,
                    store_op: Store,
                    initial_layout: Undefined,
                    final_layout: PresentSrc,
                },
                depth: {
                    format: depth_format,
                    samples: samples,
                    load_op: Clear,
                    store_op: DontCare,
                    initial_layout: Undefined,
                    final_layout: DepthStencilAttachmentOptimal,
                },
            },
            passes: [
                {
                    color: [color_msaa],
                    color_resolve: [color_resolve],
                    depth_stencil: { depth },
                    input: [],
                },
                {
                    color: [color_resolve],
                    color_resolve: [],
                    depth_stencil: {},
                    input: [],
                }
            ],
        )
        .unwrap()
    }
}