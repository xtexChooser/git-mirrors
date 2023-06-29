---
title: "New SSH & GPG keys enabled"
date: 2023-06-29T08:32:24+08:00
---

Hey, now 2023-06-29 08:32:24 CST.

## SSH

I just enabled a new `sk-ssh-ed25519@openssh.com` SSH key.

The public key is ``sk-ssh-ed25519@openssh.com AAAAGnNrLXNzaC1lZDI1NTE5QG9wZW5zc2guY29tAAAAIBec9mnzRi149os0tSb2zA0aJf/c7/6wF6W5weK8+fHPAAAABHNzaDo= xtex.sk@xtexx.ml``.

Currently, there are two SSH-keys in use.

- `sk-ssh-ed25519@openssh.com AAAAGnNrLXNzaC1lZDI1NTE5QG9wZW5zc2guY29tAAAAIBec9mnzRi149os0tSb2zA0aJf/c7/6wF6W5weK8+fHPAAAABHNzaDo= xtex.sk@xtexx.ml` (the newer one, fingerprint is `256 SHA256:5mjvhkvAqcJ0brNDPTTO2cBEnl285gxDHrHab71Kkzo xtex.sk@xtexx.ml (ED25519-SK)`)
- `ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIC9Xq225dYzTPQLc3ixSeRSlaq/8pSTAVI+flvSMMApe xtex@xtexx.ml` (the older one, fingerprint is `256 SHA256:IEYEjkZlkUTr5U9GiDAmZU/4eZus2t2RsxusyhQqwao xtex@xtexx.ml (ED25519)`)

Happy to play with YubiKey! (I also enabled `pam_u2f` with ybk

The full update message with GPG signature is available at [here](https://pb.envs.net/?2757e4fdd6c6e02d#ZtqysZUV31wUEfHKGSbWupFbtU8nRiZmoBRNPeZZmyy).

## GPG

I know that I should use curve keys instead of RSA-4096 yesterday (thanks to @littlec), so I changed to ed25519 today!

The fingerprint is: `7231804B052C670F15A6771DB918086ED8045B91`

The full public key is:

```ASN.1
-----BEGIN PGP PUBLIC KEY BLOCK-----

mDMEZJzV9RYJKwYBBAHaRw8BAQdApAD/OWwICgMjl1ScioLml/7oq+CRogvhw9rZ
sPjFx6y0FHh0ZXggPHh0ZXhAeHRleHgubWw+iJYEExYKAD4CGyMFCwkIBwICIgIG
FQoJCAsCBBYCAwECHgcCF4AWIQRyMYBLBSxnDxWmdx25GAhu2ARbkQUCZJzWOgIZ
AQAKCRC5GAhu2ARbkS+pAQDztscLAJbuGr0ygje8y5btdYDqfhIUVdZYlgWZFYuk
MwD+Jr1cQsFPnbFbomgLLunK84lJE0zuWrDorxaKNQ7Jowu0G3h0ZXggPHh0ZXhj
aG9vc2VyQGR1Y2suY29tPoiTBBMWCgA7FiEEcjGASwUsZw8VpncduRgIbtgEW5EF
AmSc1hACGyMFCwkIBwICIgIGFQoJCAsCBBYCAwECHgcCF4AACgkQuRgIbtgEW5HC
0QEAh0av2T1GaX16xqtwS0KPLf3luYZ2jkT6edhqyDCtYTEBAIkhNY/iRSb+5h6B
PtD7JfvhoQUFuu1amJ+D3g/nerUJtBR4dGV4IDx4dGV4QGVudnMubmV0PoiTBBMW
CgA7FiEEcjGASwUsZw8VpncduRgIbtgEW5EFAmSc1ioCGyMFCwkIBwICIgIGFQoJ
CAsCBBYCAwECHgcCF4AACgkQuRgIbtgEW5E+pgD/VscUPJ+Pvfm4f97ZlvdMB0uz
CMsvQTcbaA1gBRMXwFcA/3MgMCL3v6O51agAk4gfksD2JNHyKF6j3lCi0aFrOtMF
uDgEZJzV9RIKKwYBBAGXVQEFAQEHQNJzjesOrfUllSzdtc1F5xYXxWCDiaLXh903
J2bkBbROAwEIB4h4BBgWCgAgFiEEcjGASwUsZw8VpncduRgIbtgEW5EFAmSc1fUC
GwwACgkQuRgIbtgEW5FbhAEAzNVBJEeVeqfgeOyl/EHlK/cKidfD9VhLG/7sDUE0
zHkBAJl4nMziGe6yruoa3Q54i6QUS5K/UZfSONbx6Uoy15EB
=nZKP
-----END PGP PUBLIC KEY BLOCK-----

```

This key have been submitted to `keys.openpgp.org`, `keyserver.ubuntu.com` and `pgp.mit.edu`.

The old key (`43D55EC7285804D1E9FEAAFB978F2E760D9DB0EB`) is revoked:

```ASN.1
-----BEGIN PGP PUBLIC KEY BLOCK-----
Comment: This is a revocation certificate

iQJtBCABCABXFiEEQ9VexyhYBNHp/qr7l48udg2dsOsFAmSc3D05HQNUaGUgbmV3
IGtleSBpcyA3MjMxODA0QjA1MkM2NzBGMTVBNjc3MURCOTE4MDg2RUQ4MDQ1Qjkx
AAoJEJePLnYNnbDrLu4QALXZSYxobst1QafbDALJR1t4coAUexk8c/we1mn9FbpA
htyP+obYq4d6VarUthnfSo5VtG4pRtJfNA6aC9jia9H0IItyxYd1tZnsCjNLeFlw
Dirlnq5CmXxACwiQuAMryH1wC/aLeKMSnmVOtE9cgDSdMdzA47h7UX4RnUFjcj1R
kfowJGt52rl3LQvIqQ57G69vCBERpqEMKSrU3yX6vHkUPHACAXWAWaDJBu0pHvVH
xzGh1hxaFGt6EFlM9YPwmViV7Z3mB9TXXphLmYiArb8AO8AmPVEVWaPNM8+NmEyE
cqeWsm1DxqYBexolJAIoItGYr90ICXVLcx7ODOCGdY4IfDmdIxd+FjEWpWJNlnDR
64eSAwSX87EahZ/vRF9fIKrvt+SchqutFpGA/X7C5OSSD1XVGnpVHCqfK0n5/XKm
tDTVQeFuFjGRiGuuK4pX5MkZ1Tq2imXa6GKpwAdAxuUzB+yIlvdO2q1TJ5rlcbS0
CHsxmC90nKmCfBVNgrWFdQOn1r2YY6aF4Af9mui3D7daCJ8xmeGsDJEoRLBD0WY4
W8hdIOqMNPU1G2kYiFv60ocam3vZTIAjZ1LzFrAMflgGFwyYf4IvKSqVNJJIV68b
1HdgckpRTDcpmUPTpF4tc2Q9PdXktfJwiiveB32SaOzALjJdAAuZG87ut1fmbrrA
=Ka9J
-----END PGP PUBLIC KEY BLOCK-----

```

The revocation key is uploaded to keyservers, too.

## Credits

~~Thanks to everyone who helped me. Thanks to ltlec. Thanks to Cloudflare's cheap Yubikey. Thanks to Yubico. Thanks to OpenGPG. Thanks to OpenSSH.~~ Thanks to the world.
