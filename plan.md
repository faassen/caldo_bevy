# World Plan

The world uses an entity component system.

- It uses rapier2d physics.

- Need a way to turn off gravity

- Need a way to render sprites for entities in world; either use
  bevy_protype_lyon or custom sprites that are generated somehow (from code?).

- Need to figure out whether I can speed up the physics simulation

# Custom Sprites Differences

- Size

- Shape (different polygons?)

  regular polygons from 2 upwards

  or irregular context polygons?

- Color

- Pattern

- Animation

- contained polygons

# Convex polygons

- Should we restrict ourselves to convex polygons? A trimesh can
  represent any kind of polygon, right?

- all interior angles are less than 180 degrees

- (n - 2) \* 180

- can they be represented in rapier2d? As a trimesh?

https://docs.rs/crate/bevy_lyon versus bevy_prototype_lyon

https://docs.rs/lyon/0.4.1/lyon/tessellation/basic_shapes/fn.fill_convex_polyline.html

Generate a random convex polygon:

http://cglab.ca/~sander/misc/ConvexGeneration/convex.html

Valtr algorithm

But how do we create a genetics for this? You'd like related polygons
to look alike.

Perhaps we are better off with a morphogenesis approach driven by
instructions. But how can be make shape relevant? Shape is relevant
in the 2d physics simulation. Parts of the shape could be:

- sensors

- sticky & interactive

- growth points

These could be particular points on the polygon.

https://stackoverflow.com/questions/8997099/algorithm-to-generate-random-2d-polygon

https://en.wikipedia.org/wiki/Ruppert%27s_algorithm

## Random stuff

https://dev.to/georgedoescode/tutorial-generative-blob-characters-using-svg-1igg

https://journals.plos.org/plosone/article?id=10.1371/journal.pone.0230342

https://niko.roorda.nu/computer-programs/genimal-artificial-life/

Lenia
