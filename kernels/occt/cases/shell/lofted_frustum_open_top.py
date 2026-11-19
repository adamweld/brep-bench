"""A frustum lofted between two circles, shelled with its narrow top open. A cone frustum on the z
axis made by a loft: its side lofted, ruled, between the circle of radius 6 at z = 0 and the circle
of radius 3 at z = 8, and closed by the two disks they bound. The ruled surface between two coaxial
circles is the cone through them, so the frustum's radius is ρ = 6 − 3z/8, its side leaning
α = atan(3/8) from the z direction.

Shell it inward to a wall 1 thick, open at its narrow top face, z = 8. The outside of the wall is
its side and its bottom, unmoved, and the inside is each of them moved 1 inward along its normal,
onto the surface offset 1 from it. The moved faces are extended or trimmed where they meet one
another, and run on until they meet the surface of the opening, which is left as a rim between the
outside and the inside.

The side, moved 1 inward, is the cone of the same angle whose radius is less by
1/cos α = √73/8 ≈ 1.068: ρ = 6 − 3z/8 − √73/8. The bottom moved is the plane z = 1. So the result
is the frustum less the frustum between z = 1 and z = 8 under that cone, of radius
45/8 − √73/8 ≈ 4.557 at z = 1 and 3 − √73/8 ≈ 1.932 at z = 8: a cup whose rim is the annulus
between radii 1.932 and 3 at z = 8."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from bench import Refused
from shell import face_through, shell

BOTTOM_RADIUS, TOP_RADIUS, HEIGHT = 6, 3, 8
THICKNESS = 1


def circle(z, radius):
    edge = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(gp_Pnt(0, 0, z), gp_Dir(0, 0, 1)), radius))
    return BRepBuilderAPI_MakeWire(edge.Edge()).Wire()


def build():
    loft = BRepOffsetAPI_ThruSections(True, True)  # a solid, ruled between the sections
    loft.AddWire(circle(0, BOTTOM_RADIUS))
    loft.AddWire(circle(HEIGHT, TOP_RADIUS))
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections could not loft the frustum")
    frustum = loft.Shape()
    return shell(frustum, [face_through(frustum, (0, 0, HEIGHT))], THICKNESS)
