"""A cylindrical boss on a bulging vessel, its side drafted into a cone. The vessel of
`draft/box-boss-on-a-bulging-vessel-side-drafted` (a plane region with a Bézier wall turned half a
turn about the z axis), with the cylinder of radius 4 about the vertical line through (13, 6), from
z = 4 to z = 18, united with it.

Draft the cylinder's side by 4°, with the pull direction +z and the neutral plane z = 4, the
cylinder's bottom: the side becomes a cone through the circle of radius 4 at z = 4, leaning 4° from
the z direction, inward as it rises. The vessel and the cylinder's bottom are not drafted."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused, face_through
from draft import drafted
from vessel import bulging_vessel


def build():
    boss = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(13, 6, 4), gp_Dir(0, 0, 1)), 4, 14).Shape()
    fuse = BRepAlgoAPI_Fuse(bulging_vessel(), boss)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the boss")
    part = fuse.Shape()
    side = face_through(part, (13, 10, 12))  # the cylinder's side, outside the vessel's wall
    return drafted(part, [side], 4, neutral=(0, 0, 4))
