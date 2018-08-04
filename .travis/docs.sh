#!/bin/bash

set -e

git clone --branch gh-pages "https://$GH_TOKEN@github.com/${TRAVIS_REPO_SLUG}.git" deploy_docs > /dev/null 2>&1
cd deploy_docs

git config user.name "Constantin Nickel"
git config user.email "constantin.nickel@gmail.com"

if [ "$TRAVIS_TAG" = "" ]; then
    rm -rf master
    mv ../target/doc ./master
    echo '<meta http-equiv="refresh" content="0;url=modio/index.html">' > ./master/index.html
else
    rm -rf $TRAVIS_TAG
    mv ../target/doc ./$TRAVIS_TAG
    echo '<meta http-equiv="refresh" content="0;url=modio/index.html">' > ./$TRAVIS_TAG/index.html

    latest=$(echo * | tr " " "\n" | sort -V -r | head -n1)
    if [ "$TRAVIS_TAG" = "$latest" ]; then
        echo "<meta http-equiv=refresh content=\"0;url=$latest/modio/index.html\">" > index.html
    fi
fi

git add -A .
if git commit -m "rebuild pages at ${TRAVIS_COMMIT}" > /dev/null 2>&1; then
    echo
    echo "Pushing docs..."
    git push --quiet origin gh-pages > /dev/null 2>&1
    echo
    echo "Docs published."
else
    echo
    echo "Nothing changed."
fi
