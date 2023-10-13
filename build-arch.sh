#!/bin/bash
# Build and install for Arch linux.
set -e

if [ ! -d build ]; then
  mkdir build
fi
pushd build
cmake ..
rm -rf temp
mkdir temp
mv PKGBUILD temp
cd temp
makepkg -f
yay -U *.pkg.*
popd
