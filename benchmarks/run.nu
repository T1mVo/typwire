#!/usr/bin/env nu

const bench_dir: string = path self | path dirname
const root_dir: string = $bench_dir | path dirname
const target_dir: string = $bench_dir | path join target
const results_dir: string = $bench_dir | path join results
const wasm_dir: string = $bench_dir | path join target wasm32-unknown-unknown release

const all_workloads = [
    noop
    ints
    string
    bytes
    objects
    objects-large
    nested
    natives
    natives-large
    calls-small
    calls-gradient
]

const libs = [typwire sertyp]
const untyped_workloads = [noop nested]

def typst_cmd [
    workload: string
    mode: string
    lib: string
    verify: bool
] {
    [
        typst
        compile
        --root
        $root_dir
        --input
        $"workload=($workload)"
        --input
        $"lib=($lib)"
        --input
        $"mode=($mode)"
        --input
        $"verify=($verify)"
        ($bench_dir | path join bench.typ)
        ($target_dir | path join out.pdf)
    ]
}

# `true` if the case compiles and its round trip is lossless.
def verify_case [workload: string, mode: string, lib: string] {
    let cmd = typst_cmd $workload $mode $lib true
    let res = do { ^$cmd.0 ...($cmd | skip 1) } | complete
    if $res.exit_code != 0 {
        {
            ok: false
            error: (
                $res.stderr
                | lines
                | where $it starts-with "error"
                | first
                | default "unknown error"
            )
        }
    } else {
        {ok: true, error: ""}
    }
}

def build [] {
    cd $bench_dir
    cargo build --release --target wasm32-unknown-unknown
    mkdir $target_dir
    for lib in $libs {
        let src = $wasm_dir | path join $"bench_($lib)_plugin.wasm"
        let dst = $target_dir | path join $"($lib).wasm"
        # Same flags as scripts/test.nu, applied to both plugins alike.
        wasm-opt -O4 --enable-bulk-memory-opt --enable-simd $src -o $dst
    }
}

export def main [
    ...workloads: string  # workloads to run (default: all)
    --runs: int = 10      # timed runs per case
    --warmup: int = 2     # untimed warmup runs per case
    --no-build            # skip cargo/wasm-opt, reuse target/*.wasm
] {
    let workloads = if ($workloads | is-empty) { $all_workloads } else { $workloads }
    if not $no_build { build }
    mkdir $results_dir

    let sizes = $libs | each {|lib|
        {lib: $lib, kib: ((ls ($target_dir | path join $"($lib).wasm") | get 0.size) / 1KiB | math round)}
    }

    let cases = $workloads | each {|w|
        let modes = if $w in $untyped_workloads { [any] } else { [any typed] }
        $modes | each {|m| {w: $w, mode: $m} }
    } | flatten

    let rows = $cases | each {|c|
        let w = $c.w
        let mode = $c.mode
        print $"== ($w) \(($mode)\)"
        let status = $libs | each {|lib| {lib: $lib} | merge (verify_case $w $mode $lib) }
        for s in ($status | where not ok) { print $"   ($s.lib) failed: ($s.error)" }
        let runnable = $status | where ok | get lib

        let json = $results_dir | path join $"($w)-($mode).json"
        let cmds = ["none"] ++ $runnable | each {|lib| (typst_cmd $w $mode $lib false) | str join " " | $"'($in)'" }
        let names = ["none"] ++ $runnable
        let args = $names | zip $cmds | each {|p| ["--command-name" $p.0 ($p.1 | str trim --char "'")] } | flatten
        ^hyperfine --warmup $warmup --runs $runs --export-json $json ...$args | ignore

        # Median, not mean: wall-clock outliers (disk cache, background load)
        # would otherwise skew a subtraction of two small numbers.
        let results = open $json | get results
        let base = $results | where command == none | get 0.median
        let net = {|lib|
            let r = $results | where command == $lib
            if ($r | is-empty) { null } else { ($r | get 0.median) - $base }
        }
        let ms = {|x| if $x == null { null } else { $x * 1000 | math round --precision 1 } }
        let t = do $net typwire | do $ms $in
        let s = do $net sertyp | do $ms $in
        {
            workload: $w
            mode: $mode
            baseline_ms: (do $ms $base)
            typwire_ms: ($t | default "FAILED")
            sertyp_ms: ($s | default "FAILED")
            speedup: (if $t != null and $s != null and $t > 0 { $"($s / $t | math round --precision 1)x" } else { "n/a" })
        }
    }

    let table = $rows

    print ""
    print "Net time per workload and mode (median wall time minus payload-construction baseline); speedup = sertyp / typwire"
    print $table
    print "Plugin size (wasm-opt -O4):"
    print $sizes

    let report = [
        "# Results"
        ""
        ($table | to md)
        ""
        "# Plugin size"
        ""
        ($sizes | to md)
        ""
    ] | str join "\n"
    $report | save -f ($bench_dir | path join RESULT.md)
    print $"Wrote ($bench_dir | path join RESULT.md); raw hyperfine JSON in ($results_dir)"
}
