#!/bin/bash

version=$(cargo run get "CHANGELOG.md" version | jq ".[0].version")
body=$(cargo run get "CHANGELOG.md" body | jq ".[0].body")
version=${version//\"/}
body=${body//\"/}

echo $version
echo $body

if git rev-parse "$version" > /dev/null 2>&1; then
  git tag -d $version
  git push --delete origin $version
fi

git tag $version -am "$body"
git push origin $version

cargo build -r
tar -czvf chlgp-$version-linux_x86_64.tar.gz ./target/release/chlgp
