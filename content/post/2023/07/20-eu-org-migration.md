---
title: "XTEXX.ML to XTEXX.EU.ORG Migration"
date: 2023-07-20T21:03:21+08:00
---

Today, ML. cTLD suddenly stopped responding NS records of most subdomains, without an announcement.

I believe that it is not a mistake because it is not fixed in time and the NS server of ML is still online and responding NS records for GOUV.ML.

I understand the challenge and difficulties that Mali are facing. However, considering that Gabon stopped the free service of GA cTLD a few months ago, I think it is unreliable to continue to use ML.

I am deeply sorry for the inconvenience and loss caused by events happened today.
After thinking several times, I decided to migrate all services from XTEXX.ML. to XTEXX.EU.ORG.

I deeply know that this will cause many problems, such as old links and mailboxes. But this is a necessary step, a step towards DNSSEC and a step towards providing a more stable service.

Thanks.

-- xtex

2023.7.20 20:11 CST

```ASN.1
-----BEGIN PGP SIGNED MESSAGE-----
Hash: SHA512

Today, ML. cTLD suddenly stopped responding NS records of most subdomains, without an announcement.
I believe that it is not a mistake because it is not fixed in time and the NS server of ML is still online and responding NS records for GOUV.ML.
I understand the challenge and difficulties that Mali are facing. However, considering that Gabon stopped the free service of GA cTLD a few months ago, I think it is unreliable to continue to use ML.
I am deeply sorry for the inconvenience and loss caused by events happened today.
After thinking several times, I decided to migrate all services from XTEXX.ML. to XTEXX.EU.ORG.
I deeply know that this will cause many problems, such as old links and mailboxes. But this is a necessary step, a step towards DNSSEC and a step towards providing a more stable service.
Thanks.
- -- xtex
2023.7.20 20:11 CST
-----BEGIN PGP SIGNATURE-----

iHUEARYKAB0WIQRyMYBLBSxnDxWmdx25GAhu2ARbkQUCZLklUgAKCRC5GAhu2ARb
kYBLAQCgHpX7/N+pBS0WOCycXudYqkuztd3+JOMSwcvxAcmhTQD+Op9dE6o3K2/L
/TfR0FqlsOlW0VCVRtElwSq04dBQEg4=
=qE6m
-----END PGP SIGNATURE-----
```

By the way, there are some side-effects:

1. SSH key comments changed

   ```
   ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIC9Xq225dYzTPQLc3ixSeRSlaq/8pSTAVI+flvSMMApe xtex@xtexx.eu.org
   sk-ssh-ed25519@openssh.com AAAAGnNrLXNzaC1lZDI1NTE5QG9wZW5zc2guY29tAAAAIBec9mnzRi149os0tSb2zA0aJf/c7/6wF6W5weK8+fHPAAAABHNzaDo= xtex.sk@xtexx.eu.org
   ```

2. GPG new email added

   ```ASN.1
   -----BEGIN PGP PUBLIC KEY BLOCK-----
   
   mDMEZJzV9RYJKwYBBAHaRw8BAQdApAD/OWwICgMjl1ScioLml/7oq+CRogvhw9rZ
   sPjFx6y0FHh0ZXggPHh0ZXhAeHRleHgubWw+iJMEExYKADsCGyMFCwkIBwICIgIG
   FQoJCAsCBBYCAwECHgcCF4AWIQRyMYBLBSxnDxWmdx25GAhu2ARbkQUCZLkwcAAK
   CRC5GAhu2ARbkQ9AAP4/g7/KJR169V3BSUheMyq9SkPc1TCvBMvv1L7BhKHpAgEA
   j9ywK5cEbtG1wGFTYqRFOCTI0E2ro/DcNDdRIXdzRQu0G3h0ZXggPHh0ZXhjaG9v
   c2VyQGR1Y2suY29tPoiTBBMWCgA7FiEEcjGASwUsZw8VpncduRgIbtgEW5EFAmSc
   1hACGyMFCwkIBwICIgIGFQoJCAsCBBYCAwECHgcCF4AACgkQuRgIbtgEW5HC0QEA
   h0av2T1GaX16xqtwS0KPLf3luYZ2jkT6edhqyDCtYTEBAIkhNY/iRSb+5h6BPtD7
   JfvhoQUFuu1amJ+D3g/nerUJtBR4dGV4IDx4dGV4QGVudnMubmV0PoiTBBMWCgA7
   FiEEcjGASwUsZw8VpncduRgIbtgEW5EFAmSc1ioCGyMFCwkIBwICIgIGFQoJCAsC
   BBYCAwECHgcCF4AACgkQuRgIbtgEW5E+pgD/VscUPJ+Pvfm4f97ZlvdMB0uzCMsv
   QTcbaA1gBRMXwFcA/3MgMCL3v6O51agAk4gfksD2JNHyKF6j3lCi0aFrOtMFtBh4
   dGV4IDx4dGV4QHh0ZXh4LmV1Lm9yZz6IlgQTFgoAPgIbIwULCQgHAgIiAgYVCgkI
   CwIEFgIDAQIeBwIXgBYhBHIxgEsFLGcPFaZ3HbkYCG7YBFuRBQJkuTBwAhkBAAoJ
   ELkYCG7YBFuRMpQBANLGIs5Q9mXfid93JbNUgke6EDRjIqPD5965imyz1XyeAP9P
   gLC5yV3pXF92v7yi3gyqmPTsu2OqKq1eaT2at4HGCrg4BGSc1fUSCisGAQQBl1UB
   BQEBB0DSc43rDq31JZUs3bXNRecWF8Vgg4mi14fdNydm5AW0TgMBCAeIeAQYFgoA
   IBYhBHIxgEsFLGcPFaZ3HbkYCG7YBFuRBQJknNX1AhsMAAoJELkYCG7YBFuRW4QB
   AMzVQSRHlXqn4HjspfxB5Sv3ConXw/VYSxv+7A1BNMx5AQCZeJzM4hnusq7qGt0O
   eIukFEuSv1GX0jjW8elKMteRAQ==
   =JS3I
   -----END PGP PUBLIC KEY BLOCK-----
   
   ```

   This key has been uploaded to keyservers.

I hate ML NIC, and Freenom.

Bye.
