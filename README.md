# fast-raycaster
A raycaster in rust that only casts rays for each vertex instead of horizontal pixel.

## How it works
A normal raycaster casts a ray for every horizontal pixel on the screen. For a 1080p monitor, that's 1080 rays.
Since the program already knows where the walls are, this one just calculates the distance from each wall's verteces to the camera, and draws a quad on the screen using the distance and angle of the ray.

## Known Bugs:
- Camera clipping
- Walls not drawn in order
