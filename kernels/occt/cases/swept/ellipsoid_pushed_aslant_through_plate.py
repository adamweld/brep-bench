"""An ellipsoid pushed aslant wholly through a plate. A plate, the box from (−15, −15, 0) to (15,
15, 4), with a hole punched through it by an ellipsoid pushed aslant wholly through it.

The ellipsoid has semi-axes 3, 2 and 1.5 along x, y and z: everything with ((x + 2)/3)² + ((y +
1.5)/2)² + ((z + 4)/1.5)² ≤ 1, centered at (−2, −1.5, −4), wholly below the plate.

It slides without turning along the vector (4, 3, 12), ending centered at (2, 1.5, 8), wholly above
the plate. Cut from the plate everything the ellipsoid passes through.

The ellipsoid touches what it sweeps along one smooth closed curve, where its normal is square to
the push. That curve's path is the hole's wall, an oblique elliptic cylinder: every level of the
plate loses the same ellipse."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_GTransform, BRepBuilderAPI_NurbsConvert
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeSphere
from OCP.gp import gp_GTrsf, gp_Mat, gp_XYZ, gp_Pnt

from sweep import cut, plane, slid, split_solid

PLATE = ((-15, -15, 0), (15, 15, 4))
CENTRE = (-2, -1.5, -4)
SEMI = (3, 2, 1.5)  # the semi-axes along x, y and z
PUSH = (4, 3, 12)


def build():
    # At a point p of the ellipsoid, the image of the point q of the unit ball, p = c + Aq with
    # A = diag(3, 2, 1.5), the normal is along A⁻¹q, square to the push v where q·A⁻¹v = 0: the
    # image of the unit ball's great circle square to A⁻¹v. Split the ball along that circle,
    # where the plane meets it exactly, before stretching it.
    ball = split_solid(BRepPrimAPI_MakeSphere(1).Shape(),
                       [plane((0, 0, 0), tuple(v / a for v, a in zip(PUSH, SEMI)))])
    # OCCT has no ellipsoid: the ball stretched by a non-uniform scale and moved to the centre,
    # which GTransform carries out exactly, as a rational B-spline
    stretch = gp_GTrsf(gp_Mat(SEMI[0], 0, 0, 0, SEMI[1], 0, 0, 0, SEMI[2]), gp_XYZ(*CENTRE))
    ellipsoid = BRepBuilderAPI_GTransform(ball, stretch, True).Shape()
    # The hole's wall is the slide of the ellipse, a surface of linear extrusion of a rational
    # B-spline curve, which OCCT's integration of area and volume (BRepGProp) gets wrong by 4e-4
    # once the plate trims it; the same surface written as a NURBS (an exact conversion) it
    # integrates right.
    swept = [BRepBuilderAPI_NurbsConvert(piece, True).Shape() for piece in slid(ellipsoid, PUSH)]
    return cut(BRepPrimAPI_MakeBox(gp_Pnt(*PLATE[0]), gp_Pnt(*PLATE[1])).Shape(), swept)
