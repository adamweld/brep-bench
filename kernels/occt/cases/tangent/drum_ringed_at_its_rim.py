"""A drum united with a ring lying on its rim. The cylinder of radius 10 on the z axis from z = 0
to z = 10, united with the torus about the z axis whose middle circle, of radius 7, lies in the
plane z = 10, its tube of radius 3: the torus's outer equator is the drum's top rim."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeTorus
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    drum = BRepPrimAPI_MakeCylinder(10, 10).Shape()
    ring = BRepPrimAPI_MakeTorus(gp_Ax2(gp_Pnt(0, 0, 10), gp.DZ_s()), 7, 3).Shape()
    fuse = BRepAlgoAPI_Fuse(drum, ring)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the drum and the ring")
    return fuse.Shape()
