# Nanolabo + 🦀

### Organization

- cdt (constrained delaunay triangulation) - could be in triangulation
- mesh
- io - shall it be splitted?
  - obj
  - stl
  - step (+express)
- scene (ecs and stuff)
- nurbs (surface definitions)
- triangulation
- macros
- render
- wasm (could be external)

### Roadmap

- [x] Generalize nalgebra as defacto solution for linear algebra
- [x] Make use of SharedMesh for triangulation process
- [ ] **Reorganize project**
- [ ] Make triangulation not reference STEP (it should only rely on NURBS)
- [ ] Integrate scene into OBJ read/write
- [ ] **Integrate scene into STEP read**
- [ ] Implement STL binary read
- [ ] Implement STL ascii read / write
- [ ] Redo website wireframe
- [ ] Create first sharable POC, host it somewhere and test it
- [ ] Add some unit tests regarding triangulation / NURBS / STEP
- [ ] Expose control over triangulation quality
- [ ] **Integrate decimation in wasm**
- [ ] Add remove hidden function with wgpu
- [ ] **Implement GLTF write**
- [ ] **Implement FBX write**
