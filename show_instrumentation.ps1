[CmdletBinding()]
param()

# set PATH=%PATH%;C:\Users\TheBeast\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\bin\
$PROJECT_NAME_UNDERSCORE = 'rusty_ray_tracer'
$env:RUSTFLAGS = '-C instrument-coverage'
$env:LLVM_PROFILE_FILE = "target/debug/coverage/$PROJECT_NAME_UNDERSCORE-%p-%m.profraw"
$IGNORE_PATTERN = '(.*\.cargo[/\\]+registry[/\\]+.*)|(.*\.rustup[/\\]+.*)|(.*test.*)'

$cargo_arguments = '+nightly'

cargo $cargo_arguments test --no-fail-fast $CARGO_OPTIONS

# set execs=--object target\debug\deps\canvas_feature-efef0e4a2bf4f757.exe --object target\debug\deps\captures-6f61d5583fc9e11f.exe --object target\debug\deps\example-a5c3f12cee7d627a.exe --object target\debug\deps\rusty_ray_tracer-893708ccb3b75b85.exe --object target\debug\deps\tuples_feature-b8efade23724b769.exe
$executables = cargo $cargo_arguments test --tests --no-run --message-format=json | ConvertFrom-Json | Where-Object { $_.profile.test -and $_.executable -ne $null } | Select-Object -ExpandProperty executable

Write-Verbose "cargo $cargo_arguments profdata -- merge -sparse target/debug/coverage/$PROJECT_NAME_UNDERSCORE-*.profraw -o target/debug/coverage/merged.profdata"
cargo $cargo_arguments profdata -- merge -sparse target/debug/coverage/$PROJECT_NAME_UNDERSCORE-*.profraw -o target/debug/coverage/merged.profdata

Write-Verbose "cargo $cargo_arguments cov -- export $executables --instr-profile=target/debug/coverage/merged.profdata --ignore-filename-regex=`"$IGNORE_PATTERN`" --skip-functions --format=lcov >lcov.info"
$llvmcov = cargo $cargo_arguments cov -- export $executables --instr-profile=target/debug/coverage/merged.profdata --ignore-filename-regex="$IGNORE_PATTERN" --skip-functions --format=lcov >lcov.info

# $llvmcov | cargo llvm-codecov-converter > lcov.info

# # llvm-cov report --use-color -ignore-filename-regex="$IGNORE_PATTERN" --instr-profile=target/debug/coverage/merged.profdata $executables
# llvm-cov show   --use-color -ignore-filename-regex="$IGNORE_PATTERN" --instr-profile=target/debug/coverage/merged.profdata $executables --show-instantiations --show-line-counts-or-regions --Xdemangler=rustfilt > show-instrumentation.log