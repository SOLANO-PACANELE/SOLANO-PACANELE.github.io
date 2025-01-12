#!/bin/bash
set -ex
cd "$(dirname "${BASH_SOURCE[0]}")"

dx build --release --platform web --package web_pacanele


rm -rf ../.github_pages
cp -a target/dx/web_pacanele/release/web/public/ ../.github_pages
cd ../.github_pages
echo '/* /index.html 200' > _redirects
cp index.html 404.html
