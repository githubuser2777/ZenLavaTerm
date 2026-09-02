import re
with open('.github/workflows/release.yml', 'r') as f:
    c = f.read()

c = c.replace('sed -i "s/sha256 \\"__SOURCE_SHA__\\"/sha256 \\"${SOURCE_SHA}\\"/"', 'sed -i "s/__SOURCE_SHA__/${SOURCE_SHA}/g"')
# wait, the simplest is just to replace all __SOURCE_SHA__
c = re.sub(r'sed -i .* packaging/homebrew/lavaterm.rb\n', '', c)
c = re.sub(r'sed -i .* packaging/aur/PKGBUILD\n', '', c)
c = re.sub(r'sed -i .* packaging/aur/\.SRCINFO\n', '', c)
c = re.sub(r'sed -i .* packaging/arch/PKGBUILD\n', '', c)

replacement = """
          sed -i "s/__SOURCE_SHA__/${SOURCE_SHA}/g" packaging/homebrew/lavaterm.rb packaging/aur/PKGBUILD packaging/aur/.SRCINFO packaging/arch/PKGBUILD
          sed -i "s/v[0-9]\\+\\.[0-9]\\+\\.[0-9]\\+/v${VERSION}/g" packaging/homebrew/lavaterm.rb
          sed -i "s/pkgver=[0-9]\\+\\.[0-9]\\+\\.[0-9]\\+/pkgver=${VERSION}/g" packaging/aur/PKGBUILD packaging/arch/PKGBUILD
          sed -i "s/pkgver = [0-9]\\+\\.[0-9]\\+\\.[0-9]\\+/pkgver = ${VERSION}/g" packaging/aur/.SRCINFO
"""

c = c.replace('          # Replace placeholders with real hash\n', '          # Replace placeholders with real hash\n' + replacement)

with open('.github/workflows/release.yml', 'w') as f:
    f.write(c)
