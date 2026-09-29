#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.toml, plugin.roundtrip-center-toml, (x: 30%, y: 70%))
#roundtrip-format(typwire.toml, plugin.roundtrip-center-toml, (x: 0%, y: 100%))
