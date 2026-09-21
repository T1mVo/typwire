#import "../../common.typ": roundtrip, plugin

// Every horizontal alignment, standalone.
#roundtrip(plugin.roundtrip_alignment, start)
#roundtrip(plugin.roundtrip_alignment, left)
#roundtrip(plugin.roundtrip_alignment, center)
#roundtrip(plugin.roundtrip_alignment, right)
#roundtrip(plugin.roundtrip_alignment, end)

// Every vertical alignment, standalone.
#roundtrip(plugin.roundtrip_alignment, top)
#roundtrip(plugin.roundtrip_alignment, horizon)
#roundtrip(plugin.roundtrip_alignment, bottom)

// 2D combinations.
#roundtrip(plugin.roundtrip_alignment, left + top)
#roundtrip(plugin.roundtrip_alignment, right + bottom)
#roundtrip(plugin.roundtrip_alignment, center + horizon)
