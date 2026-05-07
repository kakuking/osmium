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
        match (config.enable_depth, config.render_pass.samples) {
            (false, 1) => Self::color(config, device, image_format),
            (false, _) => Self::color_msaa(config, device, image_format),
            (true, 1) => Self::color_depth(config, device, image_format, depth_format),
            (true, _) => Self::color_depth_msaa(config, device, image_format, depth_format),
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

    fn color(
        config: &RendererConfig,
        device: Arc<Device>,
        image_format: Format,
    ) -> Arc<RenderPass> {
        match (
            config.render_pass.clear_color,
            config.render_pass.store_color,
        ) {
            (true, true) => {
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
                    },
                    pass: {
                        color: [color],
                        depth_stencil: {},
                    },
                )
                .unwrap()
            }

            (true, false) => {
                vulkano::single_pass_renderpass!(
                    device,
                    attachments: {
                        color: {
                            format: image_format,
                            samples: 1,
                            load_op: Clear,
                            store_op: DontCare,
                            initial_layout: PresentSrc,
                            final_layout: PresentSrc,
                        },
                    },
                    pass: {
                        color: [color],
                        depth_stencil: {},
                    },
                )
                .unwrap()
            }

            (false, true) => {
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
                    },
                    pass: {
                        color: [color],
                        depth_stencil: {},
                    },
                )
                .unwrap()
            }

            (false, false) => {
                vulkano::single_pass_renderpass!(
                    device,
                    attachments: {
                        color: {
                            format: image_format,
                            samples: 1,
                            load_op: DontCare,
                            store_op: DontCare,
                            initial_layout: PresentSrc,
                            final_layout: PresentSrc,
                        },
                    },
                    pass: {
                        color: [color],
                        depth_stencil: {},
                    },
                )
                .unwrap()
            }
        }
    }

    fn color_depth(
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
    ) -> Arc<RenderPass> {
        let samples = config.render_pass.samples;

        vulkano::single_pass_renderpass!(
            device,
            attachments: {
                color_msaa: {
                    format: image_format,
                    samples: samples,
                    load_op: Clear,
                    store_op: DontCare,
                    initial_layout: ColorAttachmentOptimal,
                    final_layout: ColorAttachmentOptimal,
                },
                color_resolve: {
                    format: image_format,
                    samples: 1,
                    load_op: DontCare,
                    store_op: Store,
                    initial_layout: PresentSrc,
                    final_layout: PresentSrc,
                },
            },
            pass: {
                color: [color_msaa],
                color_resolve: [color_resolve],
                depth_stencil: {},
            },
        )
        .unwrap()
    }

    fn color_depth_msaa(
        config: &RendererConfig,
        device: Arc<Device>,
        image_format: Format,
        depth_format: Format
    ) -> Arc<RenderPass> {
        let samples = config.render_pass.samples;

        vulkano::single_pass_renderpass!(
            device,
            attachments: {
                color_msaa: {
                    format: image_format,
                    samples: samples,
                    load_op: Clear,
                    store_op: DontCare,
                    initial_layout: ColorAttachmentOptimal,
                    final_layout: ColorAttachmentOptimal,
                },
                color_resolve: {
                    format: image_format,
                    samples: 1,
                    load_op: DontCare,
                    store_op: Store,
                    initial_layout: PresentSrc,
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
            pass: {
                color: [color_msaa],
                color_resolve: [color_resolve],
                depth_stencil: {depth},
            },
        )
        .unwrap()
    }
}