#!/bin/bash

version=$(cargo run get "CHANGELOG.md" --head 1 version)
body=$(cargo run get "CHANGELOG.md" --head 1 body)
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
