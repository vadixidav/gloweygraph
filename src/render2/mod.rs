use glium::{self, Surface};
mod linear;
mod qbezier;

/// Node is used to pass nodes into the renderer.
#[derive(Copy, Clone, RustcDecodable, RustcEncodable, Debug)]
pub struct Node {
    pub position: [f32; 2],
    pub inner_color: [f32; 4],
    /// Decreasing falloff makes the nodes brightness more centered at the middle and increasing it makes it consistent.
    pub falloff: f32,
    pub falloff_color: [f32; 4],
    pub falloff_radius: f32,
    pub inner_radius: f32,
}

implement_vertex!(Node,
                  position,
                  inner_color,
                  falloff,
                  falloff_color,
                  falloff_radius,
                  inner_radius);

/// QBezier is used to pass a quadratic bezier curve into the shader with interpolating values.
#[derive(Copy, Clone, RustcDecodable, RustcEncodable, Debug)]
pub struct QBezier {
    pub position0: [f32; 2],
    pub position1: [f32; 2],
    pub position2: [f32; 2],
    pub inner_color0: [f32; 4],
    pub inner_color1: [f32; 4],
    pub falloff_color0: [f32; 4],
    pub falloff_color1: [f32; 4],
    /// Decreasing falloff makes the nodes brightness more centered at the middle and increasing it makes it consistent.
    pub falloff0: f32,
    pub falloff1: f32,
    pub falloff_radius0: f32,
    pub falloff_radius1: f32,
    pub inner_radius0: f32,
    pub inner_radius1: f32,
}

implement_vertex!(QBezier,
                  position0,
                  position1,
                  position2,
                  inner_color0,
                  inner_color1,
                  falloff0,
                  falloff1,
                  falloff_color0,
                  falloff_color1,
                  falloff_radius0,
                  falloff_radius1,
                  inner_radius0,
                  inner_radius1);

/// A Renderer is tied to the lifetime of the glium Display and making one builds a GLSL program internally.
pub struct Renderer<'a, D>
    where D: 'a
{
    display: &'a D,
    node_program: glium::Program,
    round_edge_program: glium::Program,
    flat_edge_program: glium::Program,
    round_qbezier_program: glium::Program,
    flat_qbezier_program: glium::Program,
    params: glium::DrawParameters<'a>,
}

impl<'a, D> Renderer<'a, D>
    where D: glium::backend::Facade
{
    /// Make a new Renderer from a Facade.
    pub fn new(display: &'a D) -> Self {
        Renderer {
            display: display,
            node_program: glium::Program::from_source(display,
                                                      linear::VSHADER_SOURCE,
                                                      linear::FSHADER_SOURCE,
                                                      Some(linear::NODE_GSHADER_SOURCE))
                .unwrap(),
            round_edge_program:
                glium::Program::from_source(display,
                                            linear::VSHADER_SOURCE,
                                            linear::FSHADER_SOURCE,
                                            Some(linear::ROUND_EDGE_GSHADER_SOURCE))
                .unwrap(),
            flat_edge_program: glium::Program::from_source(display,
                                                           linear::VSHADER_SOURCE,
                                                           linear::FSHADER_SOURCE,
                                                           Some(linear::FLAT_EDGE_GSHADER_SOURCE))
                .unwrap(),
            round_qbezier_program: glium::Program::from_source(display,
                                                               qbezier::VSHADER_SOURCE,
                                                               qbezier::FSHADER_SOURCE,
                                                               Some(qbezier::GSHADER_SOURCE_ROUND))
                .unwrap(),
            flat_qbezier_program: glium::Program::from_source(display,
                                                              qbezier::VSHADER_SOURCE,
                                                              qbezier::FSHADER_SOURCE,
                                                              Some(qbezier::GSHADER_SOURCE_FLAT))
                .unwrap(),
            params: glium::DrawParameters {
                blend: glium::Blend::alpha_blending(),
                ..Default::default()
            },
        }
    }

    /// Take a series of nodes and draw them in parallel on the GPU.
    pub fn render_nodes<S>(&self,
                           target: &mut S,
                           modelview: [[f32; 3]; 3],
                           projection: [[f32; 3]; 3],
                           nodes: &[Node])
        where S: Surface
    {
        let vertex_buffer = glium::VertexBuffer::new(self.display, nodes).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

        let uniforms = uniform! {
            modelview: modelview,
            projection: projection,
        };

        target.draw(&vertex_buffer,
                  &indices,
                  &self.node_program,
                  &uniforms,
                  &self.params)
            .unwrap();
    }

    /// Take a series of lines (edges) and draw them in parallel on the GPU.
    ///
    /// These will have round ends.
    pub fn render_edges_round<S>(&self,
                                 target: &mut S,
                                 modelview: [[f32; 3]; 3],
                                 projection: [[f32; 3]; 3],
                                 edges: &[Node])
        where S: Surface
    {
        let vertex_buffer = glium::VertexBuffer::new(self.display, edges).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);

        let uniforms = uniform! {
            modelview: modelview,
            projection: projection,
        };

        target.draw(&vertex_buffer,
                  &indices,
                  &self.round_edge_program,
                  &uniforms,
                  &self.params)
            .unwrap();
    }

    /// Take a series of lines (edges) and draw them in parallel on the GPU.
    ///
    /// These will have flat ends.
    pub fn render_edges_flat<S>(&self,
                                target: &mut S,
                                modelview: [[f32; 3]; 3],
                                projection: [[f32; 3]; 3],
                                edges: &[Node])
        where S: Surface
    {
        let vertex_buffer = glium::VertexBuffer::new(self.display, edges).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);

        let uniforms = uniform! {
            modelview: modelview,
            projection: projection,
        };

        target.draw(&vertex_buffer,
                  &indices,
                  &self.flat_edge_program,
                  &uniforms,
                  &self.params)
            .unwrap();
    }

    /// Take a series of triangles (quadratic bezier curves) and draw them in parallel on the GPU.
    ///
    /// These will have round ends.
    pub fn render_qbeziers_round<S>(&self,
                                    target: &mut S,
                                    modelview: [[f32; 3]; 3],
                                    projection: [[f32; 3]; 3],
                                    qbeziers: &[QBezier])
        where S: Surface
    {
        let vertex_buffer = glium::VertexBuffer::new(self.display, qbeziers).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

        let uniforms = uniform! {
            modelview: modelview,
            projection: projection,
        };

        target.draw(&vertex_buffer,
                  &indices,
                  &self.round_qbezier_program,
                  &uniforms,
                  &self.params)
            .unwrap();
    }

    /// Take a series of triangles (quadratic bezier curves) and draw them in parallel on the GPU.
    ///
    /// These will have flat ends.
    pub fn render_qbeziers_flat<S>(&self,
                                   target: &mut S,
                                   modelview: [[f32; 3]; 3],
                                   projection: [[f32; 3]; 3],
                                   qbeziers: &[QBezier])
        where S: Surface
    {
        let vertex_buffer = glium::VertexBuffer::new(self.display, qbeziers).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

        let uniforms = uniform! {
            modelview: modelview,
            projection: projection,
        };

        target.draw(&vertex_buffer,
                  &indices,
                  &self.flat_qbezier_program,
                  &uniforms,
                  &self.params)
            .unwrap();
    }
}
