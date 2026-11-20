"""A cylinder in common with the two pieces a wider cylinder cuts it into. A is the cylinder of
radius 2 on the z axis, from z = 0 to 6; B the cylinder of radius 3 on the vertical line through
(1.5, 0), from z = 0 to 3. P = A ∩ B, Q = A \\ B; the common part of A with the compound of P and Q."""
from OCP.BRep import BRep_Builder
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common, BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.TopoDS import TopoDS_Compound
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def cylinder(x, radius, height):
    return BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(x, 0, 0), gp.DZ_s()), radius, height).Shape()


def build():
    a = cylinder(0, 2, 6)
    b = cylinder(1.5, 3, 3)
    p = BRepAlgoAPI_Common(a, b)
    q = BRepAlgoAPI_Cut(a, b)
    if not (p.IsDone() and q.IsDone()):
        raise Refused("BRepAlgoAPI could not cut the cylinder into its two pieces")
    tool = TopoDS_Compound()
    builder = BRep_Builder()
    builder.MakeCompound(tool)
    builder.Add(tool, p.Shape())
    builder.Add(tool, q.Shape())
    common = BRepAlgoAPI_Common(a, tool)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the cylinder and its pieces")
    return common.Shape()
