---
title: Burble DN42 Ping Challenge
#excerpt: Burble Ping Challenge的解题思路（
date: 2022-08-21
---

[Ping Challenge | burble.dn42 ](https://dn42.burble.com/services/ping/)的解题思路

**剧透警告**，尽量做的时候别看

耗时三天，暑假，回校前一天上午完成（2022/8/21 10:49）。IYKYK（懂得都懂），看不懂也没关系就不是写来让你看的（

用到了[CyberChef](https://cyberchef.org/)，NodeJS，最后一部分需要控制 ICMP Ping data 的用 Windows 自带的`ping`做不了，但是可以用我写的几个小工具发，有关 API 在[Icmpapi.h header](https://docs.microsoft.com/en-us/windows/win32/api/icmpapi/)，busybox 可以直接发。

第一次玩这种 Cyber 的玩意，有点难度，好好玩，学到的许多，下次继续（听说 DN42 里头还有一个）

中途还对我的内部网络大改，这个挑战的信息是绑定到源 IP 的，所幸使用的设备 IP 没变。

感谢 Chimon、Lan Tian 提供的帮助。

有关源码已经打包上传：[Burble Ping Challenge.tar](https://anonfiles.com/P7v9F54dyc/Burble_Ping_Challenge_tar)（[.sig](https://anonfiles.com/Q6v9F747y5/Burble_Ping_Challenge.tar_sig)），以 CC0 发布，[Internet Archive](https://archive.org/details/burble-ping-challenge)。

最后几个 Ping payload 有关的，错误代码可以看[Microsoft Docs](https://docs.microsoft.com/en-us/windows/win32/debug/system-error-codes--9000-11999-)，0 是超时，11010 是被 QOS 限制了重试几次就好。

`fd42:4242:2601:31f0::1`

## Challenge 1

```
Welcome to the burble.dn42 ping challenge. Ping fd42:4242:2601:31f0:1a2:a1a0:5974:166d to continue.
```

`fd42:4242:2601:31f0:1a2:a1a0:5974:166d`

## Challenge 2

```
Congratulations, you have completed level 1. Challenges are dynamically generated, and the server retains state between each level. Each IP you see here is specific to you and your source IP address. Ping fd42:4242:2601:31f0:2f8:5679:c098:8f36 to continue.
```

`fd42:4242:2601:31f0:2f8:5679:c098:8f36`

## Challenge 3

```
You can ping fd42:4242:2601:31f0::2 to check your current challenge level. Ping '467d:2fa3:b32f:7d3:0f13:1062:2424:24df' to continue.
```

`467d:2fa3:b32f:7d3:0f13:1062:2424:24df`

` fd42:4242:2601:31f0:3d7:f23b:3af2:d764`

## Challenge 4

```
You can also ping fd42:4242:2601:31f0::3 to check the current challenge again. Ping 'Foxtrot Delta Four Two : Four Two Four Two : Two Six Zero One : Three One Foxtrot Zero : Four Delta One : Delta Foxtrot Seven Five : Five Delta Six Three : Six One Charlie Six ' to continue.
```

`fd42:4242:2601:31f0:4d1:df75:5d63:61c6`

## Challenge 5

```
A CyberChef may come in handy when completing the challenges. Ping '..-. -.. ....- ..--- : ....- ..--- ....- ..--- : ..--- -.... ----- .---- : ...-- .---- ..-. ----- : ..... -.. ..-. : -.. -.-. -... -.-. : ...-- ..--- ----. ..-. : ...-- -.... ----. ..--- ' to continue.
```

1. `..-. -.. ....- ..---` = `fd42`
2. `....- ..--- ....- ..---` = `4242`
3. `..--- -.... ----- .----` = `2601`
4. `...-- .---- ..-. -----` = `31f0`
5. `..... -.. ..-.` = `5df`
6. `-.. -.-. -... -.-.` = `dcbc`
7. `...-- ..--- ----. ..-.` = `329f`
8. `...-- -.... ----. ..--- ` = `3692`

`fd42:4242:2601:31f0:5df:dcbc:329f:3692`

## Challenge 6

```
Ping 'fd424242260131f006e1b47ca52f1f6d' to continue.
```

`fd424242260131f006e1b47ca52f1f6d`

`fd42:4242:2601:31f0:06e1:b47c:a52f:1f6d`

## Challenge 7

```
Ping '11111101010000100100001001000010001001100000000100110001111100000000011110101001100011010000110101100101000011000111111001000011' to continue.
```

`11111101010000100100001001000010001001100000000100110001111100000000011110101001100011010000110101100101000011000111111001000011`

```javascript
parseInt(
  "11111101010000100100001001000010001001100000000100110001111100000000011110101001100011010000110101100101000011000111111001000011",
  2
).toString(16);
```

`FD424242260131F007A98D0D650C7E43`

`fd42:4242:2601:31f0:07a9:8d0d:650c:7e43`

## Challenge 8

```
Swap everything. Ping 'c2c29508a681d3f6424242fdf0310126' to continue.
```

`c2c29508a681d3f6424242fdf0310126`

长度相同 不是映射（c->f/4） `fd` `42`

1. `c2c29508a681d3f6424242fd f0310126` -> `f0310126 c2c29508a681d3f6424242fd`

2. `f0 31 01 26 c2 c2 95 08 a6 81 d3 f6 42 42 42 fd` -> `fd424242f6d381a60895c2c2260131f0`
3. `fd42 4242 f6d381a60895c2c2 260131f0` -> `fd42 4242 2601 31f0 f6d381a60895c2c2`
4. `fd424242260131f0 f6d381a6 0895 c2c2` -> `fd424242260131f0 0895 c2c2 f6d381a6 `

## Challenge 9

```
UGluZyBmZDQyOjQyNDI6MjYwMTozMWYwOjkwMDoyMmUwOmYxZjA6ZjVlYyB0byBjb250aW51ZS4=
```

`=` Base64

`Ping fd42:4242:2601:31f0:900:22e0:f1f0:f5ec to continue.`

`fd42:4242:2601:31f0:900:22e0:f1f0:f5ec`

## Challenge 10 = Checkpoint

## Challenge 11

```
Well done, you have reached the first checkpoint. Checkpoints are fixed IPs that will reset your progress at any time. Note that checkpoints are specific to your source IP address. Ping fd42:4242:2601:31f0:be6:4d7:42a0:9c9f to continue.
```

Ping ::2 got 11

## Challenge 12

```
The next set of challenges are all related to encryption. This IP address here is simply encrypted, but requires no key ?! MzD0Zwb0ZwDlBwV2ZQR6ZmSzZQcwMQD6LGDlMGcvL2V4BzMxZN==
```

`MzD0Zwb0ZwDlBwV2ZQR6ZmSzZQcwMQD6LGDlMGcvL2V4BzMxZN==`

```javascript
var enc = "MzD0Zwb0ZwDlBwV2ZQR6ZmSzZQcwMQD6LGDlMGcvL2V4BzMxZN=="
   decodeBase64(decodeROT13(enc))
 = decodeBase64(enc, "N-ZA-Mn-za-m0-9+/=")
 = "fd42:4242:2601:31f0:cd4:a42e:bcb8:fd0"
```

`fd42:4242:2601:31f0:cd4:a42e:bcb8:fd0`

## Challenge 13

```
Either A or B, but not both: 539519e3c849093c9235c0f8b4e8c621 aed75ba1ee4838cc9f9649c991799820
```

`Either A or B, but not both` `539519e3c849093c9235c0f8b4e8c621` `aed75ba1ee4838cc9f9649c991799820`

`XOR(0x539519e3c849093c9235c0f8b4e8c621, 0xaed75ba1ee4838cc9f9649c991799820) = fd 42 42 42 26 01 31 f0 0d a3 89 31 25 91 5e 01`

```javascript
(0xaed75ba1ee4838cc9f9649c991799820n ^ 0x539519e3c849093c9235c0f8b4e8c621n).toString(16) = 'fd424242260131f00da3893125915e01'
```

`fd42:4242:2601:31f0:0da3:8931:2591:5e01`

## Challenge 14

```
The challenges so far have been too simple, a more Advanced Encryption Standard is required: 0349f977bc7b3dd620eab4d41390a13b 47b3c7bb0b388c33d10088365379ce35 11ec1b0d5e9f20785f56636af70fbb54
```

`Advanced Encryption Standard` = AES

Key = `0349f977bc7b3dd620eab4d41390a13b`

IV = `47b3c7bb0b388c33d10088365379ce35`

Encrypted = `11ec1b0d5e9f20785f56636af70fbb54`

Mode = CFB

Decrypted = `fd424242260131f00ee433c479b85e98`

`fd42:4242:2601:31f0:0ee4:33c4:79b8:5e98`

## Challenge 15

```
This time the key is  in plain sight.. 7165d0e0c4c105c4de8e7ea85a6ef4fb e7da0d7a50946d590f8ebd5bde61c4b7
```

~~`\sin plain sight.` (" in plain sight.", 16, 128 bits, `20696e20706c61696e2073696768742e`)~~

`in plain sight..`(16, 128 bits, `696e20706c61696e2073696768742e2e`, thanks to @Chimon)

`7165d0e0c4c105c4de8e7ea85a6ef4fb` (16 bytes, 128 bits)

`e7da0d7a50946d590f8ebd5bde61c4b7` (16 bytes, 128 bits)

- Blowfish/DES: 64 bits key
- Triple DES: 192 bits key
- Vigenère Decode/Bifid Cipher Decode: only letter key
- **AES-128/RC2/RC4**
  - AES-128
    - 16 bytes key
    - 16 bytes IV

Key = `696e20706c61696e2073696768742e2e`

IV = `7165d0e0c4c105c4de8e7ea85a6ef4fb`

Encrypted = `e7da0d7a50946d590f8ebd5bde61c4b7`

Mode = OFB

Decrypted = `fd424242260131f00f9505d3437332c9`

`fd42:4242:2601:31f0:0f95:05d3:4373:32c9`

## Challenge 16

```
Cryptoanalysis required: 327777775b36662545055d7fa47ad1da
```

`327777775b36662545055d7fa47ad1da`

`32 77 77 77 5b 36 66 25 45 05 5d 7f a4 7a d1 da`

32 chars, 128 bits

reversable, not hashing, same size = mapping

~~`fd42:4242:2601:31f0` to `AxBC:BCBC:CxDE:xEAD` to `xxABABAB`~~

~~`424242` to `n(n-2)n(n-2)n(n-2)`~~

`fd424242` = `xxAAAAAA` = `32777777`

| Src | Dst | Src | Dst |
| --- | --- | --- | --- |
| fd  | 32  | 01  | 36  |
| 42  | 77  | 31  | 66  |
| 26  | 5b  | f0  | 25  |

~~fd - 32 = cb~~(byte overflow) fd + 35 = 32

36 - 01 = 35

66 - 31 = 35

`dst = src + 35`

`src = dst - 35`

`45 05 5d 7f a4 7a d1 da` = `10d0:284a:6f45:9ca5`

`fd42:4242:2601:31f0:10d0:284a:6f45:9ca5`

## Challenge 19

```
I forgot the last two bytes of the IP ! I did manage to checksum the address first though: fd42:4242:2601:31f0:1300:d2a9:7f78:0 / SHA256:8bd1cd77f57fd1a55e48575a258477fc32d752d88ed9a146ca4213737d18acd9
```

```javascript
for (var n = 0; n < 65536; n++) {
  const hash = crypto.createHash("sha256");
  hash.update(
    "fd42:4242:2601:31f0:1300:d2a9:7f78:" + n.toString(16).padStart(4, "0")
  );
  if (
    hash.digest("hex").toLowerCase() ==
    "8bd1cd77f57fd1a55e48575a258477fc32d752d88ed9a146ca4213737d18acd9"
  )
    console.log("Found " + n.toString(16));
}
```

```
Found 1803
```

`fd42:4242:2601:31f0:1300:d2a9:7f78:1803`

## Challenge 20 = Checkpoint

```
Well done, you have reached the next checkpoint. You can use this address to restart from this level at any time. Ping fd42:4242:2601:31f0:14a2:133:bb0a:213b to continue.
```

## Challenge 21

```
The next level requires a payload size appropriate for this decentralised network: fd42:4242:2601:31f0:159f:e67e:2ff:e2b6
```

Size 0: `Incorrect packet: 0`

Wireshark filter: `icmpv6 && icmpv6.type == 129`

Fuzzed, found correct packet size 42.

`this decentralised network` = `DN42` = `42`

## Challenge 22

```
IP packet size, int((rev(IP))*100): fd42:4242:2601:31f0:16eb:f338:669d:810d
```

`int((rev(IP))*100)`

`fd42:4242:2601:31f0:16eb:f338:669d:810d`

~~`int()`: size not matching~~

- ~~from upper: too big for ICMP ping~~
- ~~from lower~~

~~`IP`:~~

- ~~`fd42:4242:2601:31f0:16eb:f338:669d:810d`~~
- ~~Internet Protocol~~

~~`int((rev(IP))*100)` = `int( ( rev(IP) ) * 100 )` = 600~~

~~IP header = 40 bytes~~

~~ICMP header = 8 bytes~~

~~ICMP ping data size = `int((rev(IP))*100)` - 48~~

~~MTU ~= 1500~~

~~Max IPv6 Frame Size = 1368 bytes~~

Fuzzed, got correct ICMP data size 266

`rev(IP)` = `reverse(IP)` = `PI` = `3.14`

## Challenge 24

```
Make it just in time: fd42:4242:2601:31f0:18e0:82a8:4195:e5a4
```

0 -> `Incorrect packet: 122`

1 -> `Incorrect packet: 122`

122 -> `Incorrect packet: 122.....................................................................................................`

123 -> `Incorrect packet: 122......................................................................................................`

124 -> `Incorrect packet: 122.......................................................................................................`

`122` = `0x7a` = `'z'`

Fuzzed, all incorrect.

Not related to data size

Found after a tracert, its about TTL.

Make the TTL of the packet which arrived at the remote server is 0.

`tracert`(`traceroute`) finish it quickly.

## Challenge 25

```
Arrive early, with plenty of time to spare: fd42:4242:2601:31f0:19c9:f419:d01e:2a89
```

`fd42:4242:2601:31f0:19c9:f419:d01e:2a89`

## Challenge 26

```
Single byte payload, you know the number by now: fd42:4242:2601:31f0:1ae7:1959:c3e1:4dcb
```

`fd42:4242:2601:31f0:1ae7:1959:c3e1:4dcb`

`Single byte payload`

Ping with one byte: `Incorrect packet: 1/97`

`97` = `'a'` = default payload on Windows

Fuzzed, value = 42

## Challenge 27

```
The payload should be the first rule of dn42: fd42:4242:2601:31f0:1b94:654f:fc67:a476
```

`fd42:4242:2601:31f0:1b94:654f:fc67:a476`

Search in Bing, got [howto/networksettings (dn42.dev)](https://www.dn42.dev/howto/networksettings).

> The first rule of dn42: Always disable `rp_filter`.

`Always disable rp_filter.`

After tries, found correct payload `rp_filter`.

## Challenge 28

```
Use the source Luke: fd10:78d4:da31:0:5cce:353e:83bb:ec4c -> fd42:4242:2601:31f0:1cb0:8168:d0fa:677f
```

`fd10:78d4:da31:0:5cce:353e:83bb:ec4c`

`fd42:4242:2601:31f0:1cb0:8168:d0fa:677f`

Use the given address to ping.

## Challenge 29

```
Now return back to your original src IP. Ping fd42:4242:2601:31f0:1d00:cfe5:fb89:ebb9 to continue
```

`fd42:4242:2601:31f0:1d00:cfe5:fb89:ebb9`

```
Congratulations ! You have completed the ping challenge. I hope you had fun along the way and, of course, please let me know any issues or feedback. burble.
```

## 本页历史

- 2028.8.21：First Finish!
