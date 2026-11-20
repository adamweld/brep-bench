"""A half-elliptic prism with its two upright corners rounded, then its whole top outline. The
half-ellipse x²/64 + y²/16 ≤ 1, y ≥ 0, from z = 0 to 4; its upright edges at (±8, 0) rounded to
3/2, then the whole top outline (one tangent chain) to 1/2."""
from bench import other_case
from blend import edges_where, rounded


def build():
    prism = other_case("fillet/half-elliptic-prism-arc-edge").half_ellipse_prism(8, 4, 4)
    uprights = edges_where(prism, lambda p: abs(abs(p.X()) - 8) < 1e-7 and abs(p.Y()) < 1e-7
                           and abs(p.Z() - 2) < 1e-7)
    assert len(uprights) == 2, len(uprights)
    block = rounded(prism, 1.5, uprights, "the two upright edges")
    top = edges_where(block, lambda p: abs(p.Z() - 4) < 1e-7)
    return rounded(block, 0.5, top, "the top outline")
