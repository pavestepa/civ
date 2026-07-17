# civ-render

Bevy rendering layer.

## Responsibility

- Camera3d setup
- PBR directional and ambient lighting
- Mesh attachment for terrain, units, cities
- GLTF scene loading (via Bevy assets)

## Rules

- No custom renderer
- No GPU API access
- Observes ECS only; never mutates simulation truth

## Future

- GPU instancing for large maps
- Frustum culling configuration
- LOD and impostor systems
