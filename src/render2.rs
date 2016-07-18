use glium::{self, Surface};

static VSHADER_SOURCE: &'static str = r#"
    #version 150
    in vec2 position;
    in vec4 inner_color;
    in vec4 falloff_color;
    in float falloff;
    in float falloff_radius;
    in float inner_radius;
    out vec4 ginner_color;
    out vec4 gfalloff_color;
    out float gfalloff;
    out float gfalloff_radius;
    out float ginner_radius;
    void main() {
        ginner_color = inner_color;
        gfalloff_color = falloff_color;
        gfalloff = falloff;
        gfalloff_radius = falloff_radius;
        ginner_radius = inner_radius;
        gl_Position = vec4(position, 0.0, 1.0);
    }
"#;

static NODE_GSHADER_SOURCE: &'static str = r#"
    #version 150

    layout(points) in;
    layout(triangle_strip, max_vertices = 3) out;

    in vec4 ginner_color[1];
    in vec4 gfalloff_color[1];
    in float gfalloff[1];
    in float gfalloff_radius[1];
    in float ginner_radius[1];
    out vec2 delta;
    out vec4 finner_color;
    out vec4 ffalloff_color;
    out float finner_radius;
    out float ffalloff_radius;
    out float ffalloff;

    void main() {
        finner_color = ginner_color[0];
        ffalloff_color = ginner_color[0];
        finner_radius = ginner_radius[0];
        ffalloff = gfalloff[0];
        ffalloff_radius = gfalloff_radius[0];
        vec2 center = gl_in[0].gl_Position.xy;
        float full_radius = finner_radius + ffalloff_radius;

        delta = full_radius * vec2(0, 2);
        gl_Position = vec4(center + delta, 0.0, 1.0);
        EmitVertex();

        delta = full_radius * vec2(-1.7320508075689, -1);
        gl_Position = vec4(center + delta, 0.0, 1.0);
        EmitVertex();

        delta = full_radius * vec2(1.7320508075689, -1);
        gl_Position = vec4(center + delta, 0.0, 1.0);
        EmitVertex();
    }
"#;

