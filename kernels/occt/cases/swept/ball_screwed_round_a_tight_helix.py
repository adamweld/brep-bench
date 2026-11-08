"""A ball screwed round a helix tighter than itself. The solid a ball fills as its center runs
along a helix whose curvature is tighter than the ball, so that the tube about the helix folds
through itself.

The ball has radius 2. Its center starts at (1.5, 0, 0) and runs along the helix of radius 1.5
about the z axis that turns right-handed (from +x toward +y) and rises 3 every turn, for two whole
turns, ending at (1.5, 0, 6). Make the solid of everything the ball passes through.

The helix's radius of curvature, (1.5² + (3/2π)²)/1.5 ≈ 1.65, is less than the ball's radius, so
the envelope of the balls folds: its inner side is cut away by the balls themselves. And the turns,
3 apart, overlap. The solid is everything within 2 of the helix's arc."""
from bench import Refused


def build():
    # The ball's sweep is the tube about the helix with a ball at each end, and OCCT makes the
    # tube as the pipe of a circle along the helix (BRepOffsetAPI_MakePipeShell). But the
    # helix turns on a radius of about 1.65, less than the ball's 2: the pipe's sheet folds
    # through itself on the inner side of every turn, and its turns, 3 apart, cross one
    # another. OCCT's Booleans take no solid whose boundary passes through itself, and it has no
    # operation that trims a sweep where it folds, nor one that gives everything within a
    # distance of a curve.
    raise Refused("occt has no sweep of a solid, and the tube it would pipe about the helix "
                  "folds through itself where the helix turns tighter than the ball (radius "
                  "1.65 against 2); occt's Booleans take no self-intersecting solid and it has "
                  "no operation to trim a sweep where it folds")
