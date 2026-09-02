import re
with open('.github/workflows/release.yml', 'r') as f:
    c = f.read()

# Replace the push logic
old_push = """          # Commit and push
          git config --global user.name "github-actions[bot]"
          git config --global user.email "github-actions[bot]@users.noreply.github.com"
          git add packaging/
          git commit -m "chore(release): update package manifests for v${VERSION} [skip ci]" || true
          git push origin HEAD:${{ github.ref_name }} || true"""

new_push = """          # Commit and push to dev/main, not the tag
          git config --global user.name "github-actions[bot]"
          git config --global user.email "github-actions[bot]@users.noreply.github.com"
          git add packaging/
          
          # Fetch and checkout dev branch to update manifests there
          git fetch origin dev
          git checkout dev
          
          # Apply the sed changes again to the checked out branch
          sed -i "s/__SOURCE_SHA__/${SOURCE_SHA}/g" packaging/homebrew/lavaterm.rb packaging/aur/PKGBUILD packaging/aur/.SRCINFO packaging/arch/PKGBUILD
          sed -i "s/v[0-9]\+\\.[0-9]\+\\.[0-9]\+/v${VERSION}/g" packaging/homebrew/lavaterm.rb
          sed -i "s/pkgver=[0-9]\+\\.[0-9]\+\\.[0-9]\+/pkgver=${VERSION}/g" packaging/aur/PKGBUILD packaging/arch/PKGBUILD
          sed -i "s/pkgver = [0-9]\+\\.[0-9]\+\\.[0-9]\+/pkgver = ${VERSION}/g" packaging/aur/.SRCINFO
          
          git add packaging/
          git commit -m "chore(release): update package manifests for v${VERSION} [skip ci]"
          git push origin dev"""

if old_push in c:
    c = c.replace(old_push, new_push)
else:
    print("Could not find old_push")

# Also remove `|| true` from wget and ensure wget fails if it can't download
# Actually I used `wget -qO` which doesn't have `|| true`, but we should add `set -e` or check exit code.
# The `run` block usually has `set -e` by default in GitHub Actions.
# Let's just make sure wget doesn't fail silently.

c = c.replace('wget -qO source.tar.gz', 'wget --fail --show-progress -O source.tar.gz')
c = c.replace('          SOURCE_SHA=$(sha256sum source.tar.gz | awk \'{print $1}\')\n', '          test -s source.tar.gz\n          SOURCE_SHA=$(sha256sum source.tar.gz | awk \'{print $1}\')\n')

with open('.github/workflows/release.yml', 'w') as f:
    f.write(c)
