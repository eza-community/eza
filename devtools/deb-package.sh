#!/bin/bash -e

REPO_URL="https://github.com/eza-community/eza"
NAME="eza"
DESTDIR=/usr/bin
DOCDIR=/usr/share/man/

TAG=$(git describe --tags --abbrev=0)
VERSION=${TAG:1}

echo "checkout tag ${TAG}"
git checkout --quiet "${TAG}"

echo "build man pages"
just man

declare -A TARGETS
TARGETS["amd64"]="x86_64-unknown-linux-gnu"
TARGETS["arm64"]="aarch64-unknown-linux-gnu"
TARGETS["armhf"]="arm-unknown-linux-gnueabihf"

echo "download release notes"
RELEASE_NOTES=$(curl -s "${REPO_URL}/releases/tag/${TAG}")

for ARCH in "${!TARGETS[@]}"; do
    echo "building ${ARCH} package:"

    DEB_TMP_DIR="${NAME}_${VERSION}_${ARCH}"
    DEB_PACKAGE="${NAME}_${VERSION}_${ARCH}.deb"

    TARGET=${TARGETS[$ARCH]}
    echo " -> downloading ${TARGET} archive"
    wget -q -O "${ARCH}.tar.gz" "${REPO_URL}/releases/download/${TAG}/${NAME}_${TARGET}.tar.gz"

    echo " -> verifying ${TARGET} archive"
    CHECKSUM=$(md5sum "${ARCH}.tar.gz" | cut -d ' ' -f 1)
    echo "    checksum: ${CHECKSUM}"
    grep -q "${CHECKSUM}" <<< "${RELEASE_NOTES}" \
        || (echo "checksum mismatch" && exit 1)
    echo "    checksum ok"

    echo " -> creating directory structure"
    mkdir -p "${DEB_TMP_DIR}"
    mkdir -p "${DEB_TMP_DIR}${DESTDIR}"
    mkdir -p "${DEB_TMP_DIR}${DOCDIR}"
    mkdir -p "${DEB_TMP_DIR}${DOCDIR}/man1"
    mkdir -p "${DEB_TMP_DIR}${DOCDIR}/man5"
    mkdir -p "${DEB_TMP_DIR}/DEBIAN"
    mkdir -p "${DEB_TMP_DIR}/usr/share/doc/${NAME}"
    chmod 755 -R "${DEB_TMP_DIR}"

    echo " -> extract executable"
    tar -xzf "${ARCH}.tar.gz"
    cp ${NAME} "${DEB_TMP_DIR}${DESTDIR}"
    chmod 755 "${DEB_TMP_DIR}${DESTDIR}/${NAME}"

    echo " -> compress man pages"
    gzip -cn9 target/man/eza.1 > "${DEB_TMP_DIR}${DOCDIR}man1/eza.1.gz"
    gzip -cn9 target/man/eza_colors.5 > "${DEB_TMP_DIR}${DOCDIR}man5/eza_colors.5.gz"
    gzip -cn9 target/man/eza_colors-explanation.5 > "${DEB_TMP_DIR}${DOCDIR}man5/eza_colors-explanation.5.gz"
    chmod 644 "${DEB_TMP_DIR}${DOCDIR}"/**/*.gz

    echo " -> create control file"
    touch "${DEB_TMP_DIR}/DEBIAN/control"
    cat > "${DEB_TMP_DIR}/DEBIAN/control" <<EOM
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
    chmod 644 "${DEB_TMP_DIR}/DEBIAN/control"

    echo " -> copy changelog"
    cp CHANGELOG.md "${DEB_TMP_DIR}/usr/share/doc/${NAME}/changelog"
    gzip -cn9 "${DEB_TMP_DIR}/usr/share/doc/${NAME}/changelog" > "${DEB_TMP_DIR}/usr/share/doc/${NAME}/changelog.gz"
    rm "${DEB_TMP_DIR}/usr/share/doc/${NAME}/changelog"
    chmod 644 "${DEB_TMP_DIR}/usr/share/doc/${NAME}/changelog.gz"

    echo " -> create copyright file"
    touch "${DEB_TMP_DIR}/usr/share/doc/${NAME}/copyright"
    cat > "${DEB_TMP_DIR}/usr/share/doc/${NAME}/copyright" << EOM
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
    chmod 644 "${DEB_TMP_DIR}/usr/share/doc/${NAME}/copyright"

    echo " -> build ${ARCH} package"
    dpkg-deb --build --root-owner-group "${DEB_TMP_DIR}" > /dev/null

    echo " -> cleanup"
    rm -rf "${DEB_TMP_DIR}" "${ARCH}.tar.gz" "${NAME}"

    # gierens: this does not work on my arch at the moment and
    #          i'm verifying on the repo host anyway thus the || true
    echo " -> lint ${ARCH} package"
    lintian "${DEB_PACKAGE}" || true
done
