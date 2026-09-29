#import "../../../common.typ": roundtrip-format, plugin, typwire

// Every horizontal alignment, standalone.
#roundtrip-format(typwire.toml, plugin.roundtrip-alignment-toml, start)
#roundtrip-format(typwire.toml, plugin.roundtrip-alignment-toml, left)
#roundtrip-format(typwire.toml, plugin.roundtrip-alignment-toml, center)
#roundtrip-format(typwire.toml, plugin.roundtrip-alignment-toml, right)
#roundtrip-format(typwire.toml, plugin.roundtrip-alignment-toml, end)

// Every vertical alignment, standalone.
#roundtrip-format(typwire.toml, plugin.roundtrip-alignment-toml, top)
#roundtrip-format(typwire.toml, plugin.roundtrip-alignment-toml, horizon)
#roundtrip-format(typwire.toml, plugin.roundtrip-alignment-toml, bottom)

// 2D combinations.
#roundtrip-format(typwire.toml, plugin.roundtrip-alignment-toml, left + top)
#roundtrip-format(typwire.toml, plugin.roundtrip-alignment-toml, right + bottom)
#roundtrip-format(typwire.toml, plugin.roundtrip-alignment-toml, center + horizon)
