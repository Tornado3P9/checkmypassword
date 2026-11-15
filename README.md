# checkmypassword

for experimenting with github actions, not intended for professional use

```bash
# simple tags
git tag v0.1.0
git push origin v0.1.0

# simple commits
git add -A
git commit -m "test"

# Signierung von commits und tags mit SSH key oder GPG (hier test mit SSH):
git config gpg.format ssh  # git config --show-origin --get gpg.format
git config user.signingkey ~/.ssh/id_ed25519.pub  # git config --show-origin --get user.signingkey
# oder: git config --show-origin --get-regexp '^(gpg\.format|user\.signingkey)$'

# lass automatisch signieren:
git config commit.gpgsign true  # git config --get commit.gpgsign
# Wenn true, dann brauchst du -S beim commit nicht mehr anzugeben

# commiten:
git add -A
git commit -S -m "test"

# Ein signierter, annotierter Tag:
git tag -s v1.0.0 -m "Version 1.0.0"
# Verify
git tag -v v1.0.0
# Falls du einen Tag für einen bereits vorhandenen Commit setzt, häng dessen Commit-Hash an:
git tag -s v1.0.0 <commit-hash> -m "Version 1.0.0"
```

```bash
curl -fsSLO https://raw.githubusercontent.com/Tornado3P9/release-signing/refs/heads/main/release-public-signing-key.asc
gpg --import release-public-signing-key.asc
gpg --fingerprint 5721682F09E2C6BAAA8050541E6EF85F35741223
```

```bash
gpg --verify SHA256SUMS.asc SHA256SUMS
sha256sum --check SHA256SUMS
```
