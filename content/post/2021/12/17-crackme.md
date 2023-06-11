---
title: 玩CrackMe
#excerpt: 2021-10-15
date: 2021-10-15
---

[文件链接](https://crackmes.one/crackme/6045838033c5d42c3d016d5e)

### 简单使用

这是一个由用户名和 License-Key 组成的校验程序，其中，LicKey 需要至少 17 位且其中包含`KEY`，校验失败后会重试，直接扔进 Ghidra

### 定位

Ghidra 直接自动识别出了`_main`函数，点进去就是了，然后反编译的代码中往下翻可以找到`Username:`和`License-Key`字样，简单的命名之后是这样的

[<img src="https://s4.ax1x.com/2021/12/17/TAFyA1.png" alt="TAFyA1.png" style="zoom:67%;" />](https://imgtu.com/i/TAFyA1)

然后往下翻，忽略没用的错误提示，可以找到这两段代码

[<img src="https://s4.ax1x.com/2021/12/17/TAksKg.png" alt="TAksKg.png" style="zoom:67%;" />](https://imgtu.com/i/TAksKg)

[<img src="https://s4.ax1x.com/2021/12/17/TAkAvF.png" alt="TAkAvF.png" style="zoom:80%;" />](https://imgtu.com/i/TAkAvF)

前面检查了用户名，后面检查了 LicKey，两者 demangle 之后都是`___gnu_cxx::__enable_if<std::__is_char<char>::__value, bool>::__type std::operator==<char>(std::basic_string<char, std::char_traits<char>, std::allocator<char> > const&, std::basic_string<char, std::char_traits<char>, std::allocator<char> > const&)`（直接文本比较），于是可以直接知道`local_a0`和`local_104`就是对应着正确的用户名和密码。

### 解密

返回到`_main`前面，可以看到一大堆类似于这样的代码

[<img src="https://s4.ax1x.com/2021/12/17/TAEK1K.png" alt="TAEK1K.png" style="zoom: 67%;" />](https://imgtu.com/i/TAEK1K)

全部 demangle 之后很明显能看到拼接了字符串

[<img src="https://s4.ax1x.com/2021/12/17/TAVhGt.png" alt="TAVhGt.png" style="zoom:67%;" />](https://imgtu.com/i/TAVhGt)

测试后发现，`crck_me`就是 UserName，同理，`$2833-15431-51KEY`就是 LicKey

## 本文历史

- 2021.12.17：编写本文
