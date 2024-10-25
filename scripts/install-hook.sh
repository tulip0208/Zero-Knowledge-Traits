
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
HOOKS_DIR="${DIR}/../.hooks"

git config core.hooksPath "$HOOKS_DIR"
