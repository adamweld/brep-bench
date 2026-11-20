"""A round bead turned over a drum's rim. A drum, the cylinder of radius 20 about the z axis from z = 0 to 10.

A bead is turned on its rim: the part of the disc of radius 6 about the point (x, z) = (18, 10) in
the plane y = 0 that lies above the line z = 8 (a little more than half the disc), turned about the
z axis from +x towards +y through 150°. The disc reaches out to ρ = 24, so the bead overhangs the
drum's wall: it lies on the drum's top from ρ = 12 to 20, and against its wall from z = 8 to 10,
and outside the wall it has a flat underside at z = 8 from ρ = 20 to 18 + √32 ≈ 23.66. Its round
surface is the torus about the circle ρ = 18, z = 10, and its two ends are flat, in the half-planes
at 0° and 150°. The drum and the bead are one solid."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder

from ribs import fuse, revolve, segment_xz


def build():
    drum = BRepPrimAPI_MakeCylinder(20, 10).Shape()
    # The part of the disc about (x, z) = (18, 10) above the line z = 8, turned through 150 degrees.
    tilt = math.degrees(math.asin(1 / 3))
    bead = revolve(segment_xz(18, 10, 6, -tilt, 180 + tilt), 150)
    return fuse(drum, bead, "the bead and the drum")
