"""A cube and a cylinder on one of its edges, flush at top and bottom, their common part. A cube of side 10 between the origin and (10, 10, 10), and a cylinder of radius 4 whose
axis is the cube's vertical edge along the z axis, from z = 0 to z = 10, flush with the cube's bottom
and top. Make the part common to both."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(10, 10, 10).Shape()
    cylinder = BRepPrimAPI_MakeCylinder(4, 10).Shape()
    op = BRepAlgoAPI_Common(cube, cylinder)
    if not op.IsDone():
        raise Refused("BRepAlgoAPI_Common declined")
    return op.Shape()
