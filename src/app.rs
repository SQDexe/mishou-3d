/*! Core application state and lifecycle management. */

use {
    glam::{
        Vec2,
        Vec3
        },
    egui_winit_vulkano::{
        Gui,
        egui::{
            TopBottomPanel,
            Window,
            SelectableLabel,
            Slider,
            DragValue,
            Layout,
            Align,
            Align2,
            Grid,
            menu::bar as menu_bar
            }
        },
    vulkano::{
        VulkanLibrary,
        VulkanError,
        command_buffer::{
            AutoCommandBufferBuilder,
            SubpassEndInfo,
            CommandBufferUsage,
            allocator::{
                StandardCommandBufferAllocator,
                StandardCommandBufferAllocatorCreateInfo
                }
            },
        pipeline::Pipeline,
        render_pass::Framebuffer,
        shader::{
            EntryPoint,
            },
        sync::GpuFuture
        },
    vulkano_util::{
        context::VulkanoContext,
        renderer::VulkanoWindowRenderer,
        window::VulkanoWindows
        },
    winit::{
        application::ApplicationHandler,
        event::{
            ElementState,
            MouseButton,
            DeviceId,
            DeviceEvent,
            KeyEvent,
            WindowEvent
            },
        event_loop::{
            ControlFlow,
            EventLoop,
            ActiveEventLoop
            },
        keyboard::{
            PhysicalKey,
            KeyCode
            },
        window::{
            CursorGrabMode,
            Fullscreen,
            WindowId
            }
        },
    std::sync::Arc,
    core::error::Error,
    crate::{
        args::SettingsDto,
        camera::Camera,
        consts::{
            config::{
                RENDER_TIMEOUT,
                SLEEP_BIAS_CORRECTION,
                SLIDER_STEP_SIZE,
                FPS_30_VALUE,
                FPS_60_VALUE,
                FPS_120_VALUE
                },
            labels::{
                STYLISED_APP_NAME,
                PKG_VERSION
                },
            ranges::{
                FOV_RANGE_LIMIT,
                PITCH_RANGE_LIMIT,
                CAMERA_SPEED_RANGE_LIMIT,
                CAMERA_SENSITIVITY_RANGE_LIMIT
                }
            },
        error::{
            AppCreateError,
            AppResumeError,
            AppRuntimeError,
            DepthImageResizeError
            },
        log::alert_warning,
        model::ModelManager,
        gui::GuiManager,
        render::{
            RenderMode,
            RenderState,
            fragment_shader_load,
            vertex_shader_load
            },
        state::{
            Clock,
            FpsMode,
            SceneConfig,
            WindowManager
            },
        utils::{
            create_vulkano_config,
            configure_swapchain,
            get_entry_point,
            create_window_descriptor,
            get_icon,
            create_gui_config,
            is_window_drawable,
            create_framebuffer_info,
            create_renderpass_begin_info,
            create_subpass_begin_info,
            create_viewport,
            create_push_constants,
            get_z_plane_ranges
            }
        }
    };



/**
Primary application state encapsulating the Vulkan rendering context, window management, and logic flow.

This structure orchestrates the lifecycle of the 3D viewer, holding both the uninitialised and active graphics states alongside configuration and scene data.
*/
pub struct App {
    /** Global Vulkan instance and device context. */
    vulkano_context: VulkanoContext,
    /** Manager responsible for window surfaces, event loops, and swapchains. */
    windows_context: VulkanoWindows,
    /** Memory allocator dedicated to Vulkan command buffer generation. */
    command_allocator: Arc<StandardCommandBufferAllocator>,
    /** Dynamic rendering state containing pipelines and framebuffers, populated upon window creation. */
    render_state: Option<RenderState>,
    /** Entry point for the vertex processing stage of the graphics pipeline. */
    vertex_shader: EntryPoint,
    /** Entry point for the pixel colouring stage of the graphics pipeline. */
    fragment_shader: EntryPoint,
    /** Spatial controller dictating the user's viewpoint. */
    camera: Camera,
    /** Global visual settings including background and ambient properties. */
    scene_config: SceneConfig,
    /** Manager handling the asynchronous loading and storage of 3D assets. */
    model_manager: ModelManager,
    /** Timekeeping utility used for calculating frame deltas and limiting framerates. */
    clock: Clock,
    /** Utility tracking global window requests, such as closure or fullscreen toggles. */
    window_manager: WindowManager,
    /** Controller for the immediate mode user interface state. */
    gui_manager: Option<GuiManager>
    }

