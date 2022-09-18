use core::panic;
mod shaders;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

macro_rules! log {
    ( $( $t:tt )* ) => {
        unsafe {
            web_sys::console::log_1(&format!( $( $t )* ).into());
        }
    }
}

struct UniformLocations {
    projectionMatrix: wasm_bindgen::JsValue, 
    modelViewMatrix: wasm_bindgen::JsValue
}

struct AttributeLocation {
    vertexPosition: i32,
}

struct ProgramInfo {
    program: WebGlProgram,
    attrib_locations: AttributeLocation,
    uniform_locations: UniformLocations

}

impl ProgramInfo {
    pub fn new(context: WebGl2RenderingContext, program:WebGlProgram ) {
        let attrib_location = AttributeLocation { 
            vertexPosition: context.get_attrib_location(&program, "aVertexPosition") 
        }
        let uniform_locations = UniformLocations {
            projectionMatrix: context.get_active_uniforms(&program, context.get_uniform_block_index(&program, uniform_block_name), pname)
        }
        ProgramInfo {
            program,
            attrib_locations: AttributeLocation { vertexPosition: context.get_attrib_location(program, name) }
        }
    }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window()
        .unwrap_or_else(|| panic!("can't load window"))
        .document()
        .unwrap_or_else(|| panic!("can't load document"));
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap_or_else(|| panic!("can't load canvas"));
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let program = shaders::link_program(
        &context,
        vec![
            shaders::get_vert_shader(&context)?,
            shaders::get_frag_shader(&context)?,
        ],
    )?;
    


    context.use_program(Some(&program));
    context.get_attrib_location(program, name)
    let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

    let position_attribute_location = context.get_attrib_location(&program, "position");
    let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe {
        let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &positions_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    let vao = context
        .create_vertex_array()
        .ok_or("Could not create vertex array object")?;
    context.bind_vertex_array(Some(&vao));

    context.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(position_attribute_location as u32);

    context.bind_vertex_array(Some(&vao));

    let vert_count = (vertices.len() / 3) as i32;
    draw(&context, vert_count);

    Ok(())
}

fn draw(context: &WebGl2RenderingContext, vert_count: i32) {
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
}
