"""Five counterbored holes on a bolt circle round a bored disk. A round disk of radius 40 on the z
axis, from z = 0 to z = 12, with a bore of radius 12 on the z axis right through it, and five
counterbored holes on a bolt circle of radius 28 about the z axis.

Each counterbored hole is drilled down from the top face (z = 12), along a vertical axis:

- the hole, radius 3.3, right through the disk to the bottom face (z = 0);
- the counterbore, radius 5.5 and 6.5 deep, from the top face down to a flat floor at z = 5.5.

The first hole's axis is the vertical line through (28, 0); each of the others is the first turned
about the z axis by 72°, 144°, 216° and 288°. The holes stay clear of the bore (the counterbores'
nearest point is 22.5 from the z axis, the bore 12), of the rim (furthest point 33.5, the rim 40)
and of each other. If you cut the holes with cylinders, make them run past the bottom face."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt, gp_Trsf

from bench import Refused
from hole import cutter

# One counterbored hole's half section from the top face down: the counterbore, radius 5.5 to
# depth 6.5, then the hole, radius 3.3, 1 past the bottom face.
SECTION = [(0, -1), (5.5, -1), (5.5, 6.5), (3.3, 6.5), (3.3, 13), (0, 13)]
HOLES = 5


def build():
    disk = BRepPrimAPI_MakeCylinder(40, 12).Shape()
    bore = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -1), gp.DZ_s()), 12, 14).Shape()
    cut = BRepAlgoAPI_Cut(disk, bore)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the bore")
    part = cut.Shape()
    first = cutter((28, 0, 12), (0, 0, -1), SECTION)
    for k in range(HOLES):
        turn = gp_Trsf()
        turn.SetRotation(gp.OZ_s(), math.radians(360 / HOLES * k))
        cut = BRepAlgoAPI_Cut(part, BRepBuilderAPI_Transform(first, turn, True).Shape())
        if not cut.IsDone():
            raise Refused(f"BRepAlgoAPI_Cut declined counterbored hole {k + 1}")
        part = cut.Shape()
    return part
