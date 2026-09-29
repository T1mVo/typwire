#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.yaml, plugin.roundtrip-angle-yaml, 90deg)
#roundtrip-format(typwire.yaml, plugin.roundtrip-angle-yaml, -45deg)
#roundtrip-format(typwire.yaml, plugin.roundtrip-angle-yaml, 0deg)
#roundtrip-format(typwire.yaml, plugin.roundtrip-angle-yaml, 3.5rad)
