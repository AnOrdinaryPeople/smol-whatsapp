pkgname=smol-whatsapp
pkgver=0.1.0
pkgrel=1
pkgdesc="WhatsApp Wrapper"
arch=('x86_64')
url="https://github.com/AnOrdinaryPeople/smol-whatsapp"
license=('Unlicense')
depends=('cairo' 'desktop-file-utils' 'gdk-pixbuf2' 'glib2' 'gtk3' 'hicolor-icon-theme' 'pango' 'webkit2gtk-4.1')
options=('!strip' '!emptydirs')
install=${pkgname}.install
source=("local://${pkgname}.deb")
package() {
  tar -xvf data.tar.gz -C "${pkgdir}"
}
