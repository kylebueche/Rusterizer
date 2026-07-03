# Rusterizer

<hr>

Rusterizer is a software Rasterizer and Path Tracer written in Rust.

The rasterization algorithms provided in this codebase are my own implementations of classic research papers.
The path tracing algorithms are a Rust follow-along of Ray Tracing in One Weekend, and Ray Tracing: The Next Week.

## Rasterizer

Supported Features:
- Line drawing via the Bresenham & Xiaolin Wu algorithms
- Triangle drawing via the Scanline & Cross-Product Anti-Aliased algorithms
- Point drawing with specifiable radius and a circular or square shape

## Path Tracing

Supported Features:
- Spheres, Quads, Planes
- Diffuse, Reflective, Refractive & Emissive materials
- Monte Carlo-based sampling
- Bounding Volume Hierarchies (BVH)
- Volume Rendering
- Direct Illumination
- Global Illumination
- Texture Loading
- Procedural Noise Textures