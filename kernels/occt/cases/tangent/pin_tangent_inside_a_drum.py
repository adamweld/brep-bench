"""A pin touching a drum's wall from inside, united. The cylinder of radius 10 on the z axis from
z = 0 to z = 10, and the cylinder of radius 3 on the line x = 7, y = 0 from z = 4 to z = 16, which
touches the drum's wall from inside along the line x = 10, y = 0 and comes out through its top.
Both seams lie on that line."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    # OCCT's cylinder has its seam toward its placement's x direction: here +x, for both
    drum = BRepPrimAPI_MakeCylinder(10, 10).Shape()
    pin = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(7, 0, 4), gp.DZ_s(), gp.DX_s()), 3, 12).Shape()
    fuse = BRepAlgoAPI_Fuse(drum, pin)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the drum and the pin")
    return fuse.Shape()
