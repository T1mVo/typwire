#import "../../../common.typ": plugin, roundtrip-format, typwire

#roundtrip-format(typwire.json, plugin.roundtrip-bytes-json, bytes(()))
#roundtrip-format(typwire.json, plugin.roundtrip-bytes-json, bytes((1, 2, 3)))
