use crate::pass::IcedPass;
use crate::vertex::TriangleVertex;
use amethyst::renderer::{rendy::factory::Factory, types::Backend};
use glsl_layout::vec4;
use iced_native::{Color, Rectangle};

#[allow(dead_code)]
pub enum AmethystIcedPrimitive {
    Quad(Rectangle, Option<Color>),
    Image,
    Text,
    Group(Vec<AmethystIcedPrimitive>),
}

/// Wrapper struct meant to avoid an user from interfering (accidentally or not)
/// into amethyst_iced's primitives
pub(crate) struct IcedPrimitives(pub(crate) Option<AmethystIcedPrimitive>);

impl Default for IcedPrimitives {
    fn default() -> Self {
        IcedPrimitives(None)
    }
}

impl AmethystIcedPrimitive {
    /// Consumes the Primitive, rendering it on the pass
    pub fn render<B: Backend>(self, pass: &mut IcedPass<B>, factory: &Factory<B>, index: usize) {
        match self {
            AmethystIcedPrimitive::Group(primitives) => primitives.into_iter().for_each(|p| {
                p.render(pass, factory, index);
            }),
            AmethystIcedPrimitive::Quad(bounds, color) => {
                let iced_color = color.unwrap_or(Color::WHITE);
                let color: vec4 = [iced_color.r, iced_color.g, iced_color.b, iced_color.a].into();
                println!("drawing bounds {:?}", bounds);

                pass.triangle_pipeline.vertices.extend_from_slice(&[
                    TriangleVertex {
                        position: [bounds.x, bounds.y].into(),
                        color,
                    },
                    TriangleVertex {
                        position: [bounds.x + bounds.width, bounds.y].into(),
                        color,
                    },
                    TriangleVertex {
                        position: [bounds.x + bounds.width, bounds.y + bounds.height].into(),
                        color,
                    },
                    TriangleVertex {
                        position: [bounds.x, bounds.y].into(),
                        color,
                    },
                    TriangleVertex {
                        position: [bounds.x, bounds.y + bounds.height].into(),
                        color,
                    },
                    TriangleVertex {
                        position: [bounds.x + bounds.width, bounds.y + bounds.height].into(),
                        color,
                    },
                ]);
            }
            _ => unimplemented!(),
        }
    }
}
