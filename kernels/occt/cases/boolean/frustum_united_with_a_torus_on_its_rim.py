"""A cone frustum on the z axis, from z = −8 to z = 0, radius 8 at z = −8 and radius 6 at z = 0, so that its radius at height z is 6 - z/4. And a torus on the same axis, centred on the origin, which is in the plane of the frustum's top face: its tube has radius 2 and its centre circle has radius 4 in the plane z = 0. The torus's outer equator, the circle of radius 6 in the plane z = 0, is exactly the frustum's top rim; its inner equator is the circle of radius 2 there.

The frustum widens downward, so the torus's lower half lies inside it, touching nothing of its wall; the torus's upper half lies above its top face, which it meets in the ring 2 ≤ ρ ≤ 6.

Make the union of the two: the frustum with the torus's upper half standing on its top face, the half's outer foot circle on the frustum's rim. Its surface is the frustum's base, its side, the disc ρ ≤ 2 left of its top, and the torus's upper half; the torus surface meets the cone at the rim in an edge.
"""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone, BRepPrimAPI_MakeTorus
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    frustum = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, -8), gp.DZ_s()), 8, 6, 8).Shape()
    torus = BRepPrimAPI_MakeTorus(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 4, 2).Shape()
    result = BRepAlgoAPI_Fuse(frustum, torus)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the frustum and the torus")
    return result.Shape()
