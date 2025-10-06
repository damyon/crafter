use crate::camera::Camera;
use crate::drawable::Drawable;
use glium::Frame;
use glium::Program;
use glium::Surface;
use glium::backend::glutin::Display;
use glium::texture::MipmapsOption;
use glium::texture::Texture2d;
use glium::texture::UncompressedFloatFormat;
use glium::uniform;
use glutin::surface::WindowSurface;
use nalgebra::*;

/// All the things we need to know to render to the screen.
pub struct Graphics<'a> {
    pub display: &'a Display<WindowSurface>,
    pub frame: &'a mut Frame,
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub camera_program: Option<Program>,
    pub light_program: Option<Program>,
    pub shadow_depth_texture: Option<Texture2d>,
    pub shadow_texture_size: u32,
    pub swap_shaders: bool,
    pub swap_cameras: bool,
}

impl<'a> Graphics<'a> {
    /// Create a new Graphics container with default values.
    pub fn new(
        display: &'a Display<WindowSurface>,
        frame: &'a mut Frame,
        canvas_width: u32,
        canvas_height: u32,
    ) -> Graphics<'a> {
        Graphics {
            display,
            frame,
            canvas_width,
            canvas_height,
            camera_program: None,
            light_program: None,
            shadow_depth_texture: None,
            shadow_texture_size: 4096,
            swap_shaders: false,
            swap_cameras: false,
        }
    }

    /// Create a texture large enough to record depth values for shadow mapping.
    pub fn create_shadow_depth_texture(&mut self) {
        self.shadow_depth_texture = Some(
            Texture2d::empty_with_format(
                self.display,
                UncompressedFloatFormat::F32F32F32F32, // Example format: RGBA with 32-bit floats
                MipmapsOption::NoMipmap,               // No mipmaps for this example
                self.shadow_texture_size,
                self.shadow_texture_size,
            )
            .unwrap(),
        );
    }

    /// Get the view from the light for calculating shadows.
    pub fn build_light_projection(&self) -> Matrix4<f32> {
        if self.swap_cameras {
            Perspective3::new(
                self.canvas_width as f32 / self.canvas_height as f32,
                std::f32::consts::PI / 4.0, // 45 degrees
                1.0,
                200.0,
            )
            .into_inner()
        } else {
            Orthographic3::new(-64.0, 64.0, -64.0, 64.0, 1.0, 240.0).into_inner()
        }
    }

    /// Get the view from the camera.
    pub fn build_camera_projection(&self) -> Matrix4<f32> {
        if self.swap_cameras {
            Orthographic3::new(-32.0, 32.0, -32.0, 32.0, 0.1, 120.0).into_inner()
        } else {
            Perspective3::new(
                self.canvas_width as f32 / self.canvas_height as f32,
                std::f32::consts::PI / 4.0, // 45 degrees
                1.0,
                200.0,
            )
            .into_inner()
        }
    }

    /// Compile the various shaders.
    pub fn setup_shaders(&mut self) {
        self.light_program = Some(self.setup_light_shaders());
        self.camera_program = Some(self.setup_camera_shaders());
        self.create_shadow_depth_texture();
    }

    /// Compile the light shaders.
    pub fn setup_light_shaders(&mut self) -> Program {
        let vertex_shader_source = "#version 460
                in vec3 position;

                uniform mat4 uPMatrix;
                uniform mat4 uMVMatrix;

                void main (void) {
                    vec4 a_position = vec4(position, 1.0);
                    gl_Position = uPMatrix * uMVMatrix * a_position;
                }
            ";
        let fragment_shader_source = "#version 460
                precision mediump float;
                out vec4 fragColor;

                float LinearizeDepth(float depth)
                {
                    return depth;
                }

                void main()
                {
                    fragColor = vec4(vec3(LinearizeDepth(gl_FragCoord.z + 0.0005)), 1.0);
                }
                ";

        let program = glium::Program::from_source(
            self.display,
            vertex_shader_source,
            fragment_shader_source,
            None,
        );
        if program.is_err() {
            panic!("Failed to create program: {}", program.unwrap_err());
        }

        program.unwrap()
    }

    /// Compile the camera shaders.
    pub fn setup_camera_shaders(&mut self) -> Program {
        let vertex_shader_source = "#version 460
                in vec3 position;
                in vec3 normal;
                uniform mat4 uPMatrix;
                uniform mat4 uMVMatrix;
                uniform mat4 uMMatrix;
                uniform mat4 u_light_PMatrix;
                uniform mat4 u_light_MVMatrix;
                out vec4 positionFromLightPov;
                out vec4 worldPosition;
                out vec3 v_normal;

                void main(void) {
                    // Multiply the position by the matrix.
                    vec4 a_position = vec4(position, 1.0);
                    gl_Position = uPMatrix * uMVMatrix * a_position;

                    positionFromLightPov = u_light_PMatrix * u_light_MVMatrix * a_position;
                    // This is incorrect on purpose because a voxel grid aligns with the axis.
                    worldPosition = uPMatrix * uMMatrix * a_position;
                    v_normal = normal;
                }
                ";

        let fragment_shader_source = "#version 460
                precision mediump float;
                uniform vec4 u_color;
                uniform bool u_fluid;
                uniform bool u_noise;
                uniform float u_time;
                uniform int u_shadow_texture_size;
                uniform sampler2D shadowMap;
                out vec4 fragColor;
                in vec4 positionFromLightPov;
                in vec4 positionFromLightMV;
                in vec4 worldPosition;
                in vec3 v_normal;

                float rand(vec2 co){
                    return fract(sin(dot(co, vec2(12.9898, 78.233))) * 43758.5453);
                }

                float animateFluid() {
                    // We calculate the distance between the point and 3 ripple source locations
                    // and combine 3 sinewaves from the 3 distances.
                    vec3 ripple1 = vec3(100.0, 40.0, 10.0);
                    vec3 ripple2 = vec3(50.0, -40.0, 30.0);
                    vec3 ripple3 = vec3(-40.0, 40.0, -80.0);
                    vec3 ripple4 = vec3(34.0, 23.0, 12.0);
                    vec3 ripple5 = vec3(8.0, -13.0, 73.0);
                    vec3 ripple6 = vec3(-25.0, 67.0, -34.0);
                    float period = 4.0;
                    float distance1 = length(worldPosition.xyz - ripple1) * period;
                    float distance2 = length(worldPosition.xyz - ripple2) * period;
                    float distance3 = length(worldPosition.xyz - ripple3) * period;
                    float distance4 = length(worldPosition.xyz - ripple4) * period;
                    float distance5 = length(worldPosition.xyz - ripple5) * period;
                    float distance6 = length(worldPosition.xyz - ripple6) * period;
                    float speed = 10.0;
                    float scale = u_time * speed;
                    return (
                        sin(distance1 + scale) +
                        sin(distance2 + scale) +
                        sin(distance3 + scale) +
                        sin(distance4 + scale) +
                        sin(distance5 + scale) +
                        sin(distance6 + scale)
                        );
                }

                void main(void) {
                    float ambientLight = 0.5;
                    vec3 positionFromLightPovInTexture = positionFromLightPov.xyz/positionFromLightPov.w * 0.5 + 0.5;
                    float shadowNess = rand(positionFromLightPovInTexture.xy) / 1.6 + 0.4;
                    shadowNess = 0.0;
                    float texelSize = 2.0 / 4096.0;

                    const int blend = 5;

                    float blendLength = float(blend) * 2.0 + 1.0;
                    blendLength = blendLength * blendLength;

                    for (int x = -blend; x <= blend; x++) {

                        for (int y = -blend; y <= blend; y++) {
                            int bigx = 1 * x;
                            int bigy = 1 * y;
                            float depth = texture(shadowMap, positionFromLightPovInTexture.xy + vec2(bigx, bigy) * texelSize).x;

                            // Range for positionFromLightPovInTexture.z is about 0.24 to 0.31
                            //shadow = positionFromLightPovInTexture.z < 0.31;

                            // Range for depthValue is about 0.21 to 0.31
                            // false is black?
                            if (depth < positionFromLightPovInTexture.z) {
                                shadowNess += 1.0;
                            }

                        }
                    }

                    shadowNess /= blendLength;

                    // Diffuse
                    vec3 lightDir = normalize(-(vec3(-3.0, -10.0, 5.0)));
                    vec3 normal = normalize(v_normal);
                    float shade = max(dot(normal, lightDir), 0.0);


                    float combined = ambientLight + 0.6 * shade - 0.2 * shadowNess;
                    float fluidCompensation = 1.0;
                    float noiseCompensation = 1.0;

                    if (u_fluid) {
                        fluidCompensation = animateFluid() * 0.2 + 0.9;
                    }
                    if (u_noise) {
                        noiseCompensation = rand(worldPosition.xy) * 0.2 + 0.9;
                    }
                    fragColor = vec4(u_color.rgb * combined * noiseCompensation, u_color.a * fluidCompensation);
                }
                ";

        let program = glium::Program::from_source(
            self.display,
            vertex_shader_source,
            fragment_shader_source,
            None,
        );
        if program.is_err() {
            panic!("Failed to create program: {}", program.unwrap_err());
        }

        program.unwrap()
    }

    /// Render to the shadow buffer so we can compute shadows.
    pub fn draw_shadow(&mut self, drawable: &impl Drawable, light: Camera) {
        let vertices_buffer =
            glium::VertexBuffer::new(self.display, drawable.vertices().as_slice()).unwrap();
        let indices = glium::index::NoIndices(drawable.primitive_type());

        let eye = light.eye;
        let target = light.target;
        let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());

        let model = Isometry3::new(
            Vector3::from_row_slice(drawable.translation()),
            Vector3::from_row_slice(drawable.rotation()),
        );

        // Compute the matrices
        let projection_matrix = self.build_light_projection();
        let model_view = (view * model).to_homogeneous();
        let model_view_array: [[f32; 4]; 4] = model_view.into();
        let projection_array: [[f32; 4]; 4] = projection_matrix.into();

        let uniforms = uniform! {
          uMVMatrix: model_view_array,
          uPMatrix: projection_array,
        };

        let params = glium::DrawParameters {
            line_width: Some(2.0),
            backface_culling: glium::BackfaceCullingMode::CullCounterClockwise,
            viewport: Some(glium::Rect {
                left: 0,
                bottom: 0,
                width: self.shadow_texture_size,
                height: self.shadow_texture_size,
            }),
            ..Default::default()
        };
        let mut surface = self
            .shadow_depth_texture
            .as_ref()
            .expect("Shadow depth texture not initialized")
            .as_surface();
        surface
            .draw(
                &vertices_buffer,
                &indices,
                self.light_program.as_ref().expect("Shader"),
                &uniforms,
                &params,
            )
            .unwrap();
    }

    /// Render to the actual color buffer.
    pub fn draw(&mut self, drawable: &impl Drawable, camera: Camera, light: Camera, elapsed: f32) {
        let vertices_buffer =
            glium::VertexBuffer::new(self.display, drawable.vertices().as_slice()).unwrap();
        let indices = glium::index::NoIndices(drawable.primitive_type());

        println!("Vertices buffer: {:?}", vertices_buffer.len());
        let color = drawable.color();

        // We need to calculate the model matrix for the drawable object
        let eye = camera.eye;
        let target = camera.target;
        let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());

        let model = Isometry3::new(
            Vector3::from_row_slice(drawable.translation()),
            Vector3::from_row_slice(drawable.rotation()),
        );

        let projection_matrix = self.build_camera_projection();
        let model_view = (view * model).to_homogeneous();
        let model_matrix = model.to_homogeneous();
        let model_view_array: [[f32; 4]; 4] = model_view.into();
        let model_array: [[f32; 4]; 4] = model_matrix.into();
        let projection_array: [[f32; 4]; 4] = projection_matrix.into();
        // Also do these for the light matrices.

        let light_eye = light.eye;
        let light_target = light.target;
        let light_view = Isometry3::look_at_rh(&light_eye, &light_target, &Vector3::y());
        let light_projection_matrix = self.build_light_projection();
        let light_model_view = (light_view * model).to_homogeneous();
        let light_model_view_array: [[f32; 4]; 4] = light_model_view.into();
        let light_projection_array: [[f32; 4]; 4] = light_projection_matrix.into();
        let shadow_texture = self.shadow_depth_texture.as_ref().unwrap();
        let uniforms = uniform! {
          u_color: *color,
          u_fluid: drawable.fluid() != 0,
          u_noise: drawable.noise() != 0,
          u_time: elapsed,
          u_shadow_texture_size:       self.shadow_texture_size,
          uMVMatrix: model_view_array,
          uMMatrix: model_array,
          uPMatrix: projection_array,
          u_light_MVMatrix: light_model_view_array,
          u_light_PMMatrix: light_projection_array,
          shadowMap: shadow_texture
        };

        let params = glium::DrawParameters {
            line_width: Some(2.0),
            blend: glium::Blend::alpha_blending(),
            backface_culling: glium::BackfaceCullingMode::CullCounterClockwise,
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            viewport: Some(glium::Rect {
                left: 0,
                bottom: 0,
                width: self.canvas_width,
                height: self.canvas_height,
            }),
            ..Default::default()
        };
        self.frame
            .draw(
                &vertices_buffer,
                &indices,
                self.camera_program.as_ref().expect("Shader"),
                &uniforms,
                &params,
            )
            .unwrap();

        /*
        let color_location_opt = self
            .gl
            .get_uniform_location(shader.expect("fail"), "u_color");
        if color_location_opt.is_some() {
            self.gl
                .uniform4fv_with_f32_array(color_location_opt.as_ref(), drawable.color());
        }

        let fluid_location_opt = self
            .gl
            .get_uniform_location(shader.expect("fail"), "u_fluid");
        if fluid_location_opt.is_some() {
            self.gl
                .uniform1i(fluid_location_opt.as_ref(), drawable.fluid());
        }

        let noise_location_opt = self
            .gl
            .get_uniform_location(shader.expect("fail"), "u_noise");
        if noise_location_opt.is_some() {
            self.gl
                .uniform1i(noise_location_opt.as_ref(), drawable.noise());
        }

        let time_location_opt = self
            .gl
            .get_uniform_location(shader.expect("fail"), "u_time");

        let world_time = elapsed;
        //log::debug!("Set time to {world_time}");

        if time_location_opt.is_some() {
            self.gl.uniform1f(time_location_opt.as_ref(), world_time);
        }

        let u_shadow_texture_size_location_opt = self
            .gl
            .get_uniform_location(shader.expect("fail"), "u_shadow_texture_size");
        if u_shadow_texture_size_location_opt.is_some() {
            self.gl.uniform1i(
                u_shadow_texture_size_location_opt.as_ref(),
                self.shadow_texture_size,
            );
        }

        // We want a model / view / projection matrix
        // Compute the matrices
        // Our camera looks toward the point (0.0, 0.0, 0.0).
        // It is located at (2.0, 2.0, 2.0).
        let eye = camera.eye;
        let target = camera.target;
        let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());

        // This is translation, rotation
        let model = Isometry3::new(
            Vector3::from_row_slice(drawable.translation()),
            Vector3::from_row_slice(drawable.rotation()),
        );

        let projection_matrix = self.build_camera_projection();
        let model_view = (view * model).to_homogeneous();
        let model_matrix = model.to_homogeneous();
        let u_mv_matrix_location = self
            .gl
            .get_uniform_location(shader.expect("fail"), "uMVMatrix")
            .unwrap();
        self.gl.uniform_matrix4fv_with_f32_array(
            Some(&u_mv_matrix_location),
            false,
            model_view.as_slice(),
        );

        let u_m_matrix_location = self
            .gl
            .get_uniform_location(shader.expect("fail"), "uMMatrix")
            .unwrap();
        self.gl.uniform_matrix4fv_with_f32_array(
            Some(&u_m_matrix_location),
            false,
            model_matrix.as_slice(),
        );

        let u_p_matrix_location = self
            .gl
            .get_uniform_location(shader.expect("fail"), "uPMatrix")
            .unwrap();

        self.gl.uniform_matrix4fv_with_f32_array(
            Some(&u_p_matrix_location),
            false,
            projection_matrix.as_slice(),
        );

        // Repeat these shenanigans for the light matrices.
        let light_eye = light.eye;
        let light_target = light.target;
        let light_view = Isometry3::look_at_rh(&light_eye, &light_target, &Vector3::y());

        // This is translation, rotation
        let light_projection_matrix = self.build_light_projection();
        let light_model_view = (light_view * model).to_homogeneous();

        let u_light_mv_matrix_location = self
            .gl
            .get_uniform_location(shader.expect("fail"), "u_light_MVMatrix");

        if let Some(u) = u_light_mv_matrix_location {
            self.gl
                .uniform_matrix4fv_with_f32_array(Some(&u), false, light_model_view.as_slice());
        }

        let u_light_p_matrix_location = self
            .gl
            .get_uniform_location(shader.expect("fail"), "u_light_PMatrix");

        if let Some(u) = u_light_p_matrix_location {
            self.gl.uniform_matrix4fv_with_f32_array(
                Some(&u),
                false,
                light_projection_matrix.as_slice(),
            );
        }

        self.gl.line_width(2.0);

        let chunk_size: i32 = self.vertex_buffer_limit;
        let upper = ((drawable.count_vertices() as i32) + (chunk_size - 1)) / chunk_size;

        for chunk in 0..upper {
            let count = min(
                chunk_size,
                drawable.count_vertices() as i32 - (chunk * chunk_size),
            );

            let reduced_count = count / 3;
            self.gl.draw_arrays(render_mode, 0, reduced_count);
        }
        self.gl.flush();*/
    }

    /// Prepare to draw the shadow.
    pub fn prepare_shadow_frame(&self) {
        /*self.use_light_shader();

        // Draw to our off screen drawing buffer
        self.gl.bind_framebuffer(
            WebGlRenderingContext::FRAMEBUFFER,
            self.shadow_frame_buffer.as_ref(),
        );

        // Set the viewport to our shadow texture's size
        self.gl
            .viewport(0, 0, self.shadow_texture_size, self.shadow_texture_size);
        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear_depth(1.0);
        self.gl.clear(
            WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT,
        );*/
    }

    /// Complete the shadow drawing.
    pub fn finish_shadow_frame(&self) {
        //self.gl
        //    .bind_framebuffer(WebGlRenderingContext::FRAMEBUFFER, None);
    }

    /// Prepare the camera frame.
    pub fn prepare_camera_frame(&mut self) {
        self.frame.clear_color_and_depth((0.5, 0.5, 0.7, 1.0), 1.0);

        /*self.gl
            .bind_framebuffer(WebGlRenderingContext::FRAMEBUFFER, None);

        self.gl
            .viewport(0, 0, self.canvas_width, self.canvas_height);
        self.gl.clear_color(0.5, 0.5, 0.7, 1.0);
        self.gl.clear_depth(1.0);
        self.gl.clear(
            WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT,
        );
        // Bind the shadow texture
        self.gl.bind_texture(
            WebGlRenderingContext::TEXTURE_2D,
            self.shadow_depth_texture.as_ref(),
        );
        let u_shadow_map = self
            .gl
            .get_uniform_location(self.camera_program.as_ref().expect("Fail"), "shadowMap");
        if u_shadow_map.is_some() {
            self.gl.uniform1i(u_shadow_map.as_ref(), 0);
        }*/
    }

    /// We are done with the camera frame.
    pub fn finish_camera_frame(&self) {}
}
