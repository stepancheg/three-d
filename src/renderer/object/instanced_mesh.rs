use crate::core::*;
use crate::renderer::Geometry;

pub struct InstancedModel {
    context: Context,
    pub(in crate::renderer) mesh: InstancedMesh,
    pub cull: CullType,
}

impl InstancedModel {
    pub fn new(
        context: &Context,
        transformations: &[Mat4],
        cpu_mesh: &CPUMesh,
    ) -> Result<Self, Error> {
        let mesh = InstancedMesh::new(context, transformations, cpu_mesh)?;
        unsafe {
            MESH_COUNT += 1;
        }
        Ok(Self {
            context: context.clone(),
            mesh,
            cull: CullType::default(),
        })
    }

    pub fn transformation(&self) -> &Mat4 {
        self.mesh.transformation()
    }

    pub fn set_transformation(&mut self, transformation: Mat4) {
        self.mesh.set_transformation(transformation);
    }

    ///
    /// Render the instanced model with a color per triangle vertex. The colors are defined when constructing the instanced model.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    /// The transformation can be used to position, orientate and scale the instanced model.
    ///
    /// # Errors
    /// Will return an error if the instanced model has no colors.
    ///
    pub fn render_color(&self, camera: &Camera) -> Result<(), Error> {
        let program = self.get_or_insert_program(&format!(
            "{}{}",
            include_str!("../../core/shared.frag"),
            include_str!("shaders/mesh_vertex_color.frag")
        ))?;
        self.mesh.render(
            self.render_states(self.mesh.transparent),
            program,
            camera.uniform_buffer(),
            camera.viewport(),
        )
    }

    ///
    /// Render the instanced model with the given color. The color is assumed to be in gamma color space (sRGBA).
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    /// The transformation can be used to position, orientate and scale the instanced model.
    ///
    pub fn render_with_color(&self, color: &Color, camera: &Camera) -> Result<(), Error> {
        let program = self.get_or_insert_program(include_str!("shaders/mesh_color.frag"))?;
        program.use_uniform_vec4("color", &color.to_vec4())?;
        self.mesh.render(
            self.render_states(color.a != 255),
            program,
            camera.uniform_buffer(),
            camera.viewport(),
        )
    }

    ///
    /// Render the instanced model with the given texture which is assumed to be in sRGB color space with or without an alpha channel.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    /// The transformation can be used to position, orientate and scale the instanced model.
    ///
    /// # Errors
    /// Will return an error if the instanced model has no uv coordinates.
    ///
    pub fn render_with_texture(
        &self,
        texture: &impl Texture,
        camera: &Camera,
    ) -> Result<(), Error> {
        let program = self.get_or_insert_program(include_str!("shaders/mesh_texture.frag"))?;
        program.use_texture("tex", texture)?;
        self.mesh.render(
            self.render_states(texture.format() == Format::RGBA),
            program,
            camera.uniform_buffer(),
            camera.viewport(),
        )
    }

    pub(in crate::renderer) fn render_states(&self, transparent: bool) -> RenderStates {
        if transparent {
            RenderStates {
                cull: self.cull,
                write_mask: WriteMask::COLOR,
                blend: Some(BlendParameters::TRANSPARENCY),
                ..Default::default()
            }
        } else {
            RenderStates {
                cull: self.cull,
                ..Default::default()
            }
        }
    }

    pub(in crate::renderer) fn get_or_insert_program(
        &self,
        fragment_shader_source: &str,
    ) -> Result<&InstancedMeshProgram, Error> {
        unsafe {
            if PROGRAMS.is_none() {
                PROGRAMS = Some(std::collections::HashMap::new());
            }
            if !PROGRAMS
                .as_ref()
                .unwrap()
                .contains_key(fragment_shader_source)
            {
                PROGRAMS.as_mut().unwrap().insert(
                    fragment_shader_source.to_string(),
                    InstancedMeshProgram::new(&self.context, fragment_shader_source)?,
                );
            };
            Ok(PROGRAMS
                .as_ref()
                .unwrap()
                .get(fragment_shader_source)
                .unwrap())
        }
    }
}

impl Geometry for InstancedModel {
    fn render_depth_to_red(&self, camera: &Camera, max_depth: f32) -> Result<(), Error> {
        let program = self.get_or_insert_program(include_str!("shaders/mesh_pick.frag"))?;
        program.use_uniform_float("maxDistance", &max_depth)?;
        self.mesh.render(
            RenderStates {
                write_mask: WriteMask {
                    red: true,
                    depth: true,
                    ..WriteMask::NONE
                },
                cull: self.cull,
                ..Default::default()
            },
            program,
            camera.uniform_buffer(),
            camera.viewport(),
        )
    }

    fn render_depth(&self, camera: &Camera) -> Result<(), Error> {
        let program = self.get_or_insert_program("void main() {}")?;
        self.mesh.render(
            RenderStates {
                write_mask: WriteMask::DEPTH,
                cull: self.cull,
                ..Default::default()
            },
            program,
            camera.uniform_buffer(),
            camera.viewport(),
        )
    }

    fn aabb(&self) -> Option<AxisAlignedBoundingBox> {
        None // TODO: Compute bounding box
    }
}

impl Drop for InstancedModel {
    fn drop(&mut self) {
        unsafe {
            MESH_COUNT -= 1;
            if MESH_COUNT == 0 {
                PROGRAMS = None;
            }
        }
    }
}

static mut PROGRAMS: Option<std::collections::HashMap<String, InstancedMeshProgram>> = None;
static mut MESH_COUNT: u32 = 0;
