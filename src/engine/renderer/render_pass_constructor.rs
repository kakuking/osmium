use std::sync::Arc;

use vulkano::{
    device::Device, 
    format::Format, 
    render_pass::RenderPass
};

use crate::engine::renderer::config::RendererConfig;

pub struct RenderPassConstructor {

}

impl RenderPassConstructor {
    pub fn create_render_pass(
        config: &RendererConfig,
        device: Arc<Device>,
        image_format: Format,
        depth_format: Format
    ) -> Arc<RenderPass> {
        match (config.depth_enabled, config.render_pass.samples) {
            (false, 1) => Self::color(config, device, image_format),
            (false, _) => Self::color_msaa(config, device, image_format),
            (true, 1) => Self::color_depth(config, device, image_format, depth_format),
            (true, _) => Self::color_depth_msaa(config, device, image_format, depth_format),
        }
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
                        },
                        depth: {
                            format: depth_format,
                            samples: 1,
                            load_op: Clear,
                            store_op: DontCare,
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
                        },
                        depth: {
                            format: depth_format,
                            samples: 1,
                            load_op: DontCare,
                            store_op: DontCare,
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
                        },
                        depth: {
                            format: depth_format,
                            samples: 1,
                            load_op: Clear,
                            store_op: DontCare,
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
                        },
                        depth: {
                            format: depth_format,
                            samples: 1,
                            load_op: Clear,
                            store_op: DontCare,
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
                },
                color_resolve: {
                    format: image_format,
                    samples: 1,
                    load_op: DontCare,
                    store_op: Store,
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
                },
                color_resolve: {
                    format: image_format,
                    samples: 1,
                    load_op: DontCare,
                    store_op: Store,
                },
                depth: {
                    format: depth_format,
                    samples: samples,
                    load_op: Clear,
                    store_op: DontCare,
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