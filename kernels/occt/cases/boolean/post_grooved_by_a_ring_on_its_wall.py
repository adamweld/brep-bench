"""A post grooved by a ring whose middle circle lies on its wall. The cylinder of radius 6 on the z
axis from z = 0 to z = 12, less the torus about the z axis whose middle circle, of radius 6, lies
in the plane z = 6, its tube of radius 2."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeTorus
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    post = BRepPrimAPI_MakeCylinder(6, 12).Shape()
    ring = BRepPrimAPI_MakeTorus(gp_Ax2(gp_Pnt(0, 0, 6), gp.DZ_s()), 6, 2).Shape()
    cut = BRepAlgoAPI_Cut(post, ring)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the ring")
    return cut.Shape()
