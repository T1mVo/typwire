#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.yaml, plugin.roundtrip-bytes-yaml, bytes(()))
#roundtrip-format(typwire.yaml, plugin.roundtrip-bytes-yaml, bytes((1, 2, 3)))
