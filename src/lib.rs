pub mod input;
pub mod objects;
pub mod rendering;

// TODO IN PROGRESS:
// [ ] - 2D camera
// [ ] - Separation 2D between UI without camera and non UI with camera
//          [ ] - Special UI queue for DrawObject2D
// [X] - Builder pattern for all objects
// [ ] - Add text components
// [ ] - Wrap draw objects in other object with buffer info and draw implementation when in context (look at polygon buffers)
// [X] - Queues for draw objects in stead of Vecs
//
// TODO PRE PHYSICS:
// [X] - Depth testing
// [X] - Normals
// [X] - Phong shading
// [X] - Backface culling
// [X] - Split objects into 2D and 3D and draw 2D always on top
// [X] - Builder pattern for block
// [X] - Update to 0.32.0
//
// LATER TODO:
// [ ] - Configurable lights in context
// [ ] - Figure out rotation
// [ ] - Resizing of window
// [ ] - Delta time
// [ ] - Render/update closures
// [ ] - Working borders
// [ ] - Line renderer
//
// OPTIONAL TODO:
// [ ] - Shadows
// [ ] - Figure out a better drawing strategy