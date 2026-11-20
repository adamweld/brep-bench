"""A cylinder less a cube that shares its edge, flush at top and bottom. A cube of side 10 between the origin and (10, 10, 10), and a cylinder of radius 4 whose
axis is the cube's vertical edge along the z axis, from z = 0 to z = 10, flush with the cube's bottom
and top. Cut the cube from the cylinder."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(10, 10, 10).Shape()
    cylinder = BRepPrimAPI_MakeCylinder(4, 10).Shape()
    op = BRepAlgoAPI_Cut(cylinder, cube)
    if not op.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined")
    return op.Shape()
