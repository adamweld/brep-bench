"""A half-torus ring standing on a plate, its two roots rounded. A plate from (-25, -25, -5) to
(25, 25, 0), and standing on its top face the part above z = 0 of the torus about the z axis whose
tube has radius 2 and whose centre circle has radius 10, the two fused into one solid. The two
circles where the ring meets the plate, of radius 8 and 12, are rounded to radius 3."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeTorus
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused
from blend import edges_where, rounded

TOLERANCE = 1e-6


def build():
    plate = BRepPrimAPI_MakeBox(gp_Pnt(-25, -25, -5), gp_Pnt(25, 25, 0)).Shape()
    # The tube's angle from 0 to pi: the half of the torus above its equatorial plane.
    ring = BRepPrimAPI_MakeTorus(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 10, 2, 0, math.pi).Shape()
    fuse = BRepAlgoAPI_Fuse(plate, ring)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the ring to the plate")
    part = fuse.Shape()
    roots = edges_where(part, lambda p: abs(p.Z()) < TOLERANCE
                        and min(abs(math.hypot(p.X(), p.Y()) - 8),
                                abs(math.hypot(p.X(), p.Y()) - 12)) < TOLERANCE)
    if len(roots) != 2:
        raise Refused(f"{len(roots)} root edges")
    return rounded(part, 3, roots, "the ring's two roots")
