"""A disc with a round recess, cut by a cylinder that overlaps the recess and pokes out of the disc
wall. A disc: the cylinder of radius 10 about the z axis from z = 0 to z = 4. Cut from it, one
after the other, two flat-bottomed cylinders that both begin at the plane z = 3 and run up past
the disc's top face: first the recess, of radius 4 about the z axis, then the tool, of radius 5
about the axis parallel to z through (8, 0). The tool's axis is 8 from the disc's, less than the 9
that the tool's and the recess's radii add, so it overlaps the recess part way; and the tool's far
side reaches 13 from the disc's axis, so it pokes out of the disc's wall, taking a notch there.
The recess is 1 deep, the tool takes 1 of the disc's thickness as well, and the two floors are one
plane, z = 3."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    shape = BRepPrimAPI_MakeCylinder(10, 4).Shape()
    for x, radius in ((0, 4), (8, 5)):
        tool = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(x, 0, 3), gp_Dir(0, 0, 1)), radius, 3).Shape()
        cut = BRepAlgoAPI_Cut(shape, tool)
        if not cut.IsDone():
            raise Refused("BRepAlgoAPI_Cut declined a cylinder")
        shape = cut.Shape()
    return shape
