# Features

## Renderer
* [ ] Implement proper [.obj parsing](#obj-parsing-details) `WIP`
* [ ] Add basic primitive geometry:
    * [ ] Cubes
    * [ ] Spheres
    * [ ] Cylinders
    * [ ] Capsules
* [ ] Implement [lighting system](#lighting-details)

---

# Notes & Resources

## Renderer Deep Dives

### OBJ Parsing Details
* **Date:** 2026-08-11
* **Current Limitation:** The system only supports pre-triangulated models.
* **Inefficiency:** It creates an individual index per vertex rather than handling duplicate vertices optimally.
* **Missing Features:** It does not parse extra properties stored in `.obj` files (e.g., bump maps).

### Lighting Details
* **Status:** Specifications and requirements are currently a work in progress.
