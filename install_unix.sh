#! /bin/bash

if [ ! -d .git ]; then
    echo "ERROR: .git folder not found. Please run this script in the root directory of your UpOrDawn git repository."
    exit 1
fi

release="x86_64-unknown-linux-gnu"

if [ "$(uname)" == "Darwin" ]; then
    release="x86_64-apple-darwin"
fi

curl -LO https://github.com/Snoupix/pre-commit_upordawn/releases/download/$release/pre-commit
chmod +x pre-commit
mv pre-commit .git/hooks/pre-commit

echo "Installed pre-commit hook successfully."
exit 0
