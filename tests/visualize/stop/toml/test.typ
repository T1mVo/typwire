#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.toml, plugin.roundtrip-stop-toml, (color: rgb("#1b1f8a"), offset: 25%))
#roundtrip-format(typwire.toml, plugin.roundtrip-stop-toml, (color: luma(40%), offset: 0%))
