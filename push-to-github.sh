#!/bin/bash

# Push changes to GitHub
# Usage: ./push-to-github.sh "Your commit message"

set -e

if [ $# -eq 0 ]; then
    echo "Usage: ./push-to-github.sh \"Your commit message\""
    echo "Example: ./push-to-github.sh \"Fix icon sizes\""
    exit 1
fi

COMMIT_MSG="$1"

echo "🔍 Checking git status..."
git status

echo ""
echo "📦 Adding all changes..."
git add .

echo ""
echo "💾 Committing with message: $COMMIT_MSG"
git commit -m "$COMMIT_MSG"

echo ""
echo "🚀 Pushing to GitHub..."
git push origin main

echo ""
echo "✅ Successfully pushed to GitHub!"
echo ""
echo "📋 Next steps:"
echo "1. Go to https://github.com/11dj/heic2jpg-converter/actions"
echo "2. Wait for the build to complete"
echo "3. Download your builds from the artifacts"
