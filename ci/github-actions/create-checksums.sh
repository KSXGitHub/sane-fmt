#!/bin/sh
cd ./flatten || exit $?
sha1sum ./* > ../sha1sum.txt || exit $?
sha256sum ./* > ../sha256sum.txt || exit $?
sha512sum ./* > ../sha512sum.txt || exit $?
