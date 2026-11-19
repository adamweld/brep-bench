"""A notched plate grown by 1 with sharp edges, its slanted wall moved 16, past a jaw. A plate whose
outline in the xy plane is the polygon through (0, 0), (40, 0), (40, 10), (25, 10), (20, 20),
(40, 20), (40, 30) and (0, 30), extruded from z = 0 to z = 10: a rectangle with a notch cut into
its side x = 40, between a lower jaw (y ≤ 10) and an upper jaw (y ≥ 20). The notch's inner wall
slants, from (25, 10) to (20, 20), in the plane 2x + y = 60.

Grow it with its edges kept sharp: move every face 1 outward along its normal, except the slanted
wall, which moves 16 outward, along (2, 1)/√5, into the notch; and extend the moved faces until
they meet, so that no round is added. The outside becomes the box from (−1, −1, −1) to (41, 31,
11), the jaws' inner faces move to y = 11 and y = 19, and the wall to the plane 2x + y = 60 +
16√5 (≈ 95.78).

The wall moves past the lower jaw: at y = 11 it would lie at x ≈ 42.39, beyond the jaws' ends at
x = 41. So the lower jaw's inner face is gone, overtaken, and the wall runs on to meet the end
face x = 41 directly, at y = 16√5 − 22 ≈ 13.78. What is left of the notch is a groove whose
section is a right triangle, between the wall, the upper jaw's face y = 19 from x = (41 + 16√5)/2
≈ 38.39 to 41, and the opening in the face x = 41 from y ≈ 13.78 to 19."""
from OCP.BRepOffset import BRepOffset_MakeOffset, BRepOffset_Skin
from OCP.GeomAbs import GeomAbs_Intersection

from bench import Refused, face_through
from edit import extruded

OUTLINE = [(0, 0), (40, 0), (40, 10), (25, 10), (20, 20), (40, 20), (40, 30), (0, 30)]
THICKNESS, GROWTH, WALL = 10, 1, 16


def build():
    plate = extruded([(x, y, 0) for x, y in OUTLINE], (0, 0, THICKNESS))
    wall = face_through(plate, (22.5, 15, 5))
    offset = BRepOffset_MakeOffset()
    offset.Initialize(plate, GROWTH, 1e-7, BRepOffset_Skin, True, False, GeomAbs_Intersection,
                      False, False)
    offset.SetOffsetOnFace(wall, WALL)
    offset.MakeOffsetShape()
    if not offset.IsDone():
        raise Refused(f"BRepOffset_MakeOffset could not grow the plate ({offset.Error().name})")
    return offset.Shape()