/**
Temporary wrapper holding mutable references to the active application components.

Constructed only when the primary window is initialised and available, allowing safe and structured execution of the rendering loop and event handling.
*/
struct ActiveApp<'a> {
    /** Mutable reference to the global Vulkan context. */
    vulkano_context: &'a mut VulkanoContext,
    /** Mutable reference to the active window renderer and its swapchain. */
    renderer: &'a mut VulkanoWindowRenderer,
    /** Reference to the memory allocator for command buffers. */
    command_allocator: &'a Arc<StandardCommandBufferAllocator>,
    /** Mutable reference to the active rendering pipeline state. */
    render_state: &'a mut RenderState,
    /** Mutable reference to the spatial view controller. */
    camera: &'a mut Camera,
    /** Mutable reference to the global visual settings. */
    scene_config: &'a mut SceneConfig,
    /** Mutable reference to the 3-D asset manager. */
    model_manager: &'a mut ModelManager,
    /** Mutable reference to the frame timekeeper. */
    clock: &'a mut Clock,
    /** Mutable reference to the window state tracker. */
    window_manager: &'a mut WindowManager,
    /** Mutable reference to the user interface controller. */
    gui_manager: &'a mut GuiManager
    }

impl App {
    /**
    Initialises the core application state and Vulkan context using the provided command-line settings.
    
    # Errors
    
    Returns an `AppCreateError` if the Vulkan library initialisation fails or if the logical device creation encounters an issue.
    */
    pub fn new(event_loop: &EventLoop<()>, settings: SettingsDto) -> Result<Self, AppCreateError> {
        let library = VulkanLibrary::new()?;

        let config = create_vulkano_config(&library, event_loop)?;

        let vulkano_context = VulkanoContext::new(config);
        let windows_context = VulkanoWindows::default();

        let command_allocator_info = StandardCommandBufferAllocatorCreateInfo::default();

        let device = vulkano_context.device();

        let command_allocator = Arc::new(StandardCommandBufferAllocator::new(Arc::clone(device), command_allocator_info));

        let vertex_shader = get_entry_point(Arc::clone(device), vertex_shader_load)?;
        let fragment_shader = get_entry_point(Arc::clone(device), fragment_shader_load)?;

        let SettingsDto { camera_config, scene_config, model_path } = settings;

        let camera = Camera::with_config(camera_config);

        let model_manager = model_path.map_or_else(
            ModelManager::default,
            |model_path| ModelManager::from_path(
                model_path.into_boxed_path(),
                Arc::clone(vulkano_context.memory_allocator())
                ) 
            );                

        Ok(Self {
            vulkano_context,
            windows_context,
            command_allocator,
            render_state: None,
            vertex_shader,
            fragment_shader,
            camera,
            scene_config,
            model_manager,
            clock: Clock::default(),
            window_manager: WindowManager::default(),
            gui_manager: None
            })
        }