static EDGE_GSHADER_SOURCE: &'static str = r#"
    #version 150

    layout(lines) in;
    layout(triangle_strip, max_vertices = 12) out;

    in vec4 ginner_color[2];
    in vec4 gfalloff_color[2];
    in float gfalloff[2];
    in float gfalloff_radius[2];
    in float ginner_radius[2];
    out vec2 delta;
    out vec4 finner_color;
    out vec4 ffalloff_color;
    out float finner_radius;
    out float ffalloff_radius;
    out float ffalloff;

    void main() {
        vec2 first = gl_in[0].gl_Position.xy;
        vec2 second = gl_in[1].gl_Position.xy;

        vec2 net_delta = 2 * normalize(second - first);

        float radius;

        //Face 0

        //Vertex 0
        finner_color = ginner_color[0];
        ffalloff_color = gfalloff_color[0];
        finner_radius = ginner_radius[0];
        ffalloff_radius = gfalloff_radius[0];
        ffalloff = gfalloff[0];
        radius = finner_radius + ffalloff_radius;
        delta = radius * vec2(net_delta.y, -net_delta.x);
        gl_Position = vec4(first - delta, 0.0, 1.0);
        EmitVertex();

        //Vertex 1
        finner_color = ginner_color[0];
        ffalloff_color = gfalloff_color[0];
        finner_radius = ginner_radius[0];
        ffalloff_radius = gfalloff_radius[0];
        ffalloff = gfalloff[0];
        radius = finner_radius + ffalloff_radius;
        delta = radius * net_delta;
        gl_Position = vec4(first - delta, 0.0, 1.0);
        EmitVertex();

        //Vertex 2
        finner_color = ginner_color[0];
        ffalloff_color = gfalloff_color[0];
        finner_radius = ginner_radius[0];
        ffalloff_radius = gfalloff_radius[0];
        ffalloff = gfalloff[0];
        radius = finner_radius + ffalloff_radius;
        delta = radius * vec2(-net_delta.y, net_delta.x);
        gl_Position = vec4(first - delta, 0.0, 1.0);
        EmitVertex();

        EndPrimitive();

        //Face 1

        //Vertex 0
        finner_color = ginner_color[0];
        ffalloff_color = gfalloff_color[0];
        finner_radius = ginner_radius[0];
        ffalloff_radius = gfalloff_radius[0];
        ffalloff = gfalloff[0];
        radius = finner_radius + ffalloff_radius;
        delta = radius * vec2(net_delta.y, -net_delta.x);
        gl_Position = vec4(first - delta, 0.0, 1.0);
        EmitVertex();

        //Vertex 2
        finner_color = ginner_color[0];
        ffalloff_color = gfalloff_color[0];
        finner_radius = ginner_radius[0];
        ffalloff_radius = gfalloff_radius[0];
        ffalloff = gfalloff[0];
        radius = finner_radius + ffalloff_radius;
        delta = radius * vec2(-net_delta.y, net_delta.x);
        gl_Position = vec4(first - delta, 0.0, 1.0);
        EmitVertex();

        //Vertex 3
        finner_color = ginner_color[1];
        ffalloff_color = gfalloff_color[1];
        finner_radius = ginner_radius[1];
        ffalloff_radius = gfalloff_radius[1];
        ffalloff = gfalloff[1];
        radius = finner_radius + ffalloff_radius;
        delta = radius * vec2(net_delta.y, -net_delta.x);
        gl_Position = vec4(second - delta, 0.0, 1.0);
        EmitVertex();

        EndPrimitive();

        //Face 2

        //Vertex 2
        finner_color = ginner_color[0];
        ffalloff_color = gfalloff_color[0];
        finner_radius = ginner_radius[0];
        ffalloff_radius = gfalloff_radius[0];
        ffalloff = gfalloff[0];
        radius = finner_radius + ffalloff_radius;
        delta = radius * vec2(-net_delta.y, net_delta.x);
        gl_Position = vec4(first - delta, 0.0, 1.0);
        EmitVertex();

        //Vertex 4
        finner_color = ginner_color[1];
        ffalloff_color = gfalloff_color[1];
        finner_radius = ginner_radius[1];
        ffalloff_radius = gfalloff_radius[1];
        ffalloff = gfalloff[1];
        radius = finner_radius + ffalloff_radius;
        delta = radius * vec2(-net_delta.y, net_delta.x);
        gl_Position = vec4(second - delta, 0.0, 1.0);
        EmitVertex();

        //Vertex 3
        finner_color = ginner_color[1];
        ffalloff_color = gfalloff_color[1];
        finner_radius = ginner_radius[1];
        ffalloff_radius = gfalloff_radius[1];
        ffalloff = gfalloff[1];
        radius = finner_radius + ffalloff_radius;
        delta = radius * vec2(net_delta.y, -net_delta.x);
        gl_Position = vec4(second - delta, 0.0, 1.0);
        EmitVertex();

        EndPrimitive();

        //Face 3

        //Vertex 5
        finner_color = ginner_color[1];
        ffalloff_color = gfalloff_color[1];
        finner_radius = ginner_radius[1];
        ffalloff_radius = gfalloff_radius[1];
        ffalloff = gfalloff[1];
        radius = finner_radius + ffalloff_radius;
        delta = radius * net_delta;
        gl_Position = vec4(second + delta, 0.0, 1.0);
        EmitVertex();

        //Vertex 3
        finner_color = ginner_color[1];
        ffalloff_color = gfalloff_color[1];
        finner_radius = ginner_radius[1];
        ffalloff_radius = gfalloff_radius[1];
        ffalloff = gfalloff[1];
        radius = finner_radius + ffalloff_radius;
        delta = radius * vec2(net_delta.y, -net_delta.x);
        gl_Position = vec4(second - delta, 0.0, 1.0);
        EmitVertex();

        //Vertex 4
        finner_color = ginner_color[1];
        ffalloff_color = gfalloff_color[1];
        finner_radius = ginner_radius[1];
        ffalloff_radius = gfalloff_radius[1];
        ffalloff = gfalloff[1];
        radius = finner_radius + ffalloff_radius;
        delta = radius * vec2(-net_delta.y, net_delta.x);
        gl_Position = vec4(second - delta, 0.0, 1.0);
        EmitVertex();

        EndPrimitive();
    }
