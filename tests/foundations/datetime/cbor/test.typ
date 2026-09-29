#import "../../../common.typ": roundtrip-format, plugin, typwire

#roundtrip-format(typwire.cbor, plugin.roundtrip-datetime-cbor, datetime(year: 2026, month: 8, day: 25))
#roundtrip-format(typwire.cbor, plugin.roundtrip-datetime-cbor, datetime(hour: 13, minute: 5, second: 0))
#roundtrip-format(typwire.cbor, plugin.roundtrip-datetime-cbor,
  datetime(year: 1999, month: 12, day: 31, hour: 23, minute: 59, second: 58),
)
