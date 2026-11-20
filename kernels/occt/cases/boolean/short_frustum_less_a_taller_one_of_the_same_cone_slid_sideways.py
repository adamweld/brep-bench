"""A short frustum less a taller one of the same cone slid sideways. First frustum: axis the z axis, z = 0
to 8, radius 6 − z/2. Second: axis the vertical line through (4, 0), z = 0 to 5, radius 6 − z/2. Cut the
first away from the second."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere, BRepPrimAPI_MakeTorus
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    tall = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 6, 2, 8).Shape()
    short = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(4, 0, 0), gp.DZ_s()), 6, 3.5, 5).Shape()
    cut = BRepAlgoAPI_Cut(short, tall)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the two frustums")
    return cut.Shape()
