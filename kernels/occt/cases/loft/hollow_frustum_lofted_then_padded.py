"""A hollow frustum lofted between two rings, with a tube padded on its top. A loft between two
rings on the z axis, and then a pad on its top.

The loft: from the ring between radii 14 and 20 in the plane z = 0 to the ring between radii 8 and
12 in the plane z = 24, ruled. Each circle is joined by straight lines to the matching circle of the
other ring, outer to outer and inner to inner. So the loft is a hollow frustum: the frustum of a
cone from radius 20 at z = 0 to 12 at z = 24, less the frustum from radius 14 to 8.

The pad: the loft's flat top, the ring between radii 8 and 12 at z = 24, extruded up 10 to z = 34
and joined to it. It is a tube whose walls carry on from the frustum's rims, each turning there from
a cone's slope to upright."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepFeat import BRepFeat_MakePrism
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from bench import Refused, face_through

LOW, HIGH = 0, 24
OUTER = (20, 12)  # radii at z = 0 and z = 24
INNER = (14, 8)
PAD = 10


def circle(radius, z):
    edge = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(gp_Pnt(0, 0, z), gp_Dir(0, 0, 1)), radius))
    return BRepBuilderAPI_MakeWire(edge.Edge()).Wire()


def frustum(radii):
    """A ruled loft between the circle of radius radii[0] at z = 0 and radii[1] at z = 24."""
    loft = BRepOffsetAPI_ThruSections(True, True)
    loft.AddWire(circle(radii[0], LOW))
    loft.AddWire(circle(radii[1], HIGH))
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections could not loft the circles")
    return loft.Shape()


def build():
    # OCCT lofts wire to wire: a ring is lofted as its outer circles less its inner ones.
    hollow = BRepAlgoAPI_Cut(frustum(OUTER), frustum(INNER))
    if not hollow.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not hollow the lofted frustum")
    hollow = hollow.Shape()
    top = face_through(hollow, (10, 0, HIGH))
    pad = BRepFeat_MakePrism(hollow, top, top, gp_Dir(0, 0, 1), 1, True)
    pad.Perform(PAD)
    if not pad.IsDone():
        raise Refused(f"BRepFeat_MakePrism could not pad the top: {pad.CurrentStatusError()}")
    return pad.Shape()
