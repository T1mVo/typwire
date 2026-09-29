#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.json, plugin.roundtrip-bytes-json, bytes(()))
#roundtrip-format(typwire.json, plugin.roundtrip-bytes-json, bytes((1, 2, 3)))