"#;

static FSHADER_SOURCE: &'static str = r#"
    #version 150
    in vec2 delta;
    in vec4 finner_color;
    in vec4 ffalloff_color;
    in float finner_radius;
    in float ffalloff_radius;
    in float ffalloff;
    out vec4 color;
    void main() {
        float length = length(delta);
        if (length <= finner_radius) {
            float travel = length / finner_radius;
            // Manually interpolate the inner color into the falloff color.
            color = finner_color * (1.0 - travel) + ffalloff_color * travel;
        } else {
            color = vec4(ffalloff_color.xyz,
                ffalloff_color.a * max(0.0, 1.0 - pow((length - finner_radius) / ffalloff_radius, ffalloff)));
        }
    }
"#;

/// Node is used to pass nodes into the renderer.
#[derive(Copy, Clone)]
pub struct Node2 {
    pub position: [f32; 2],
    pub inner_color: [f32; 4],
    /// Decreasing falloff makes the nodes brightness more centered at the middle and increasing it makes it consistent.
    pub falloff: f32,
    pub falloff_color: [f32; 4],
    pub falloff_radius: f32,
    pub inner_radius: f32,
}

implement_vertex!(Node2,
                  position,
                  inner_color,
                  falloff,
                  falloff_color,
                  falloff_radius,
                  inner_radius);

pub struct Renderer2<'a> {
    display: &'a glium::Display,
    node_program: glium::Program,
    edge_program: glium::Program,
    params: glium::DrawParameters<'a>,
}

/// A Renderer is tied to the lifetime of the glium Display and making one builds a GLSL program internally.
impl<'a> Renderer2<'a> {
    /// Make a new Renderer from a glium::Display.
    pub fn new(display: &'a glium::Display) -> Self {
        Renderer2 {
            display: display,
            node_program: glium::Program::from_source(display,
                                                      VSHADER_SOURCE,
                                                      FSHADER_SOURCE,
                                                      Some(NODE_GSHADER_SOURCE))
                .unwrap(),
            edge_program: glium::Program::from_source(display,
                                                      VSHADER_SOURCE,
                                                      FSHADER_SOURCE,
                                                      Some(EDGE_GSHADER_SOURCE))
                .unwrap(),
            params: glium::DrawParameters {
                blend: glium::Blend::alpha_blending(),
                ..Default::default()
            },
        }
    }

    /// Take a series of nodes and draw them in parallel on the GPU.
    pub fn render_nodes<S>(&self, target: &mut S, nodes: &[Node2])
        where S: Surface
    {
        let vertex_buffer = glium::VertexBuffer::new(self.display, nodes).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

        target.draw(&vertex_buffer,
                  &indices,
                  &self.node_program,
                  &glium::uniforms::EmptyUniforms,
                  &self.params)
            .unwrap();
    }

    /// Take a series of lines (edges) and draw them in parallel on the GPU.
    pub fn render_edges<S>(&self, target: &mut S, edges: &[Node2])
        where S: Surface
    {
        let vertex_buffer = glium::VertexBuffer::new(self.display, edges).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);

        target.draw(&vertex_buffer,
                  &indices,
                  &self.edge_program,
                  &glium::uniforms::EmptyUniforms,
                  &self.params)
            .unwrap();
    }
}
