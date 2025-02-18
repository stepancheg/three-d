use crate::camera::*;
use crate::core::*;
use crate::definition::*;
use crate::math::*;
use crate::object::*;

///
/// Three arrows indicating the three main axes; the x-axis (red), the y-axis (green) and the z-axis (blue).
/// Used for easily debugging where objects are placed in the 3D world.
///
#[derive(Clone)]
pub struct Axes {
    x: Mesh,
    y: Mesh,
    z: Mesh,
}

impl Axes {
    ///
    /// Creates a new axes object consisting of three arrows with the given radius and length.
    ///
    pub fn new(context: &Context, radius: f32, length: f32) -> Result<Self, Error> {
        let x = Mesh::new(context, &CPUMesh::arrow(radius, length, 16))?;
        let mut y = Mesh::new(context, &CPUMesh::arrow(radius, length, 16))?;
        let mut z = Mesh::new(context, &CPUMesh::arrow(radius, length, 16))?;
        y.transformation = Mat4::from_angle_z(degrees(90.0));
        z.transformation = Mat4::from_angle_y(degrees(-90.0));
        Ok(Self { x, y, z })
    }

    ///
    /// Render the axes.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    /// The transformation can be used to position, orientate and scale the axes.
    ///
    pub fn render(&self, viewport: Viewport, camera: &Camera) -> Result<(), Error> {
        self.x.render_with_color(
            &vec4(1.0, 0.0, 0.0, 1.0),
            RenderStates::default(),
            viewport,
            camera,
        )?;
        self.y.render_with_color(
            &vec4(0.0, 1.0, 0.0, 1.0),
            RenderStates::default(),
            viewport,
            camera,
        )?;
        self.z.render_with_color(
            &vec4(0.0, 0.0, 1.0, 1.0),
            RenderStates::default(),
            viewport,
            camera,
        )?;

        Ok(())
    }
}
