use clap::ValueEnum;



/**
Rendering mode selection for the scene.

This parameter determines the visual representation style of the models drawn on the screen.
*/
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum)]
pub enum RenderMode {
    /** Render objects using solid, filled polygons. */
    #[default]
    Polygone,
    /** Render objects using only their outlined edges. */
    Wireframe
    }