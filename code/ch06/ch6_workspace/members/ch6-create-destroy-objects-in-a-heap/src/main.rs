use ch6_create_destroy_objects_in_a_heap::{World, N_PARTICLES, WHITE};
use piston_window::{ PistonWindow, WindowSettings, clear, rectangle };


fn main() {
    let (width, height) = (640, 480);
    let mut window: PistonWindow = 
                        WindowSettings::new("particles", [width, height])
                        .exit_on_esc(true)
                        .build()
                        .expect("Could not create a window.");
    
    // Initialize
    let mut world = World::new(width, height);
    world.add_shapes(N_PARTICLES);

    while let Some(event) = window.next() { // main loop
        // Update Step
        for shape in &mut world.shapes {
            shape.update();
        }
        world.update();

        // Render Step
        // let function = |Context, &mut G2d, &mut gfx_device_gl| {
        //     clear(WHITE, renderer);
        //     for s in &mut world.shapes {
        //         let rect = [s.position[0], s.position[1], s.width, s.height];
        //         let transformation_matrix = ctx.transform; 
        //         rectangle(s.color, rect, transformation_matrix, renderer); // create a graphics::Rectangle and call draw() on it
        //     }
        // };
        // window.draw_2d(&event, function);
    }
}