    /**
    Handles the application resume event by creating the primary window and initialising the rendering surface.
    
    # Errors
    
    Returns an `AppResumeError` if the window renderer or internal graphics pipelines fail to initialise properly.
    */
    fn resume_handler(&mut self, event_loop: &ActiveEventLoop) -> Result<(), AppResumeError> {
        let descriptor = create_window_descriptor();
        
        let window_id = self.windows_context.create_window(event_loop, &self.vulkano_context, &descriptor, configure_swapchain);
        
        let renderer = self.windows_context.get_renderer(window_id)
            .ok_or(AppResumeError::NoRenderer)?;

        #[cfg(windows)]
        renderer.window().set_window_icon(get_icon());

        let render_state = RenderState::new(
            Arc::clone(self.vulkano_context.device()),
            Arc::clone(self.vulkano_context.memory_allocator()),
            renderer,
            self.vertex_shader.clone(),
            self.fragment_shader.clone()
            )?;

        let gui = Gui::new(
            event_loop,
            renderer.surface(),
            self.vulkano_context.graphics_queue().clone(),
            renderer.swapchain_format(),
            create_gui_config()
            );

        let gui_manager = GuiManager::new(gui);

        self.render_state = Some(render_state);
        self.gui_manager = Some(gui_manager);

        Ok(())
        }

    /**
    Passes incoming window events to the graphical user interface manager.
    
    Returns `true` if the event was consumed by the interface elements, indicating it should not affect the underlying 3D scene.
    */
    fn update_gui(&mut self, event: &WindowEvent) -> bool {
        match self.gui_manager {
            Some(ref mut gui_manger) =>
                gui_manger.gui.update(event),
            _ => false
            }
        }

    /** Attempts to construct an `ActiveApp` instance by verifying that all required rendering components are fully initialised. */
    fn active_app(&mut self, window_id: WindowId) -> Option<ActiveApp<'_>> {
        let renderer = self.windows_context.get_renderer_mut(window_id)?;
        let render_state = self.render_state.as_mut()?;
        let gui_manager = self.gui_manager.as_mut()?;

        let active_app = ActiveApp {
            vulkano_context: &mut self.vulkano_context,
            renderer,
            command_allocator: &self.command_allocator,
            render_state,
            camera: &mut self.camera,
            scene_config: &mut self.scene_config,
            model_manager: &mut self.model_manager,
            clock: &mut self.clock,
            window_manager: &mut self.window_manager,
            gui_manager,
            };

        Some(active_app)
        }
    }

impl From<App> for Result<(), Box<dyn Error>> {
    fn from(value: App) -> Self {
        match value.window_manager.into() {
            Some(error) => Err(error),
            None => Ok(())
            }
        }
    }

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.windows_context.primary_window_id().is_some() {
            return;
            }

        if let Err(error) = self.resume_handler(event_loop) {
            self.window_manager.report_error(error);
            }
        }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        let no_gui_update = ! self.update_gui(&event);

        match event {
            WindowEvent::CloseRequested =>
                event_loop.exit(),
            WindowEvent::RedrawRequested if let Some(mut active_app) = self.active_app(window_id) => 
                if let Err(error) = active_app.redraw_handler() {
                    self.window_manager.report_error(error);
                    },
            WindowEvent::Resized(size) if is_window_drawable(size) && let Some(mut active_app) = self.active_app(window_id) =>
                if let Err(error) = active_app.resize(size.into()) {
                    self.window_manager.report_error(error);
                    },
            WindowEvent::KeyboardInput { event, .. } if no_gui_update && let Some(mut active_app) = self.active_app(window_id) => 
                active_app.keyboard_event_handler(event),
            WindowEvent::MouseInput { state, button, .. } if no_gui_update && let Some(mut active_app) = self.active_app(window_id) =>
                active_app.mouse_event_handler(state, button),
            _ => ()
            }
        }

    fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, event: DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta: (x, y) } = event {
            self.camera.rotate_camera(Vec2::new(x as f32, y as f32));
            }
        }
        
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        /* Activated in order to keep the image from freezing. */
        if self.window_manager.should_exit() {
            event_loop.exit();
            return;
            }

        let Some(renderer) = self.windows_context.get_primary_renderer() else {
            return;
            };

        let window = renderer.window();

        let time_since_last_frame = self.clock.time_elapsed();

        match self.window_manager.fps_mode() {
            FpsMode::WaitFor { duration, .. } if time_since_last_frame < *duration => {
                let timeout = *duration - time_since_last_frame;

                if SLEEP_BIAS_CORRECTION < timeout {
                    event_loop.set_control_flow(ControlFlow::wait_duration(timeout - SLEEP_BIAS_CORRECTION));
                } else {
                    event_loop.set_control_flow(ControlFlow::Poll);
                    }
                },
            FpsMode::VSync | FpsMode::Unlimited | FpsMode::WaitFor { .. } => {
                event_loop.set_control_flow(ControlFlow::Poll);
                window.request_redraw();
                }
            }
        }
    }

