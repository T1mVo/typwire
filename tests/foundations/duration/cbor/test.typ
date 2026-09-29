#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-duration-cbor, duration(hours: 2))
#roundtrip-format(typwire.cbor, plugin.roundtrip-duration-cbor, duration(days: 1, minutes: 30))
#roundtrip-format(typwire.cbor, plugin.roundtrip-duration-cbor, duration(seconds: -90))
