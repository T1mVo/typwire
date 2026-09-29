#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.yaml, plugin.roundtrip-stop-yaml, (color: rgb("#1b1f8a"), offset: 25%))
#roundtrip-format(typwire.yaml, plugin.roundtrip-stop-yaml, (color: luma(40%), offset: 0%))
