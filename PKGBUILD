# Maintainer: Doodles <moraes.eduardo@proton.me>

_realname=gtknotes
pkgbase=mingw-w64-${_realname}
pkgname="${MINGW_PACKAGE_PREFIX}-${_realname}"
pkgver=0.1.0
pkgrel=1
arch=('any')
mingw_arch=('mingw32' 'mingw64' 'ucrt64' 'clang64' 'clang32')
pkgdesc="A free and open source cross platform note taking application"
depends=("${MINGW_PACKAGE_PREFIX}-adwaita-icon-theme"
         "${MINGW_PACKAGE_PREFIX}-appstream-glib"
         "${MINGW_PACKAGE_PREFIX}-gsettings-desktop-schemas"
         "${MINGW_PACKAGE_PREFIX}-gtksourceview4"
         "${MINGW_PACKAGE_PREFIX}-gtk3"
         "${MINGW_PACKAGE_PREFIX}-libpeas"
         "${MINGW_PACKAGE_PREFIX}-python-gobject"
         "${MINGW_PACKAGE_PREFIX}-gspell"
         "${MINGW_PACKAGE_PREFIX}-gobject-introspection-runtime")
makedepends=("${MINGW_PACKAGE_PREFIX}-gobject-introspection"
             "${MINGW_PACKAGE_PREFIX}-gtk-doc"
             "${MINGW_PACKAGE_PREFIX}-gettext"
             "${MINGW_PACKAGE_PREFIX}-itstool"
             "${MINGW_PACKAGE_PREFIX}-meson"
             "${MINGW_PACKAGE_PREFIX}-pkg-config"
             "${MINGW_PACKAGE_PREFIX}-python"
             "${MINGW_PACKAGE_PREFIX}-vala"
             "${MINGW_PACKAGE_PREFIX}-yelp-tools"
             "${MINGW_PACKAGE_PREFIX}-cc")
options=('strip' 'staticlibs')
license=("GPL")
url="https://www.doodles.dev"
install=${_realname}-${MSYSTEM}.install
source=(https://github.com/DoodlesEpic/QtNotes/archive/refs/tags/v${pkgver}.tar.gz)
sha256sums=('479d186d960ec5b3cb7ea79af5b2337306593ffa828d08a6c74ee017a87e4304')

prepare() {
  cd "${srcdir}"/${_realname}-${pkgver}
}

build() {
  [[ -d build-${MINGW_CHOST} ]] && rm -rf build-${MINGW_CHOST}
  mkdir -p build-${MINGW_CHOST} && cd build-${MINGW_CHOST}

  MSYS2_ARG_CONV_EXCL="--prefix=" \
  ${MINGW_PREFIX}/bin/meson.exe \
    --prefix=${MINGW_PREFIX} \
    --wrap-mode=nodownload \
    --default-library=both \
    --buildtype=plain \
    "../${_realname}-${pkgver}"

  ${MINGW_PREFIX}/bin/meson.exe compile
}

package() {
  cd "${srcdir}/build-${MINGW_CHOST}"
  DESTDIR="${pkgdir}" ${MINGW_PREFIX}/bin/meson.exe install

  install -Dm644 "${srcdir}/${_realname}-${pkgver}/COPYING" "${pkgdir}${MINGW_PREFIX}/share/licenses/${_realname}/COPYING"
}
