"""A chamfered block grown by 0.5, its chamfer moved 2.5. The polygon through (0, 0), (12, 0),
(12, 3), (9, 7) and (0, 7), extruded from z = 0 to z = 6, every face moved 0.5 outward but the
chamfer, moved 2.5, with sharp edges."""
from OCP.BRepOffset import BRepOffset_MakeOffset, BRepOffset_Skin
from OCP.GeomAbs import GeomAbs_Intersection

from bench import Refused, face_through
from edit import extruded

OUTLINE = [(0, 0), (12, 0), (12, 3), (9, 7), (0, 7)]


def build():
    block = extruded([(x, y, 0) for x, y in OUTLINE], (0, 0, 6))
    chamfer = face_through(block, (10.5, 5, 3))
    maker = BRepOffset_MakeOffset()
    maker.Initialize(block, 0.5, 1e-7, BRepOffset_Skin, True, False, GeomAbs_Intersection,
                     False, False)
    maker.SetOffsetOnFace(chamfer, 2.5)
    maker.MakeOffsetShape()
    if not maker.IsDone():
        raise Refused(f"BRepOffset_MakeOffset could not grow the block ({maker.Error().name})")
    return maker.Shape()
