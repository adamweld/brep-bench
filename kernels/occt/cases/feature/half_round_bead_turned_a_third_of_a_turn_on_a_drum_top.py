"""A half-round bead turned a third of a turn on a drum's top. A drum, the cylinder of radius 20 about the z axis from z = 0 to 10.

A bead stands on its top: the half disc of radius 6 about the point (x, z) = (12, 10) in the plane
y = 0, the part above z = 10 (its diameter lies on the drum's top, from x = 6 to 18), turned about
the z axis from +x towards +y through 120°. So the bead is a third of a half torus: its round
surface is the torus about the circle ρ = 12, z = 10, and its two ends are half discs in the
half-planes at 0° and 120°. It is joined to the drum along the ring sector of the drum's top from
ρ = 6 to 18, and the drum and the bead are one solid."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder

from ribs import fuse, revolve, segment_xz


def build():
    drum = BRepPrimAPI_MakeCylinder(20, 10).Shape()
    # The half disc about (x, z) = (12, 10) above z = 10, turned from +x towards +y through 120 degrees.
    bead = revolve(segment_xz(12, 10, 6, 0, 180), 120)
    return fuse(drum, bead, "the bead and the drum")
