#!/usr/bin/env bash
# Builds the Nethermind Zisk stateless-validator guest ELF and installs it
# under the directory expected by `ere-hosts --execution-client nethermind`.
#
# Pulls Nethermind from a branch that already carries:
#   * Nethermind.Stateless.Executor: optional chain_config_json trailing
#     section in InputSerializer + SpecProvider built from it in
#     StatelessExecutor (so the guest validates blocks from any chain).
#   * tools/StatelessInputGen: --from-fixture / --chain-config-envelope
#     command-line modes plus FixtureReader / TransactionReader.
#
# No patches are applied here — anything we need lives in the source branch.
#
# Usage:
#   ./build-nethermind-guest.sh
#   ./build-nethermind-guest.sh --src /path/to/nethermind --output-dir ./nethermind-guest
#   NETHERMIND_REF=some-branch ./build-nethermind-guest.sh
#
# After a successful run, point ere-hosts at the output directory:
#
#   NETHERMIND_STATELESS_INPUT_GEN=./nethermind-guest/stateless-input-gen \
#   cargo run -p ere-hosts --release -- --zkvms zisk \
#       --bin-path ./nethermind-guest \
#       stateless-validator --execution-client nethermind
#
# Requirements: git, docker (running, supports linux/amd64), dotnet, make.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NETHERMIND_REPO_URL="${NETHERMIND_REPO_URL:-https://github.com/NethermindEth/nethermind.git}"
NETHERMIND_SRC="${NETHERMIND_SRC:-${REPO_ROOT}/.nethermind-src}"
NETHERMIND_REF="${NETHERMIND_REF:-master}"
OUTPUT_DIR="${OUTPUT_DIR:-${REPO_ROOT}/nethermind-guest}"
OUTPUT_NAME="stateless-validator-nethermind-zisk.elf"

usage() {
    cat <<'EOF'
Usage: build-nethermind-guest.sh [--src DIR] [--output-dir DIR] [--ref GIT_REF]

Options:
  --src DIR          Nethermind source checkout (default: ./.nethermind-src).
                     If the directory does not contain a git checkout, clone
                     ${NETHERMIND_REPO_URL} into it.
  --output-dir DIR   Destination for the renamed ELF (default: ./nethermind-guest).
  --ref GIT_REF      Branch / tag / commit to check out (default: master).

Environment variables overriding the same options:
  NETHERMIND_SRC, OUTPUT_DIR, NETHERMIND_REF, NETHERMIND_REPO_URL
EOF
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --src) NETHERMIND_SRC="$2"; shift 2 ;;
        --output-dir) OUTPUT_DIR="$2"; shift 2 ;;
        --ref) NETHERMIND_REF="$2"; shift 2 ;;
        -h|--help) usage; exit 0 ;;
        *) echo "Unknown argument: $1" >&2; usage >&2; exit 1 ;;
    esac
done

log() {
    printf '[build-nethermind-guest] %s\n' "$*"
}

require() {
    if ! command -v "$1" >/dev/null 2>&1; then
        echo "Missing required tool: $1" >&2
        exit 1
    fi
}

require git
require docker
require dotnet
require make

if [[ ! -d "${NETHERMIND_SRC}/.git" ]]; then
    log "Cloning ${NETHERMIND_REPO_URL} (branch ${NETHERMIND_REF}) into ${NETHERMIND_SRC}"
    git clone --recurse-submodules --branch "${NETHERMIND_REF}" "${NETHERMIND_REPO_URL}" "${NETHERMIND_SRC}"
else
    log "Using existing Nethermind checkout at ${NETHERMIND_SRC}"
    log "Fetching and checking out ${NETHERMIND_REF}"
    git -C "${NETHERMIND_SRC}" fetch --tags --quiet
    git -C "${NETHERMIND_SRC}" checkout "${NETHERMIND_REF}"
fi

GUEST_DIR="${NETHERMIND_SRC}/src/Nethermind/Nethermind.Stateless.ZiskGuest"
if [[ ! -f "${GUEST_DIR}/Makefile" ]]; then
    echo "Expected ${GUEST_DIR}/Makefile not found. Wrong branch?" >&2
    exit 1
fi

# NOTE: Do NOT strip Nethermind.Analyzers here. On master the analyzer
# ProjectReference in src/Nethermind/Directory.Build.props is already guarded by
# `'$(EnableZkEvm)' != 'true'`, so the guest build (EnableZkEvm=true) never loads
# it — no CS9057. The StatelessInputGen publish below is a normal host build that
# *needs* the analyzer: it ships the HardforkLabelsGenerator source generator that
# emits the implementation of HardforkLabels.BuildAll(); stripping it breaks that
# build with CS8795 (the ZK_EVM stub only applies to the guest).

log "Running 'make build' in ${GUEST_DIR}"
make -C "${GUEST_DIR}" build

BUILT_ELF="${GUEST_DIR}/bin/nethermind"
if [[ ! -f "${BUILT_ELF}" ]]; then
    echo "make build did not produce ${BUILT_ELF}" >&2
    exit 1
fi

mkdir -p "${OUTPUT_DIR}"
cp "${BUILT_ELF}" "${OUTPUT_DIR}/${OUTPUT_NAME}"

# Record the Nethermind commit the guest was built from. The benchmark runner
# reads this to populate the version field in metrics output paths so different
# builds don't overwrite each other.
NETHERMIND_GIT_SHA=$(git -C "${NETHERMIND_SRC}" rev-parse --short HEAD 2>/dev/null || echo unknown)
printf "%s\n" "${NETHERMIND_GIT_SHA}" > "${OUTPUT_DIR}/${OUTPUT_NAME%.elf}.version"
log "Recorded Nethermind version: ${NETHERMIND_GIT_SHA}"

# Also build & install StatelessInputGen — the host-side benchmark runner
# delegates Nethermind input serialization to this binary (single source of
# truth, mirrors the C# InputSerializer the guest expects). Published as
# self-contained linux-x64 framework-bundled directory: the launcher script
# in ${OUTPUT_DIR}/${INPUT_GEN_NAME} just shells through to it.
INPUT_GEN_DIR="${NETHERMIND_SRC}/tools/StatelessInputGen"
INPUT_GEN_NAME="stateless-input-gen"
if [[ -d "${INPUT_GEN_DIR}" ]]; then
    log "Publishing StatelessInputGen (self-contained linux-x64)"
    dotnet publish -c release -r linux-x64 --self-contained true \
        -o "${OUTPUT_DIR}/stateless-input-gen-bin" \
        "${INPUT_GEN_DIR}/StatelessInputGen.csproj" >/dev/null
    cat > "${OUTPUT_DIR}/${INPUT_GEN_NAME}" <<EOF
#!/usr/bin/env bash
# Launcher for the self-contained StatelessInputGen build.
exec "\$(dirname "\$0")/stateless-input-gen-bin/StatelessInputGen" "\$@"
EOF
    chmod +x "${OUTPUT_DIR}/${INPUT_GEN_NAME}"
    log "Installed ${OUTPUT_DIR}/${INPUT_GEN_NAME}"
fi

log "Installed ${OUTPUT_DIR}/${OUTPUT_NAME}"
log "Next step:"
log "  NETHERMIND_STATELESS_INPUT_GEN=${OUTPUT_DIR}/${INPUT_GEN_NAME} \\"
log "  cargo run -p ere-hosts --release -- --zkvms zisk \\"
log "      --bin-path ${OUTPUT_DIR} \\"
log "      stateless-validator --execution-client nethermind"
