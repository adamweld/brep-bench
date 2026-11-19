"""A square frame extruded with its outside and its hole both flaring. A square frame extruded with
a taper that opens its hole as well as its outside. The frame is the square |x| ≤ 20, |y| ≤ 20 less
the square hole |x| ≤ 10, |y| ≤ 10, in the plane z = 0. It is extruded up to z = 10, and every side
face, outside and in, leans outward from the z axis by the same angle, arctan(1/5), moving 1
sideways for every 5 it rises.

So the outside grows, to the square |x|, |y| ≤ 22 at z = 10, and the hole grows with it, to the
square |x|, |y| ≤ 12. The hole's walls lean into the frame's material, the opposite of a draft,
which leans every wall away from the material and would close the hole. The frame stays 10 wide all
the way up. The solid is the frustum of a square pyramid from 40 × 40 at z = 0 to 44 × 44 at z = 10,
less the frustum from 20 × 20 to 24 × 24. Its walls are eight flat trapezoids, all with sharp
corners."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepOffsetAPI import BRepOffsetAPI_DraftAngle
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Dir, gp_Pln, gp_Pnt, gp_Vec

from bench import Refused
from draft import box_sides

OUTER, HOLE, HEIGHT = 20, 10, 10
TAPER = math.atan(1 / 5)


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
    # The taper as a draft about the sketch plane, every wall leaning out from the axis as it
    # rises: the outside away from the material (a negative angle), the hole's walls into it (a
    # positive one), so the hole flares with the outside.
    pull, plane = gp_Dir(0, 0, 1), gp_Pln(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1))
    draft = BRepOffsetAPI_DraftAngle(prism)
    for half, angle in ((OUTER, -TAPER), (HOLE, TAPER)):
        for face in box_sides(prism, (-half, -half, 0), (half, half, HEIGHT)):
            draft.Add(face, pull, angle, plane)
            if not draft.AddDone():
                raise Refused(f"BRepOffsetAPI_DraftAngle declined a wall: {draft.Status()}")
    draft.Build()
    if not draft.IsDone():
        raise Refused(f"BRepOffsetAPI_DraftAngle could not taper the frame: {draft.Status()}")
    return draft.Shape()
