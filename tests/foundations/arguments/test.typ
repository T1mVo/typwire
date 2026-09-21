#import "../../common.typ": roundtrip, plugin

#roundtrip(plugin.roundtrip_arguments, arguments())
#roundtrip(plugin.roundtrip_arguments, arguments(1, "two", 3.0))
#roundtrip(plugin.roundtrip_arguments, arguments(a: 1, b: "two"))
#roundtrip(plugin.roundtrip_arguments, arguments(1, "two", a: 3, b: 4))
#roundtrip(
  plugin.roundtrip_arguments,
  arguments(12pt, red, nested: (1, 2, 3), tag: none),
)
