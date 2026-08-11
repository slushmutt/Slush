# Features

### Renderer
* [ ] Implement proper [.obj parsing](#obj-parsing-details) `WIP`
* [ ] Add basic primitive geometry:
    * [ ] Cubes
    * [ ] Spheres
    * [ ] Cylinders
    * [ ] Capsules
* [ ] Implement [lighting system](#lighting-details)
* [ ] Trenchbroom [.map file](#trenchbroom-map-details) parsing and rendering
* [ ] UI Rendering
* [ ] [Stylized Shaders](#shader-details)
    * [ ] Vertex Warping
* [ ] Seperate my models bind groups from whatever the last loaded mesh was so that it can have its own position.

### Engine
* [ ] [Collision detection](#collision-details)
* [ ] [Physics System](#physics-details)
* [ ] Trenchbroom [.map file](#trenchbroom-map-details) Implementation
* [ ] Logging System
* [ ] UI System
* [ ] [Multiplayer](#multiplayer-details)

# Notes & Resources

## Renderer Details

### OBJ Parsing Details
* **Last Changed:** 2026-08-11
* **Current Limitation:** The system only supports pre-triangulated models.
* **Inefficiency:** It creates an individual index per vertex rather than handling duplicate vertices optimally.
* **Missing Features:** It does not parse extra properties stored in `.obj` files (e.g., bump maps).

### Lighting Details
* **Last Changed:** 2026-08-11
* **Status:** Specifications and requirements are currently a work in progress.
### Shader Details
* **Last Changed:** 2026-08-11
* **Vertex Warping Resource:** [PS1 look](https://www.reddit.com/r/opengl/comments/cp9meo/what_is_the_best_way_to_simulate_the_ps1_look/)

## Engine Details

### Collision Details
* **Last Changed:** 2026-08-11
* **Implementation:** Need to decide a method for collision. More than likely will choose something that works well with BSP Trees as that will be the way I do physics.
### Physics Details
* **Last Changed:** 2026-08-11
* **Implementation:** Going to do a system similar to Quake, more than likely going to be BSP Trees.
### Trenchbroom Map Details
* **Last Changed:** 2026-08-11
* **.map File Resource:** [.map files](https://github.com/stefanha/map-files/blob/master/MAPFiles.pdf)
* **Math behind the plane system:** [Plane related math](https://paulbourke.net/geometry/pointlineplane/)
### Multiplayer Details
* **Last Changed:** 2026-08-11
* **Status:** Specifications and requirements are currently a work in progress.
