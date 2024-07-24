#!/usr/bin/env bash

#
# Copyright © 2024 the original author or authors.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#

# shellcheck disable=SC2128
CURRENT_DIR="$(cd "$(dirname "$BASH_SOURCE")" && pwd)"
# shellcheck disable=SC2164
cd "$CURRENT_DIR"
echo "current path: $CURRENT_DIR"

set -euo pipefail
IFS=$'\n\t'

if [ $# -eq 0 ]; then
    echo "Usage: publish.sh [SUBMODULE]"
    exit 1
fi

cargo publish --manifest-path "crates/$1/Cargo.toml"
