#!/bin/bash

if [ -z "$1" ]
  then
    echo "Usage: deb-package.sh <version>"
    echo ""
    echo "Package the given version number into a .deb package."
    echo "Example: deb-package.sh 0.10.7"
    exit 1
fi

DESTDIR=/usr/bin
DOCDIR=/usr/share/man/
NAME="eza"
VERSION=$1
ARCH="amd64"
DEB_TMP_DIR="${NAME}_${VERSION}_${ARCH}"
DEB_PACKAGE="${NAME}_${VERSION}_${ARCH}.deb"

read -r -d '' DEB_CONTROL << EOM
Package: ${NAME}
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: ${ARCH}
Depends: libc6
Maintainer: Sandro-Alessio Gierens <sandro@gierens.de>
Description: Modern replacement for ls
 eza is a modern replacement for ls.  It uses colours for information by
 default, helping you distinguish between many types of files, such as whether
 you are the owner, or in the owning group.
 .
 It also has extra features not present in the original ls, such as viewing the
 Git status for a directory, or recursing into directories with a tree view.
EOM

read -r -d '' DEB_COPYRIGHT << EOM
Format: http://www.debian.org/doc/packaging-manuals/copyright-format/1.0/
Upstream-Name: ${NAME}
Upstream-Contact: Christina Sørensen <christina@cafkafk.com>
Source: https://github.com/eza-community/eza/releases

Files: *
License: MIT
Copyright: 2023 Christina Sørensen <christina@cafkafk.com>

Files: debian/*
License: MIT
Copyright: 2023 Christina Sørensen <christina@cafkafk.com>

License: MIT
 Permission is hereby granted, free of charge, to any person obtaining a copy
 of this software and associated documentation files (the "Software"), to deal
 in the Software without restriction, including without limitation the rights
 to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 copies of the Software, and to permit persons to whom the Software is
 furnished to do so, subject to the following conditions:
 .
 The above copyright notice and this permission notice shall be included in all
 copies or substantial portions of the Software.
 .
 THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 SOFTWARE.
EOM

# create temporary build directory
mkdir -p ${DEB_TMP_DIR}

# create directory structure
mkdir -p ${DEB_TMP_DIR}${DESTDIR}
mkdir -p ${DEB_TMP_DIR}${DOCDIR}
mkdir -p ${DEB_TMP_DIR}${DOCDIR}/man1
mkdir -p ${DEB_TMP_DIR}${DOCDIR}/man5
mkdir -p ${DEB_TMP_DIR}/DEBIAN
mkdir -p ${DEB_TMP_DIR}/usr/share/doc/${NAME}

# fix directory permissions
chmod 755 -R ${DEB_TMP_DIR}

# binary
cp target/release/${NAME} ${DEB_TMP_DIR}${DESTDIR}
chmod 755 ${DEB_TMP_DIR}${DESTDIR}/${NAME}

# man page
gzip -cn9 target/man/eza.1 > ${DEB_TMP_DIR}${DOCDIR}man1/eza.1.gz
gzip -cn9 target/man/eza_colors.5 > ${DEB_TMP_DIR}${DOCDIR}man5/eza_colors.5.gz
gzip -cn9 target/man/eza_colors-explanation.5 > ${DEB_TMP_DIR}${DOCDIR}man5/eza_colors-explanation.5.gz
chmod 644 ${DEB_TMP_DIR}${DOCDIR}/**/*.gz

# control file
touch ${DEB_TMP_DIR}/DEBIAN/control
echo "${DEB_CONTROL}" > ${DEB_TMP_DIR}/DEBIAN/control
chmod 644 ${DEB_TMP_DIR}/DEBIAN/control

# changelog
cp CHANGELOG.md ${DEB_TMP_DIR}/usr/share/doc/${NAME}/changelog
gzip -cn9 ${DEB_TMP_DIR}/usr/share/doc/${NAME}/changelog > ${DEB_TMP_DIR}/usr/share/doc/${NAME}/changelog.gz
rm ${DEB_TMP_DIR}/usr/share/doc/${NAME}/changelog
chmod 644 ${DEB_TMP_DIR}/usr/share/doc/${NAME}/changelog.gz

# copyright file
touch ${DEB_TMP_DIR}/usr/share/doc/${NAME}/copyright
echo "${DEB_COPYRIGHT}" > ${DEB_TMP_DIR}/usr/share/doc/${NAME}/copyright
chmod 644 ${DEB_TMP_DIR}/usr/share/doc/${NAME}/copyright

# build package
dpkg-deb --build --root-owner-group ${DEB_TMP_DIR}

# clean up
rm -rf ${DEB_TMP_DIR}

# test package
lintian ${DEB_PACKAGE}
