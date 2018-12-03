#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
extern crate glfw;
use self::glfw::Context;

extern crate gl;

use std::ffi::CStr;

use common::{process_events, processInput};
use shader::Shader;
use camera::Camera;
use model::Model;

use cgmath::{Matrix4, vec3, Point3, Deg, perspective};

// settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main_tree() {
    let mut camera = Camera {
        Position: Point3::new(0.0, 0.0, 3.0),
        ..Camera::default()
    };

    let mut firstMouse = true;
    let mut lastX: f32 = SCR_WIDTH as f32 / 2.0;
    let mut lastY: f32 = SCR_HEIGHT as f32 / 2.0;

    // timing
    let mut deltaTime: f32; // time between current frame and last frame
    let mut lastFrame: f32 = 0.0;

    // glfw: initialize and configure
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // glfw window creation
    let (mut window, events) = glfw.create_window(SCR_WIDTH, SCR_HEIGHT, "tree", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_framebuffer_size_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);

    // tell GLFW to capture our mouse
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    // gl: load all OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let (model_shader, treeModel) = unsafe {
        // configure global opengl state
        gl::Enable(gl::DEPTH_TEST);

        // build and compile shaders
        let model_shader = Shader::new(
            "src/tree/shaders/model_loading.vs",
            "src/tree/shaders/model_loading.fs");

        // load models
        let treeModel = Model::new("resources/tree/DeadTree.obj");
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        (model_shader, treeModel)
    };

    // render loop
    while !window.should_close() {
        // per-frame time logic
        let currentFrame = glfw.get_time() as f32;
        deltaTime = currentFrame - lastFrame;
        lastFrame = currentFrame;

        // events
        process_events(&events, &mut firstMouse, &mut lastX, &mut lastY, &mut camera);

        // input
        processInput(&mut window, deltaTime, &mut camera);

        // render
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            model_shader.useProgram();

            // view/projection transformations
            let projection: Matrix4<f32> = perspective(Deg(camera.Zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32, 0.1, 100.0);
            let view = camera.GetViewMatrix();
            model_shader.setMat4(c_str!("projection"), &projection);
            model_shader.setMat4(c_str!("view"), &view);

            // render the loaded model
            let mut model = Matrix4::<f32>::from_translation(vec3(0.0, -1.75, 0.0)); // translate it down so it's at the center of the scene
            model = model * Matrix4::from_scale(0.2);  // it's a bit too big for our scene, so scale it down
            model_shader.setMat4(c_str!("model"), &model);
            treeModel.Draw(&model_shader);

            let mut model1 = Matrix4::<f32>::from_translation(vec3(1.0, -1.75, 1.0));
            model1 = model1 * Matrix4::from_scale(0.05);
            model_shader.setMat4(c_str!("model"), &model1);
            treeModel.Draw(&model_shader);

            let mut model2 = Matrix4::<f32>::from_translation(vec3(2.0, -1.75, 0.5));
            model2 = model2 * Matrix4::from_scale(0.02);
            model_shader.setMat4(c_str!("model"), &model2);
            treeModel.Draw(&model_shader);

            let mut model3 = Matrix4::<f32>::from_translation(vec3(1.0, -1.75, -1.0));
            model3 = model3 * Matrix4::from_scale(0.05);
            model_shader.setMat4(c_str!("model"), &model3);
            treeModel.Draw(&model_shader);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }
}
