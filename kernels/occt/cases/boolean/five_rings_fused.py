"""Five equal rings placed apart and fused one after another. Each is a torus about a vertical
axis, major radius 4 and tube radius 1, the centres 12 apart along x from the origin."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeTorus
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    solid = BRepPrimAPI_MakeTorus(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 4, 1).Shape()
    for k in range(1, 5):
        ring = BRepPrimAPI_MakeTorus(gp_Ax2(gp_Pnt(12 * k, 0, 0), gp.DZ_s()), 4, 1).Shape()
        fuse = BRepAlgoAPI_Fuse(solid, ring)
        if not fuse.IsDone():
            raise Refused("BRepAlgoAPI_Fuse declined a ring")
        solid = fuse.Shape()
    return solid
