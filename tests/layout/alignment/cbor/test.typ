#import "../../../common.typ": roundtrip-format, plugin, typwire

// Every horizontal alignment, standalone.
#roundtrip-format(typwire.cbor, plugin.roundtrip-alignment-cbor, start)
#roundtrip-format(typwire.cbor, plugin.roundtrip-alignment-cbor, left)
#roundtrip-format(typwire.cbor, plugin.roundtrip-alignment-cbor, center)
#roundtrip-format(typwire.cbor, plugin.roundtrip-alignment-cbor, right)
#roundtrip-format(typwire.cbor, plugin.roundtrip-alignment-cbor, end)

// Every vertical alignment, standalone.
#roundtrip-format(typwire.cbor, plugin.roundtrip-alignment-cbor, top)
#roundtrip-format(typwire.cbor, plugin.roundtrip-alignment-cbor, horizon)
#roundtrip-format(typwire.cbor, plugin.roundtrip-alignment-cbor, bottom)

// 2D combinations.
#roundtrip-format(typwire.cbor, plugin.roundtrip-alignment-cbor, left + top)
#roundtrip-format(typwire.cbor, plugin.roundtrip-alignment-cbor, right + bottom)
#roundtrip-format(typwire.cbor, plugin.roundtrip-alignment-cbor, center + horizon)
