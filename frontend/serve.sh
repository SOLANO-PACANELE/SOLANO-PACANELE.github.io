#!/bin/bash
set -ex
cd "$(dirname "${BASH_SOURCE[0]}")"

dx serve --platform web --package web_pacanele 
