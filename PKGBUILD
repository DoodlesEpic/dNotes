# Maintainer: Doodles <moraes.eduardo@proton.me>

_realname=gtknotes
pkgbase=mingw-w64-${_realname}
pkgname="${MINGW_PACKAGE_PREFIX}-${_realname}"
pkgver=42.0
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
source=(https://download.gnome.org/sources/${_realname}/${pkgver%.*}/${_realname}-${pkgver}.tar.xz
        "0001-workaround-no-delayed-loading.patch")
sha256sums=('a87991f42961eb4f6abcdbaabb784760c23aeaeefae6363d3e21a61e9c458437'
            '96f71347451a4443e29135853abea2260e3d4cc32b1691569f2669f18720dd52')

prepare() {
  cd "${srcdir}"/${_realname}-${pkgver}

  # originally from gedit
  # gedit requires delayed loading which isn't supported by meson
  # yet, so move the lib to /bin.
  # https://github.com/mesonbuild/meson/issues/5093
  patch -Np1 -i "${srcdir}"/0001-workaround-no-delayed-loading.patch
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
