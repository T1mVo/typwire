#import "../../../common.typ": roundtrip-format, plugin, typwire

// Every horizontal alignment, standalone.
#roundtrip-format(typwire.yaml, plugin.roundtrip-alignment-yaml, start)
#roundtrip-format(typwire.yaml, plugin.roundtrip-alignment-yaml, left)
#roundtrip-format(typwire.yaml, plugin.roundtrip-alignment-yaml, center)
#roundtrip-format(typwire.yaml, plugin.roundtrip-alignment-yaml, right)
#roundtrip-format(typwire.yaml, plugin.roundtrip-alignment-yaml, end)

// Every vertical alignment, standalone.
#roundtrip-format(typwire.yaml, plugin.roundtrip-alignment-yaml, top)
#roundtrip-format(typwire.yaml, plugin.roundtrip-alignment-yaml, horizon)
#roundtrip-format(typwire.yaml, plugin.roundtrip-alignment-yaml, bottom)

// 2D combinations.
#roundtrip-format(typwire.yaml, plugin.roundtrip-alignment-yaml, left + top)
#roundtrip-format(typwire.yaml, plugin.roundtrip-alignment-yaml, right + bottom)
#roundtrip-format(typwire.yaml, plugin.roundtrip-alignment-yaml, center + horizon)
