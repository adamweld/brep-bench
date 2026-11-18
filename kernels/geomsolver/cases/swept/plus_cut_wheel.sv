// A wheel cut by a plus always in mesh with it, as a gear shaper cuts. The plus, four arms 30 long
// and 8 wide about the origin, is drawn in the top plane and extruded from z = -10 to 0; the wheel
// is the disc of radius 40 about the vertical through (-50, 0), z from -8 to -2. The plus turns
// about z while the wheel turns two thirds as fast the other way about its own axis; the kernel's
// sweep of the plus over 0..540 degrees of that motion, seen from the wheel, is cut from it.
unit mm
use std
use std.solids (cut)
use std.place (vertical, coords)
use std.motion (turns)

in std.front {
  construction stem := vertical line(std.origin, hint((0, 10)))
  length(10mm) stem
  wc := point hint((-50, 0))
  std.origin distance(-50mm, along: u) wc
  construction spindle := vertical line(wc, hint((-50, 10)))
  length(10mm) spindle
}

// the plus: each arm k, along the direction 90k degrees, ends in corners a and b 30 out and 4 to
// either side, and meets the next arm at the central square's corner c
in std.top {
  repeat 4 as k {
    a := point hint((30 * cos(90deg * k) - 4 * sin(90deg * k), 30 * sin(90deg * k) + 4 * cos(90deg * k)))
    b := point hint((30 * cos(90deg * k) + 4 * sin(90deg * k), 30 * sin(90deg * k) - 4 * cos(90deg * k)))
    c := point hint((4 * sqrt(2) * cos(90deg * k - 45deg), 4 * sqrt(2) * sin(90deg * k - 45deg)))
    a coords(du: 30mm * cos(90deg * k) - 4mm * sin(90deg * k), dv: 30mm * sin(90deg * k) + 4mm * cos(90deg * k)) std.top
    b coords(du: 30mm * cos(90deg * k) + 4mm * sin(90deg * k), dv: 30mm * sin(90deg * k) - 4mm * cos(90deg * k)) std.top
    c coords(du: 4mm * sqrt(2) * cos(90deg * k - 45deg), dv: 4mm * sqrt(2) * sin(90deg * k - 45deg)) std.top
  }
  plus := face(c[0], b[0], a[0], c[1], b[1], a[1], c[2], b[2], a[2], c[3], b[3], a[3], -> close)
  wo := point
  wo coincident wc
  rim := circle(center: wo) hint(r: 40)
  radius(40mm) rim
}
construction tool := solid(std.solids.Prism(plus, from: -10mm, to: 0mm))
part := solid(std.solids.Prism(face(rim), from: -8mm, to: -2mm))

// the plus turning about its stem, the wheel two thirds as fast the other way about its spindle,
// and what the plus passes through as the wheel sees it over a whole period of the pair
tool_turn := frame
tool_turn turns(1) stem
wheel_turn := frame
wheel_turn turns(-2 / 3) spindle
construction space := solid({ p | some t in (0deg, 540deg): p in tool at(under: tool_turn, seen_from: wheel_turn) t })
space cut part
