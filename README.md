# checkmypassword

for experimenting with github actions, not intended for professional use

```
git tag v0.1.0
git push origin v0.1.0
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

