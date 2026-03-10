set dotenv-load := true

default:
    just --list

# Tag and release a new version - custom script by carlo.

# Usage: just tag
alias t := tag
tag:
    sh tag_and_release.sh
