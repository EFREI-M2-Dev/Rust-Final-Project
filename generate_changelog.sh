#!/bin/bash

if [ -z "$1" ]; then
    echo "Usage: $0 <version_number>"
    exit 1
fi

VERSION_NUMBER=$1
CHANGELOG_FILE="changelog.txt"

git tag -a "v$VERSION_NUMBER" -m "Release version $VERSION_NUMBER"

echo "Changelog" > $CHANGELOG_FILE
echo "" >> $CHANGELOG_FILE
echo "Version $VERSION_NUMBER" >> $CHANGELOG_FILE
echo "" >> $CHANGELOG_FILE

LAST_TAG=$(git describe --tags --abbrev=0 @^ 2>/dev/null)

if [ -n "$LAST_TAG" ]; then
    git log $LAST_TAG..HEAD --pretty=format:"- %s" >> $CHANGELOG_FILE
else
    echo "Aucun tag précédent trouvé. Inclure tous les commits depuis le début." >> $CHANGELOG_FILE
    git log --pretty=format:"- %s" >> $CHANGELOG_FILE
fi

echo "" >> $CHANGELOG_FILE
echo "Changelog généré avec succès dans $CHANGELOG_FILE"
