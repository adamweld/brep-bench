"""A web rib bridging the waist between two cones. Two cone frusta stacked on the z axis, with a waist between them. The lower one narrows from
radius 24 at z = 0 to radius 14 at z = 30, and the upper one widens from radius 14 at z = 30 to
radius 22 at z = 70. So the body's radius is ρ_b(z) = 24 − z/3 for 0 ≤ z ≤ 30 and
ρ_b(z) = 14 + (z − 30)/5 for 30 ≤ z ≤ 70, and its ends are the discs of radius 24 at z = 0 and of
radius 22 at z = 70. The waist is the circle ρ = 14, z = 30, a concave crease.

A web rib bridges the waist. Its profile, in the half-plane y = 0, x > 0, is the straight line from
(ρ, z) = (19, 15) on the lower cone to (17, 45) on the upper one; turned about the z axis, the line
sweeps a third cone, ρ = 19 − (z − 15)/15. The rib is the plate 4 thick, 2 to each side of the plane
y = 0 on the +x side, between the body and that cone, from z = 15 to z = 45: where it leaves the
body at z = 15 and returns to it at z = 45 it has no thickness. Its two flat sides are the planes
y = ±2. The body and the rib are one solid."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from ribs import common, fuse, polygon, revolve


def cone(r0, r1, z0, z1):
    return BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, z0), gp_Dir(0, 0, 1)), r0, r1, z1 - z0).Shape()


def build():
    body = fuse(cone(24, 14, 0, 30), cone(14, 22, 30, 70), "the two cones")
    # The rib's profile in y = 0: the line from (19, 15) to (17, 45), and the body's side under it.
    profile = polygon([(19, 0, 15), (14, 0, 30), (17, 0, 45)])
    turned = revolve(profile, 360)
    slab = BRepPrimAPI_MakeBox(gp_Pnt(0, -2, 15), gp_Pnt(25, 2, 45)).Shape()
    rib = common(slab, turned, "the slab and the turned profile")
    return fuse(body, rib, "the rib and the body")
