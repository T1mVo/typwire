#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.toml, plugin.roundtrip-bytes-toml, bytes(()))
#roundtrip-format(typwire.toml, plugin.roundtrip-bytes-toml, bytes((1, 2, 3)))
