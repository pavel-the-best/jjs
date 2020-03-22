#!/usr/bin/env bash

versionle() {
    [ "$1" = "$(printf '%s\n%s' "$1" "$2" | sort -V | head -n1)" ]
}

if [[ "x$JJS_GO_SKIP" != "x" ]]; then
    exit 1;
fi
go_path="$( command -v go )"
if [[ -z "$go_path" ]]; then
    echo "go not found";
    exit 1;
fi
version="$(go version | grep -Po "([0-9]\.?){3,}")"
if [[ $(versionle "$version" "1.13") && "$version" != "1.13" ]]; then
    echo "need go at least version 1.13"
    exit 1;
fi
mkdir GOPATH
go env -w GOPATH="$(readlink -f GOPATH)"
go env -w GOROOT=
"$go_path" get -u golang.org/x/tools/go/packages
"$go_path" run "$DATA/generate.go" > "program.go"
echo "found go at $go_path"
echo "set-env:GO=$go_path" >> "$1"
