set PATH=%PATH%;C:\Users\TheBeast\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\bin\

$executables=cargo +nightly test --tests --no-run --message-format=json | ConvertFrom-Json | where {$_.profile.test -and  $_.executable -ne $null} | select  -ExpandProperty executable

llvm-profdata merge -sparse default_*.profraw -o json5format.profdata

set execs=--object target\debug\deps\canvas_feature-efef0e4a2bf4f757.exe --object target\debug\deps\captures-6f61d5583fc9e11f.exe --object target\debug\deps\example-a5c3f12cee7d627a.exe --object target\debug\deps\rusty_ray_tracer-893708ccb3b75b85.exe --object target\debug\deps\tuples_feature-b8efade23724b769.exe

llvm-cov report --use-color -ignore-filename-regex='.*cargo.*' --instr-profile=json5format.profdata %execs%
llvm-cov show   --use-color -ignore-filename-regex='.*cargo.*' --instr-profile=json5format.profdata %execs% --show-instantiations --show-line-counts-or-regions --Xdemangler=rustfilt > show-instrumentation.log