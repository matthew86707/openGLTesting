#[macro_use]
extern crate glium;
extern crate image;
#[derive(Copy, Clone)]
	struct Vertex {
   	 position: [f32; 2],
   	 color: [f32; 3],
   	 texCoords: [f32; 2]
	}

fn main() {

	let mut t : f32 = 0.0;

	use std::io::Cursor;
	use glium::{glutin, Surface};
    use glium::glutin::{VirtualKeyCode};
    use std::fs::File;
    use std::io::{Read};

    let mut bytes : Vec<u8>;
    bytes = Vec::new();
    let mut file = (File::open("Rust-Logo.png")).unwrap();
    file.read_to_end(&mut bytes).unwrap();

    let image = image::load(Cursor::new(&bytes),
                        image::PNG).unwrap().to_rgba();
	let image_dimensions = image.dimensions();
	let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new();
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let screen_size = display.get_framebuffer_dimensions();


	let texture = glium::texture::Texture2d::new(&display, image).unwrap();

	let vertex_shader_src = r#"
    #version 330

    in vec2 position;
    in vec3 color;
    out vec3 colorPassed;
    in vec2 texCoords;
    out vec2 texCoords_v;

    uniform float shiftX;
    uniform float shiftY;

    void main() {
    	texCoords_v = texCoords;
        gl_Position = vec4(((position.x / 2) + shiftX), ((position.y / 2) + shiftY), 0.0, 1.0);
        colorPassed = color;
    }
"#;

let fragment_shader_src = r#"
    #version 330

    uniform vec2 screenSize;

    in vec2 texCoords_v;
    uniform float t;
    uniform float b;
    in vec3 colorPassed;
    out vec4 fragColor;

    uniform sampler2D tex;

    void main() {

    	vec2 newTexCoords = (gl_FragCoord.xy / screenSize);
    	vec2 scaledTexCoords;
    	scaledTexCoords.x = sin(newTexCoords.x * 3) * t/1000;
    	scaledTexCoords.y = sin(newTexCoords.y * 3) * t/1000;

    	fragColor = texture(tex, scaledTexCoords);

        //float ts = t / 2000;

        //float divisionsX = 200.0f - t / 200;
        //float divisionsY = 200.0f - t / 200;

        //float modXFracPiA = (mod(gl_FragCoord.x, divisionsX) / divisionsX) * 3.14;
        //float modYFracPiA = (mod(gl_FragCoord.y, divisionsY) / divisionsY) * 3.14;

        //float modXFracPiB = (mod(gl_FragCoord.x + divisionsX / 2, divisionsX) / divisionsX) * 3.14;
        // modYFracPiB = (mod(gl_FragCoord.y + divisionsY / 2, divisionsY) / divisionsY) * 3.14;

      	//fragColor = vec4((sin(ts) / 1.57) * (sin(modXFracPiA) / 1.57) + ((-sin(ts) / 1.57) * (sin(modYFracPiB) / 1.57)), ((sin(ts) / 1.57) * (sin(modXFracPiB) / 1.57)) + ((-sin(ts) / 1.57) * (sin(modYFracPiA) / 1.57)), b, 1.0);
    }
"#;

let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

	implement_vertex!(Vertex, position, color, texCoords);

	let vertex1 = Vertex { position: [-1.0, -1.0], color : [1.0, 1.0, 1.0], texCoords : [0.0, 0.0]};
	let vertex2 = Vertex { position: [ -1.0,  1.0], color : [1.0, 1.0, 1.0], texCoords : [0.0, 1.0]};
	let vertex3 = Vertex { position: [ 1.0, -1.0], color : [1.0, 1.0, 1.0], texCoords : [1.0, 0.0]};

	let vertex4 = Vertex { position: [1.0, 1.0], color : [1.0, 1.0, 1.0], texCoords : [1.0, 1.0]};
	let vertex5 = Vertex { position: [1.0,  -1.0], color : [1.0, 1.0, 1.0], texCoords : [1.0, 0.0]};
	let vertex6 = Vertex { position: [ -1.0, 1.0], color : [1.0, 1.0, 1.0], texCoords : [0.0, 1.0]};

	let shape = vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];

	let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
	let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let mut blue_val : f32 = 0.5;

    let mut x_shift : f32 = 0.0;
    let mut y_shift : f32 = 0.0;

    let mut closed = false;
    while !closed {
    	t += 3.0;
        let mut target = display.draw();
        let mut red_value : f32 = 0.0;
        target.clear_color(red_value, 0.2, 0.0, 1.0);
         target.draw(&vertex_buffer, &indices, &program, &uniform! { t: t , b : blue_val, tex : &texture, screenSize : (screen_size.0 as f32, screen_size.1 as f32), shiftX : x_shift, shiftY : y_shift},
            &Default::default()).unwrap();
        target.finish().unwrap();


        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => closed = true,
                    glutin::WindowEvent::KeyboardInput { input, .. } =>  match input.virtual_keycode { 
                    	Some(VirtualKeyCode::Escape) => {closed = true;}, 
                        Some(VirtualKeyCode::Q) => {blue_val = blue_val + 0.1},
                        Some(VirtualKeyCode::E) => {blue_val = blue_val - 0.1},
                        Some(VirtualKeyCode::A) => {x_shift = x_shift - 0.1},
                        Some(VirtualKeyCode::D) => {x_shift = x_shift + 0.1},
                        Some(VirtualKeyCode::S) => {y_shift = y_shift - 0.1},
                        Some(VirtualKeyCode::W) => {y_shift = y_shift + 0.1}
                    	_ => () },
                  	_ => (),
                },
                
                _ => (),
            }
        });
    
    }
}