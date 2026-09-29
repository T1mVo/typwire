#import "../../../common.typ": plugin, typwire

// Many small objects rather than one huge value -- stresses the recursive
// tree-walking side of encode/decode (many `Serialize`/`Deserialize` calls
// and many `wire.encode`/`decode` recursion steps) instead of the raw
// byte-count side `stress/bytes` covers.
#let count = 6000
#let values = range(count).map(i => (x: i, y: i * 2, label: "item-" + str(i)))

#let result = typwire.yaml.call(plugin.stress-many-objects-yaml, values)
#assert.eq(result.len(), count)
#assert.eq(result, values)
