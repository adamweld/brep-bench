"""A block grooved by its top face turned a full turn about a bottom edge. A block from the origin
to (12, 30, 20). Its top face, the rectangle 0 ≤ x ≤ 12, 0 ≤ y ≤ 30 at z = 20, is turned a whole
turn about the line of the block's bottom edge at x = 12, z = 0 (the line parallel to the y axis
through (12, 0, 0)), and everything it passes through is removed from the block: a groove.

The top face's points lie between 20 from that line (its edge x = 12, straight above it) and √(12² +
20²) = √544 (its edge x = 0). So the groove is every point between those two distances from the
line, and it takes away all of the block that is more than 20 from the line.

What is left is the part of the block within 20 of the line, a prism 30 long. Its section, in the
plane of x and z, is the rectangle 12 wide and 20 tall less its corner beyond the arc of radius 20
about (x, z) = (12, 0). The arc meets the block's top at the edge x = 12, tangent to it there, and
the side x = 0 at z = 16. So the solid's faces are its two ends, the bottom, the whole side x = 12,
the side x = 0 up to z = 16, and the arc's cylinder."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeRevol
from OCP.gp import gp_Ax1, gp_Dir, gp_Pnt

from bench import Refused, face_through

SIZE = (12, 30, 20)


def build():
    block = BRepPrimAPI_MakeBox(*SIZE).Shape()
    top = face_through(block, (6, 15, 20))
    # A groove: the top face turned a whole turn about the bottom edge at x = 12, z = 0.
    edge = gp_Ax1(gp_Pnt(12, 0, 0), gp_Dir(0, 1, 0))
    groove = BRepPrimAPI_MakeRevol(top, edge).Shape()
    cut = BRepAlgoAPI_Cut(block, groove)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not take the groove out of the block")
    return cut.Shape()