impl<'a> ActiveApp<'a> {
    /** Switches the primary window between windowed and borderless fullscreen modes. */
    fn toggle_fullscreen(&self) {
        let window = self.renderer.window();
        let fullscreen = window.fullscreen()
            .is_none()
            .then_some(Fullscreen::Borderless(None));

        window.set_fullscreen(fullscreen);
        }
    
    /**
    Recreates the swapchain and depth buffer images to match the new dimensions of the window.
    
    # Errors
    
    Returns a `DepthImageResizeError` if memory allocation for the new depth buffer fails.
    */
    fn resize(&mut self, extent: [u32; 2]) -> Result<(), DepthImageResizeError> {   
        self.renderer.resize();
        self.render_state.try_recreate_depth_image(
            Arc::clone(self.vulkano_context.memory_allocator()),
            extent
            )
        }

    /** Applies any pending framerate mode changes to the active swapchain presentation parameters. */
    fn update_fps_mode(&mut self) {
        let Some(payload) = self.window_manager.fps_mode_request_payload() else {
            return;
            };

        self.renderer.set_present_mode(payload.clone().into());
        self.window_manager.finish_fps_mode_request();
        }
    
    /** Processes raw keyboard input events to update the camera movement state. */
    fn keyboard_event_handler(&mut self, keyboard_event: KeyEvent) {
        let PhysicalKey::Code(key) = keyboard_event.physical_key else {
            return;
            };

        let pressed = keyboard_event.state.is_pressed();

        let camera_inputs = self.camera.inputs_mut();

        match key {
            KeyCode::KeyW | KeyCode::ArrowUp =>
                camera_inputs.set_forward(pressed),
            KeyCode::KeyS | KeyCode::ArrowDown =>
                camera_inputs.set_backward(pressed),
            KeyCode::KeyD | KeyCode::ArrowRight =>
                camera_inputs.set_right(pressed),
            KeyCode::KeyA | KeyCode::ArrowLeft =>
                camera_inputs.set_left(pressed),
            KeyCode::Space =>
                camera_inputs.set_upward(pressed),
            KeyCode::ShiftLeft | KeyCode::ShiftRight =>
                camera_inputs.set_downward(pressed),
            _ => ()
            }
        }

    /** Processes raw mouse button events to control cursor capture and view rotation. */
    fn mouse_event_handler(&mut self, mouse_state: ElementState, mouse_button: MouseButton) {
        if mouse_button != MouseButton::Left {
            return;
            }

        let window = self.renderer.window();

        match mouse_state {
            ElementState::Pressed => {
                if window.set_cursor_grab(CursorGrabMode::Locked).is_err() {
                    _ = window.set_cursor_grab(CursorGrabMode::Confined)
                    }
                
                window.set_cursor_visible(false);
                
                self.camera.set_grabbed();
                },
            ElementState::Released => {
                _ = window.set_cursor_grab(CursorGrabMode::None);
                
                window.set_cursor_visible(true);
                
                self.camera.set_loose();
                }
            }
        }

