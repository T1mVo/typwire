#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.toml, plugin.roundtrip-angle-toml, 90deg)
#roundtrip-format(typwire.toml, plugin.roundtrip-angle-toml, -45deg)
#roundtrip-format(typwire.toml, plugin.roundtrip-angle-toml, 0deg)
#roundtrip-format(typwire.toml, plugin.roundtrip-angle-toml, 3.5rad)
