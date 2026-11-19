"""A box joined to its mirror image across a slanted plane through it. A box from the origin to (20,
10, 10), joined to its mirror image in the upright plane x + 2y = 25: the plane through (5, 10, 0)
with normal (1, 2, 0), which crosses the box near its corner (20, 10).

The mirror takes p = (x, y, z) to p − (2(x + 2y − 25)/5)·(1, 2, 0). So the image is a box of the
same size, turned in the plane of x and y, from z = 0 to z = 10, its corners at (x, y) = (10, 20),
(22, 4), (14, −2) and (2, 14). It pokes out through the box's bottom side y = 0 and its side x = 20.

The box and its image overlap in a prism about the plane whose section is the hexagon (25/2, 0),
(50/3, 0), (20, 5/2), (20, 20/3), (35/2, 10), (5, 10). The solid is their union, one solid: a prism
10 tall whose section is the rectangle and the turned rectangle together, with fourteen sides."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt, gp_Trsf

from bench import Refused

SIZE = (20, 10, 10)


def build():
    box = BRepPrimAPI_MakeBox(*SIZE).Shape()
    mirror = gp_Trsf()
    mirror.SetMirror(gp_Ax2(gp_Pnt(5, 10, 0), gp_Dir(1, 2, 0)))  # the plane x + 2y = 25
    image = BRepBuilderAPI_Transform(box, mirror, True).Shape()
    joined = BRepAlgoAPI_Fuse(box, image)
    if not joined.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the box to its mirror image")
    joined.SimplifyResult()
    return joined.Shape()