    /**
    Executes the primary logic and rendering loop for a single frame.
    
    # Errors
    
    Returns an `AppRuntimeError` if drawing commands fail to record, or if the 3D model update encounters an error.
    */
    fn redraw_handler(&mut self) -> Result<(), AppRuntimeError> {
        self.clock.update();
        self.update_fps_mode();

        self.camera.move_camera(self.clock.time_delta());

        self.draw_gui();

        self.draw()?;

        if let Err(error) = self.model_manager.update() {
            alert_warning(&error.to_string());
            };

        if self.window_manager.is_fullscreen_toggle_requested() {
            self.toggle_fullscreen();
            self.window_manager.finish_fullscreen_toggle();
            }

        Ok(())
        }

    /**
    Constructs and submits the command buffers required to render the 3D scene onto the swapchain image.
    
    # Errors
    
    Returns an `AppRuntimeError` if Vulkan encounters a failure during framebuffer creation or command buffer submission.
    */
    fn draw(&mut self) -> Result<(), AppRuntimeError> {
        let window_size = self.renderer.window().inner_size(); // self.renderer.window_size();
        if ! is_window_drawable(window_size) {
            return Ok(());
            }

        let acquire_future = match self.renderer.acquire(Some(RENDER_TIMEOUT), |_| ()) {
            Ok(value) => value,
            Err(VulkanError::OutOfDate) =>
                return Ok(()),
            Err(error) => 
                return Err(error.into())
            };

        let image = Arc::clone(&self.renderer.swapchain_image_view());
        let depth_image = Arc::clone(self.render_state.depth_image());

        let extent = window_size.into();

        let framebuffer = Framebuffer::new(
            Arc::clone(self.render_state.render_pass()),
            create_framebuffer_info(Arc::clone(&image), depth_image)
            )?;

        let graphics_queue = Arc::clone(self.vulkano_context.graphics_queue()); 

        let mut builder = AutoCommandBufferBuilder::primary(
            /* Cast to `Arc<dyn ...>` */
            Arc::clone(self.command_allocator) as _,
            graphics_queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
            )?;

        let render_pass_begin_info = create_renderpass_begin_info(
            self.scene_config.background_colour,
            framebuffer
            );
        let subpass_begin_info = create_subpass_begin_info();

        let viewport = create_viewport(extent);

        let viewports = [viewport].into_iter().collect();

        let push_constants = create_push_constants(self.renderer.aspect_ratio(), self.camera, self.scene_config);

        builder.begin_render_pass(render_pass_begin_info, subpass_begin_info)?
            .set_viewport(0, viewports)?
            .bind_pipeline_graphics(Arc::clone(self.render_state.pipeline()))?
            .push_constants(Arc::clone(self.render_state.pipeline().layout()), 0, push_constants)?;

        if let Some(model) = self.model_manager.model() {
            builder.bind_vertex_buffers(0, model.vertex_buffer().clone())?
                .bind_index_buffer(model.index_buffer().clone())?;

            /*
            SAFETY:
            Invoking draw command is inherently unsafe
            - Indices must be correctly mapped onto the vertecies
            - Shaders must be correct, and can not invoke UBs
            - Parameters of the function must be correct
            */
            unsafe {
                builder.draw_indexed(model.index_buffer().len() as u32, 1, 0, 0, 0)?;
                }
            }

        builder.end_render_pass(SubpassEndInfo::default())?;

        let command_buffer = builder.build()?;

        let render_future = acquire_future.then_execute(graphics_queue, command_buffer)?;
        let gui_future = self.gui_manager.gui.draw_on_image(render_future, image);
        
        self.renderer.present(gui_future.boxed(), true);

        Ok(())
        }

