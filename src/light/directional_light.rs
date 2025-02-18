use crate::camera::*;
use crate::core::*;
use crate::definition::*;
use crate::math::*;
use crate::object::*;

///
/// A light which shines in the given direction.
/// The light will cast shadows if you [generate a shadow map](DirectionalLight::generate_shadow_map).
///
pub struct DirectionalLight {
    context: Context,
    light_buffer: UniformBuffer,
    shadow_texture: DepthTargetTexture2D,
    shadow_camera: Option<Camera>,
}

impl DirectionalLight {
    pub fn new(
        context: &Context,
        intensity: f32,
        color: &Vec3,
        direction: &Vec3,
    ) -> Result<DirectionalLight, Error> {
        let mut light = DirectionalLight {
            context: context.clone(),
            light_buffer: UniformBuffer::new(context, &[3u32, 1, 3, 1, 16])?,
            shadow_texture: DepthTargetTexture2D::new(
                context,
                1,
                1,
                Wrapping::ClampToEdge,
                Wrapping::ClampToEdge,
                DepthFormat::Depth32F,
            )?,
            shadow_camera: None,
        };

        light.set_intensity(intensity);
        light.set_color(color);
        light.set_direction(direction);
        Ok(light)
    }

    pub fn set_color(&mut self, color: &Vec3) {
        self.light_buffer.update(0, &color.to_slice()).unwrap();
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        self.light_buffer.update(1, &[intensity]).unwrap();
    }

    pub fn set_direction(&mut self, direction: &Vec3) {
        self.light_buffer
            .update(2, &direction.normalize().to_slice())
            .unwrap();
    }

    pub fn direction(&self) -> Vec3 {
        let d = self.light_buffer.get(2).unwrap();
        vec3(d[0], d[1], d[2])
    }

    pub fn clear_shadow_map(&mut self) {
        self.shadow_camera = None;
        self.shadow_texture = DepthTargetTexture2D::new(
            &self.context,
            1,
            1,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            DepthFormat::Depth32F,
        )
        .unwrap();
        self.light_buffer.update(3, &[0.0]).unwrap();
    }

    pub fn generate_shadow_map(
        &mut self,
        target: &Vec3,
        frustrum_width: f32,
        frustrum_height: f32,
        frustrum_depth: f32,
        texture_width: u32,
        texture_height: u32,
        geometries: &[&dyn Geometry],
    ) -> Result<(), Error> {
        let direction = self.direction();
        let up = compute_up_direction(direction);

        self.shadow_camera = Some(Camera::new_orthographic(
            &self.context,
            target - direction.normalize() * 0.5 * frustrum_depth,
            *target,
            up,
            frustrum_width,
            frustrum_height,
            frustrum_depth,
        )?);
        self.light_buffer.update(
            4,
            &shadow_matrix(self.shadow_camera.as_ref().unwrap()).to_slice(),
        )?;

        self.shadow_texture = DepthTargetTexture2D::new(
            &self.context,
            texture_width,
            texture_height,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            DepthFormat::Depth32F,
        )
        .unwrap();
        self.shadow_texture.write(Some(1.0), || {
            let viewport = Viewport::new_at_origo(texture_width, texture_height);
            for geometry in geometries {
                if geometry
                    .aabb()
                    .map(|aabb| self.shadow_camera.as_ref().unwrap().in_frustum(&aabb))
                    .unwrap_or(true)
                {
                    geometry.render_depth(
                        RenderStates::default(),
                        viewport,
                        self.shadow_camera.as_ref().unwrap(),
                    )?;
                }
            }
            Ok(())
        })?;
        self.light_buffer.update(3, &[1.0])?;
        Ok(())
    }

    pub fn shadow_map(&self) -> &DepthTargetTexture2D {
        &self.shadow_texture
    }

    pub fn buffer(&self) -> &UniformBuffer {
        &self.light_buffer
    }
}

fn shadow_matrix(camera: &Camera) -> Mat4 {
    let bias_matrix = crate::Mat4::new(
        0.5, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.5, 0.5, 0.5, 1.0,
    );
    bias_matrix * camera.projection() * camera.view()
}

fn compute_up_direction(direction: Vec3) -> Vec3 {
    if vec3(1.0, 0.0, 0.0).dot(direction).abs() > 0.9 {
        (vec3(0.0, 1.0, 0.0).cross(direction)).normalize()
    } else {
        (vec3(1.0, 0.0, 0.0).cross(direction)).normalize()
    }
}
