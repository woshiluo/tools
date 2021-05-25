# Maintainter: Woshiluo Luo <woshiluo.luo@outlook.com>
pkgname=woshiluo-tools
_pkgname=tools
pkgver=28a2bf
pkgrel=1
pkgdesc="小工具集合"
arch=('x86_64' 'i686')
url="https://github.com/woshiluo/tools"
license=('AGPL3')
makedepends=('git')
source=(git+https://github.com/woshiluo/tools)

md5sums=('SKIP')

pkgver() {
	cd "$srcdir/$_pkgname"

	echo $(git rev-list --all --max-count=1 | cut -b 1-6)
}

build() {
	cd "$srcdir/$_pkgname"

	for repo in ./*; do
		if [ -d $repo ]; then
			cd $repo
			cargo build --release
			cd ..
		fi
	done
}

package() {
	cd "$srcdir/$_pkgname"

	mkdir -p $pkgdir/usr/bin
	for repo in ./*; do
		if [ -d $repo ]; then
			cd $repo
			find target/release \
				-maxdepth 1 \
				-executable \
				-type f \
				-exec install -m 755 "{}" "$pkgdir"/usr/bin \;
			cd ..
		fi
	done
}