    /** Records the immediate mode graphical user interface elements for the current frame. */
    fn draw_gui(&mut self) {
        self.gui_manager.gui.immediate_ui(|gui| {
            let context = gui.context();

            TopBottomPanel::top("menu-bar").show(&context, |ui| {
                menu_bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Open model ...").clicked() {
                            self.gui_manager.file_dialog.pick_file();
                            }

                        if ui.button("Unload model").clicked() {
                            self.model_manager.unload_model();
                            }
                        
                        ui.separator();
                        
                        if ui.button("Exit").clicked() {
                            self.window_manager.request_exit();
                            }
                        });

                    ui.menu_button("View", |ui| {
                        if ui.button("Toggle fullscreen").clicked() {
                            self.window_manager.request_fullscreen_toggle();
                            }

                        ui.separator();

                        ui.checkbox(&mut self.gui_manager.show_config, "Show configuration");
                        ui.checkbox(&mut self.gui_manager.show_fps, "Show FpS");
                        
                        ui.separator();

                        ui.label("Render mode");
                        ui.selectable_value(self.render_state.render_mode_mut(), RenderMode::Polygone, "Polygone");
                        ui.selectable_value(self.render_state.render_mode_mut(), RenderMode::Wireframe, "Wireframe");

                        ui.separator();

                        ui.label("FpS");
                
                        let fps = self.window_manager.fps_mode().clone();

                        if ui.add(SelectableLabel::new(fps == FpsMode::VSync, "V-Sync")).clicked()
                            { self.window_manager.request_fps_mode(FpsMode::VSync); }
                        if ui.add(SelectableLabel::new(fps == FPS_30_VALUE, "30")).clicked()
                            { self.window_manager.request_fps_mode(FpsMode::from_fps(FPS_30_VALUE)); }
                        if ui.add(SelectableLabel::new(fps == FPS_60_VALUE, "60")).clicked()
                            { self.window_manager.request_fps_mode(FpsMode::from_fps(FPS_60_VALUE)); }
                        if ui.add(SelectableLabel::new(fps == FPS_120_VALUE, "120")).clicked()
                            { self.window_manager.request_fps_mode(FpsMode::from_fps(FPS_120_VALUE)); }
                        if ui.add(SelectableLabel::new(fps == FpsMode::Unlimited, "Unlimited")).clicked()
                            { self.window_manager.request_fps_mode(FpsMode::Unlimited); }

                        ui.separator();
                        
                        if ui.button("Reset configuration").clicked() {
                            self.scene_config.reset();
                            self.camera.reset();
                            }
                        });

                    ui.menu_button("Help", |ui| {
                        if ui.button("About").clicked() {
                            self.gui_manager.show_about = true;
                            }
                        });


                    if self.gui_manager.show_fps {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {                        
                            ui.label(format!("FpS: {}", self.clock.fps()));
                            });
                        }
                    });
                });

            if self.gui_manager.show_config {
                let SceneConfig {
                    background_colour,
                    model_colour,
                    light_direction,
                    .. } = self.scene_config;

                let camera = &mut self.camera;

                Window::new("Settings").collapsible(false).resizable(false).default_width(250.0).show(&context, |ui| {
                    ui.heading("Background");
                    ui.horizontal(|ui| {
                        ui.label("Color:");
                        let mut colour = background_colour.to_array();
                        if ui.color_edit_button_rgb(&mut colour).changed() {
                            *background_colour = colour.into();
                            }
                        });

                    ui.separator();

                    ui.heading("Model");
                    ui.horizontal(|ui| {
                        ui.label("Color:");
                        let mut colour = model_colour.to_array();
                        if ui.color_edit_button_rgb(&mut colour).changed() {
                            *model_colour = colour.into();
                            }
                        });

                    // ui.horizontal(|ui| {
                    //     ui.label("Rotation speed:");
                    //     ui.add(DragValue::new(rotation_speed).speed(SLIDER_STEP_SIZE));
                    //     });

                    ui.separator();

                    ui.heading("Lighting");
                    ui.label("Light direction:");
                    ui.horizontal(|ui| {
                        ui.add(DragValue::new(&mut light_direction.x).fixed_decimals(3).speed(SLIDER_STEP_SIZE).prefix("Yaw: ").suffix("°"));
                        ui.add(DragValue::new(&mut light_direction.y).fixed_decimals(3).speed(SLIDER_STEP_SIZE).prefix("Pitch: ").suffix("°"));
                        });

                    ui.separator();

                    ui.heading("Camera");
                    
                    ui.label("Position:");
                    ui.horizontal(|ui| {
                        let Vec3 { ref mut x, ref mut y, ref mut z } = camera.position;
                        ui.add(DragValue::new(x).fixed_decimals(3).speed(SLIDER_STEP_SIZE).prefix("X: "));
                        ui.add(DragValue::new(y).fixed_decimals(3).speed(SLIDER_STEP_SIZE).prefix("Y: "));
                        ui.add(DragValue::new(z).fixed_decimals(3).speed(SLIDER_STEP_SIZE).prefix("Z: "));
                        });

                    ui.label("Rotation:");
                    ui.horizontal(|ui| {
                        let Vec2 { ref mut x, ref mut y } = camera.rotation;
                        ui.add(DragValue::new(x).speed(SLIDER_STEP_SIZE).fixed_decimals(3).prefix("Yaw: ").suffix("°"));
                        ui.add(DragValue::new(y).speed(SLIDER_STEP_SIZE).fixed_decimals(3).range(PITCH_RANGE_LIMIT).prefix("Pitch: ").suffix("°"));
                        });

                    ui.add(Slider::new(&mut camera.speed, CAMERA_SPEED_RANGE_LIMIT).text("Speed"));

                    ui.add(Slider::new(&mut camera.sensitivity, CAMERA_SENSITIVITY_RANGE_LIMIT).text("Sensitivity"));

                    ui.add(Slider::new(&mut camera.fov, FOV_RANGE_LIMIT).text("FoV"));

                    ui. horizontal(|ui| {
                        let (near_range, far_range) = get_z_plane_ranges(camera.z_near, camera.z_far);
                        ui.add(DragValue::new(&mut camera.z_near).speed(SLIDER_STEP_SIZE).fixed_decimals(3).range(near_range).prefix("Z-Near: "));
                        ui.add(DragValue::new(&mut camera.z_far).speed(SLIDER_STEP_SIZE).fixed_decimals(3).range(far_range).prefix("Z-Far: "));
                        });
                    });

                }

            if self.model_manager.is_loading() {
                Window::new("Loading the model ...").collapsible(false).resizable(false).default_width(200.0).anchor(Align2::CENTER_CENTER, [0.0, 0.0]).show(&context, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.spinner();
                        if ui.button("Cancel").clicked() {
                            self.model_manager.cancel_model_load();
                            }
                        });
                    });
                }

            if self.gui_manager.show_about {
                Window::new("About").collapsible(false).resizable(false).anchor(Align2::CENTER_CENTER, [0.0, 0.0]).show(&context, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading(STYLISED_APP_NAME);
                        ui.small(PKG_VERSION);

                        ui.separator();

                        ui.label("A lightweight and efficient 3D model viewer powered by Rust and Vulkan.");

                        ui.separator();

                        ui.heading("Controls");
                        
                        Grid::new("about_controls_grid").num_columns(2).spacing([24.0, 8.0]).show(ui, |ui| {
                            ui.label("W / A / S / D / Arrows");
                            ui.label("Move camera horizontally");
                            ui.end_row();
                            
                            ui.label("Space / Shift");
                            ui.label("Move camera vertically");
                            ui.end_row();
                            
                            ui.label("Left Mouse Button");
                            ui.label("Capture cursor and rotate view");
                            ui.end_row();
                            });

                        ui.separator();

                        if ui.button("Close").clicked() {
                            self.gui_manager.show_about = false;
                            }
                        });
                    });
                }
            
            /* Update the file dialog window modal */
            self.gui_manager.file_dialog.update(&context);

            /* File check must be placed after file dialog context update */
            if let Some(model_path) = self.gui_manager.file_dialog.take_picked() {
                _ = self.model_manager.request_model_load(
                    model_path.into_boxed_path(),
                    Arc::clone(self.vulkano_context.memory_allocator())
                    );
                }
            });
        }
    }