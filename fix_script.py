import re
with open('scripts/update_package_manifests.sh', 'r') as f:
    c = f.read()

c = c.replace('sed -i "s/sha256 \\"__SOURCE_SHA__\\"/sha256 \\"${SOURCE_SHA}\\"/" packaging/homebrew/lavaterm.rb', 'sed -i "s/__SOURCE_SHA__/${SOURCE_SHA}/g" packaging/homebrew/lavaterm.rb')
c = c.replace("sed -i \"s/sha256sums=('__SOURCE_SHA__')/sha256sums=('${SOURCE_SHA}')/\" packaging/aur/PKGBUILD", "sed -i \"s/__SOURCE_SHA__/${SOURCE_SHA}/g\" packaging/aur/PKGBUILD")
c = c.replace("sed -i \"s/sha256sums = .*/sha256sums = ${SOURCE_SHA}/\" packaging/aur/.SRCINFO", "sed -i \"s/__SOURCE_SHA__/${SOURCE_SHA}/g\" packaging/aur/.SRCINFO")

with open('scripts/update_package_manifests.sh', 'w') as f:
    f.write(c)
