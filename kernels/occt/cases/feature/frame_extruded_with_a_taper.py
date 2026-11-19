"""A square frame extruded with a taper, its outside flaring and its hole closing. A square frame
extruded with a taper. The frame is the square |x| ≤ 20, |y| ≤ 20 less the square hole |x| ≤ 10, |y|
≤ 10, in the plane z = 0. It is extruded up to z = 10, and every side face leans away from the
frame's material by the same angle, arctan(1/5), moving 1 sideways for every 5 it rises.

So the outside grows, to the square |x|, |y| ≤ 22 at z = 10. The hole's walls also lean away from
the material, toward the z axis, so the hole closes, to the square |x|, |y| ≤ 8 at z = 10. The solid
is the frustum of a square pyramid from 40 × 40 at z = 0 to 44 × 44 at z = 10, less the frustum from
20 × 20 to 16 × 16. Its walls are eight flat trapezoids, all with sharp corners: four outside, from
40 wide to 44, and four inside, from 20 to 16."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from draft import box_sides, drafted

OUTER, HOLE, HEIGHT = 20, 10, 10
TAPER = math.degrees(math.atan(1 / 5))


def square(half, clockwise=False):
    outline = BRepBuilderAPI_MakePolygon()
    corners = [(-1, -1), (1, -1), (1, 1), (-1, 1)]
    for x, y in (reversed(corners) if clockwise else corners):
        outline.Add(gp_Pnt(x * half, y * half, 0))
    outline.Close()
    return outline.Wire()


def build():
    frame = BRepBuilderAPI_MakeFace(square(OUTER))
    frame.Add(square(HOLE, clockwise=True))
    prism = BRepPrimAPI_MakePrism(frame.Face(), gp_Vec(0, 0, HEIGHT)).Shape()
    # The taper as a draft of every side face about the sketch plane, each leaning away from the
    # material as it rises (a negative angle): the outside flares and the hole closes.
    sides = (box_sides(prism, (-OUTER, -OUTER, 0), (OUTER, OUTER, HEIGHT))
             + box_sides(prism, (-HOLE, -HOLE, 0), (HOLE, HOLE, HEIGHT)))
    return drafted(prism, sides, -TAPER)
