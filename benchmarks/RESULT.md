# Results

| workload | mode | baseline_ms | typwire_ms | sertyp_ms | speedup |
| --- | --- | --- | --- | --- | --- |
| noop | any | 12.6 | 9.8 | 6.7 | 0.7x |
| ints | any | 12.2 | 14.3 | 943.4 | 66.0x |
| ints | typed | 12.4 | 12.8 | 942.9 | 73.7x |
| string | any | 14.0 | 10.3 | 35.3 | 3.4x |
| string | typed | 14.4 | 9.3 | 34.5 | 3.7x |
| bytes | any | 48.7 | 8.9 | 18.6 | 2.1x |
| bytes | typed | 49.8 | 7.6 | 17.0 | 2.2x |
| objects | any | 13.5 | 26.4 | 372.4 | 14.1x |
| objects | typed | 13.5 | 21.9 | 384.3 | 17.5x |
| objects-large | any | 19.1 | 113.6 | FAILED | n/a |
| objects-large | typed | 18.6 | 87.3 | FAILED | n/a |
| nested | any | 24.1 | 46.5 | 529.9 | 11.4x |
| natives | any | 12.7 | 34.8 | FAILED | n/a |
| natives | typed | 13.7 | 32.6 | FAILED | n/a |
| natives-large | any | 16.8 | 91.7 | FAILED | n/a |
| natives-large | typed | 17.0 | 88.4 | FAILED | n/a |
| calls-small | any | 14.0 | 67.5 | 686.6 | 10.2x |
| calls-small | typed | 14.4 | 60.4 | 686.2 | 11.4x |
| calls-gradient | any | 12.8 | 54.3 | 992.9 | 18.3x |
| calls-gradient | typed | 12.3 | 52.4 | 993.4 | 19.0x |

# Plugin size

| lib | kib |
| --- | --- |
| typwire | 359 |
| sertyp | 1183 |
