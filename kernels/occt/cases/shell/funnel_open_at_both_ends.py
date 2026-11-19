"""A funnel: a lofted cone frustum shelled inward, open at both ends. A cone frustum about the z
axis, lofted through two circles about the axis joined by straight lines: the circle of radius 40
in the plane z = 0 and the circle of radius 4 in the plane z = 30. Its side is the cone ρ = 40 −
1.2z, and its ends are flat disks.

Shell it inward to a wall 1 thick, open at both ends, a funnel. The outside of the wall is the
frustum's side; the inside is the cone moved 1 inward along its normal, the parallel cone ρ = 40
− 1.2z − √2.44 (√(1 + 1.2²) ≈ 1.562), which runs on from z = 0 to z = 30 to meet the planes of
the openings. The rims are flat annuli, in z = 0 between radii 40 − √2.44 ≈ 38.438 and 40, and in
z = 30 between radii 4 − √2.44 ≈ 2.438 and 4."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from bench import Refused, face_through
from shell import shell


def circle(radius, z):
    edge = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(gp_Pnt(0, 0, z), gp_Dir(0, 0, 1)), radius))
    return BRepBuilderAPI_MakeWire(edge.Edge()).Wire()


def build():
    loft = BRepOffsetAPI_ThruSections(True, True)  # a solid, ruled
    loft.AddWire(circle(40, 0))
    loft.AddWire(circle(4, 30))
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections could not loft the two circles")
    frustum = loft.Shape()
    ends = [face_through(frustum, (1, 1, 0)), face_through(frustum, (1, 1, 30))]
    return shell(frustum, ends, 1)
