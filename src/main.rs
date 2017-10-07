#[macro_use]
extern crate glium;
#[derive(Copy, Clone)]
	struct Vertex {
   	 position: [f32; 2],
   	 color: [f32; 3]
	}

fn main() {

	let mut t : f32 = 0.0;

	use glium::{glutin, Surface};
    use glium::glutin::{VirtualKeyCode};

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new();
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

	let vertex_shader_src = r#"
    #version 330

    in vec2 position;
    in vec3 color;
    out vec3 colorPassed;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
        colorPassed = color;
    }
"#;

let fragment_shader_src = r#"
    #version 330

    uniform float t;
    uniform float b;
    in vec3 colorPassed;
    out vec4 fragColor;

    void main() {
        float ts = t / 2000;

        float divisionsX = 200.0f - t / 200;
        float divisionsY = 200.0f - t / 200;

        float modXFracPiA = (mod(gl_FragCoord.x, divisionsX) / divisionsX) * 3.14;
        float modYFracPiA = (mod(gl_FragCoord.y, divisionsY) / divisionsY) * 3.14;

        float modXFracPiB = (mod(gl_FragCoord.x + divisionsX / 2, divisionsX) / divisionsX) * 3.14;
        float modYFracPiB = (mod(gl_FragCoord.y + divisionsY / 2, divisionsY) / divisionsY) * 3.14;

      	fragColor = vec4((sin(ts) / 1.57) * (sin(modXFracPiA) / 1.57) + ((-sin(ts) / 1.57) * (sin(modYFracPiB) / 1.57)), ((sin(ts) / 1.57) * (sin(modXFracPiB) / 1.57)) + ((-sin(ts) / 1.57) * (sin(modYFracPiA) / 1.57)), b, 1.0);
    }
"#;

let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

	implement_vertex!(Vertex, position, color);

	let vertex1 = Vertex { position: [-1.0, -1.0], color : [1.0, 1.0, 1.0]};
	let vertex2 = Vertex { position: [ -1.0,  1.0], color : [1.0, 1.0, 1.0]};
	let vertex3 = Vertex { position: [ 1.0, -1.0], color : [1.0, 1.0, 1.0]};

	let vertex4 = Vertex { position: [1.0, 1.0], color : [1.0, 1.0, 1.0]};
	let vertex5 = Vertex { position: [1.0,  -1.0], color : [1.0, 1.0, 1.0]};
	let vertex6 = Vertex { position: [ -1.0, 1.0], color : [1.0, 1.0, 1.0]};

	let shape = vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];

	let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
	let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let mut blueVal : f32 = 0.5;

    let mut closed = false;
    while !closed {
    	t += 0.16;
        let mut target = display.draw();
        let mut red_value : f32 = 0.0;
        target.clear_color(red_value, 0.2, 0.0, 1.0);
         target.draw(&vertex_buffer, &indices, &program, &uniform! { t: t , b : blueVal},
            &Default::default()).unwrap();
        target.finish().unwrap();


        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => closed = true,
                    glutin::WindowEvent::KeyboardInput { input, .. } =>  match input.virtual_keycode { 
                    	Some(VirtualKeyCode::Escape) => {closed = true;}, 
                    	Some(VirtualKeyCode::A) => {t = 0.0;},
                        Some(VirtualKeyCode::Q) => {blueVal = blueVal + 0.1},
                        Some(VirtualKeyCode::E) => {blueVal = blueVal - 0.1},
                    	_ => () },
                  	_ => (),
                },
                
                _ => (),
            }
        });
    
    }
}