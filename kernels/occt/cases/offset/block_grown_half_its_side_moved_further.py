"""A block grown by 1, half of one side moved 3. The box from the origin to (20, 10, 6) whose side
y = 10 is in two coplanar faces meeting along x = 8, every face moved 1 outward but the part with
x >= 8, moved 3, with sharp edges."""
from OCP.BRepOffset import BRepOffset_MakeOffset, BRepOffset_Skin
from OCP.GeomAbs import GeomAbs_Intersection

from bench import Refused, face_through
from edit import extruded

OUTLINE = [(0, 0), (20, 0), (20, 10), (8, 10), (0, 10)]


def build():
    # the vertex at (8, 10) splits the side y = 10 into two coplanar faces
    block = extruded([(x, y, 0) for x, y in OUTLINE], (0, 0, 6))
    part = face_through(block, (14, 10, 3))
    maker = BRepOffset_MakeOffset()
    maker.Initialize(block, 1, 1e-7, BRepOffset_Skin, True, False, GeomAbs_Intersection,
                     False, False)
    maker.SetOffsetOnFace(part, 3)
    maker.MakeOffsetShape()
    if not maker.IsDone():
        raise Refused(f"BRepOffset_MakeOffset could not grow the block ({maker.Error().name})")
    return maker.Shape()
