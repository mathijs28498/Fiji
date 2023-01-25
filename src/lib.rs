#![allow(unused)]

mod input;
mod public;
mod rendering;

pub use crate::{
    input::{fiji_events::*, input_enums::*, *},
    public::{
        context::*,
        datatypes::*,
        objects::{background::*, camera::camera_2d::*, obj_2d::circle::*, *},
    },
};

// TODO IN PROGRESS:
// [ ] - Fix error when window size == 0
// [ ] - Recreate pipelines when window out of date
// [ ] - Add image object
//
// LATER TODO:
// [ ] - Keep render objects alive as long as their counter part is alive
//      [ ] - Keep a list of objects which can be set to active/inactive
// [ ] - Make better/less verbose abstraction for objects/render objects/pipelines
// [ ] - Add sphere object
// [ ] - Window options
//      [ ] - Capture mouse option
//      [ ] - Unresizable option
// [ ] - Check all pub modifiers to see if they should be private
// [ ] - Make text position 0, 0 in center
// [ ] - anti aliasing
//      [ ] - aa 2D
//      [ ] - aa 3D
// [ ] - Better/more camera options
// [ ] - Configurable lights in context
// [ ] - Figure out rotation
// [ ] - Render and update closures
// [ ] - Working borders
// [ ] - Line renderer
// [ ] - Borders around 2D objects
// [ ] - Shadows
// [ ] - Figure out a better drawing strategy
// [ ] - Add generic to buffercontainer (based on vertex) in stead of different container structs
// [ ] - Create a better way to store fonts
// [ ] - Better way to store all fonts
// [ ] - Font and fontsize included in key for text_pipeline set hashmap
// [ ] - A container that holds all fonts and font buffer containers with metrics
// [ ] - Replace the unsafe static BUFFERS from the render objects
// [ ] - New lines for text (using y offset)
// [ ] - Handle recreating the pipelines better
// [ ] - Use proper error handling rather than unwrap
// [ ] - 2D element alignment options (text most important)
// [ ] - 3D element alignment options
// [ ] - Add 2D camera option to all pipelines
// [ ] - Fix text drawing performance!
// [ ] - Unreverse camera position
//
// DONE:
// [X] - Delta time
// [X] - Make all fiji public objects accessible directly from lib.rs
// [X] - Camera 2d for all 2d pipelines
// [X] - Depth testing
// [X] - Normals
// [X] - Phong shading
// [X] - Backface culling
// [X] - Split objects into 2D and 3D and draw 2D always on top
// [X] - Builder pattern for block
// [X] - Update to 0.32.0
// [X] - Wrap draw objects in other object with buffer info and draw implementation when in context (look at polygon buffers)
// [X] - Queues for draw objects in stead of Vecs
// [X] - Render pass container
// [X] - Refactor the entire fucking project to abstract all render components
// [X] - Exchange loose buffers with bufferContainer2D/3D for renderpass draws
// [X] - Create RenderPass buffers when initializing new()
// [X] - Move "create_push_constants" to the respective RenderObjects
// [X] - 2D camera
// [X] - Separation 2D between UI without camera and non UI with camera
//          [X] - Special UI queue for DrawObject2D
// [X] - Builder pattern for all objects
// [X] - Rename render pass structs/file names
// [X] - Resizing of window
// [X] - Make use of static viewports
// [X] - Add text components
//      [X] - Figure out how to create R8_UINT image and sent it to the gpu
//      [X] - Create hashmap for character textures (in render pass)
//      [X] - Either get character from hashmap or create new one and put in hashmap
//      [X] - Render strings to screen using Text object
//      [X] - Spaces
// [X] - Better font
// [X] - Fix bottom alignment of text
