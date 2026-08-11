# Slush Engine

## Features:

### Renderer
* [ ] Implement proper [.obj parsing](#obj-parsing-details). **`WIP`**
* [ ] Add primitives.
    *  [ ] Cubes
    *  [ ] Spheres
    *  [ ] Cylinder
    *  [ ] Capsule
* [ ] [Lighting](#Lighting)


### Engine




# Notes & Resources
## Renderer
### OBJ Parsing Details
(08/11/26)
Currently the obj parsing system only supports models that are triangulated.
It also does not handle duplicate verticies and creates an index per vertex which is inefficient. 
Another issue is that it dosen't handle alot of the extra values stored in obj files such as bump maps.
### Lighting
Details WIP
