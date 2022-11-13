pub mod input;
pub mod rendering;
pub mod public;

// TODO IN PROGRESS:
// [ ] - Rename render pass structs/file names
// [ ] - Add text components
//      [X] - Figure out how to create R8_UINT image and sent it to the gpu
//      [ ] - Create hashmap for character textures (in render pass)
//      [ ] - Either get character from hashmap or create new one and put in hashmap
//      [ ] - Render strings to screen using Text object  

// TODO PRE PHYSICS:
//
// LATER TODO:
// [ ] - anti aliasing
//      [ ] - aa 2D
//      [ ] - aa 3D
// [ ] - Better/more camera options
// [ ] - Configurable lights in context
// [ ] - Figure out rotation
// [ ] - Resizing of window
// [ ] - Delta time
// [ ] - Render and update closures
// [ ] - Working borders
// [ ] - Line renderer
// [ ] - Borders around 2D objects
// [ ] - Shadows
// [ ] - Figure out a better drawing strategy
//
// DONE:
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