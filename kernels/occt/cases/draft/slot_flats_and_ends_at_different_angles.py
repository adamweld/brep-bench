"""A slot-shaped block, its flats and round ends drafted at different angles. A slot-shaped block:
the outline in the plane z = 0 made of the segments from (−10, −5) to (10, −5) and from (10, 5) to
(−10, 5) and the half circles of radius 5 about (10, 0) and (−10, 0) that join them, extruded from
z = 0 to z = 10. Its side is four faces: two flats, y = 5 and y = −5, and two half cylinders,
tangent to the flats along the lines x = ±10.

Draft its sides in one operation, with the pull direction +z and the neutral plane z = 0, each
leaning inward as it rises: the two flats by 6° and the two round ends by 3°. Each face turns about
the curve where it meets the base. A flat becomes the plane through its bottom edge leaning 6° from
the z direction, so at height z it is the line |y| = h = 5 − z tan 6°. A round end becomes the cone
through its bottom half circle leaning 3° from the z direction, so at height z it is the circle of
radius ρ = 5 − z tan 3° about (±10, 0). The top, z = 10, is not drafted; it shrinks to meet them.

The flats lean more than the ends, so above the base each flat cuts into the cone beside it rather
than meeting it tangentially: the flat runs on past x = ±10 until it meets the cone, along a curve
where the two cross, and the cone is trimmed there. The section at height z is the band |y| ≤ h
across the slot whose round ends have radius ρ: the rectangle |x| ≤ 10, |y| ≤ h and the parts of
the two disks of radius ρ within the band. Each flat and each cone beside it are tangent only at
the base, at the points (±10, ±5, 0), where h = ρ = 5."""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepOffsetAPI import BRepOffsetAPI_DraftAngle
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.GC import GC_MakeArcOfCircle
from OCP.gp import gp_Dir, gp_Pln, gp_Pnt, gp_Vec

from bench import Refused, face_through
from draft import status

HALF_LENGTH, RADIUS, HEIGHT = 10, 5, 10
FLATS, ENDS = 6, 3  # degrees, inward as the side rises


def build():
    corners = [gp_Pnt(-HALF_LENGTH, -RADIUS, 0), gp_Pnt(HALF_LENGTH, -RADIUS, 0),
               gp_Pnt(HALF_LENGTH, RADIUS, 0), gp_Pnt(-HALF_LENGTH, RADIUS, 0)]
    ends = [gp_Pnt(HALF_LENGTH + RADIUS, 0, 0), gp_Pnt(-HALF_LENGTH - RADIUS, 0, 0)]
    outline = BRepBuilderAPI_MakeWire()
    outline.Add(BRepBuilderAPI_MakeEdge(corners[0], corners[1]).Edge())
    outline.Add(BRepBuilderAPI_MakeEdge(GC_MakeArcOfCircle(corners[1], ends[0], corners[2])
                                        .Value()).Edge())
    outline.Add(BRepBuilderAPI_MakeEdge(corners[2], corners[3]).Edge())
    outline.Add(BRepBuilderAPI_MakeEdge(GC_MakeArcOfCircle(corners[3], ends[1], corners[0])
                                        .Value()).Edge())
    base = BRepBuilderAPI_MakeFace(outline.Wire()).Face()
    slot = BRepPrimAPI_MakePrism(base, gp_Vec(0, 0, HEIGHT)).Shape()

    pull, neutral = gp_Dir(0, 0, 1), gp_Pln(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1))
    draft = BRepOffsetAPI_DraftAngle(slot)
    middle = HEIGHT / 2
    for point, degrees in [((0, RADIUS, middle), FLATS), ((0, -RADIUS, middle), FLATS),
                           ((HALF_LENGTH + RADIUS, 0, middle), ENDS),
                           ((-HALF_LENGTH - RADIUS, 0, middle), ENDS)]:
        draft.Add(face_through(slot, point), pull, math.radians(degrees), neutral)
        if not draft.AddDone():
            raise Refused(f"BRepOffsetAPI_DraftAngle declined the face through {point} "
                          f"({status(draft)})")
    draft.Build()
    if not draft.IsDone():
        raise Refused(f"BRepOffsetAPI_DraftAngle could not draft the slot ({status(draft)})")
    return draft.Shape()
