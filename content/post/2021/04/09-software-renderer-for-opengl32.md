---
title: 在不支持OpenGL3.2的设备上用软件渲染玩MC1.17
#excerpt: 不支持OpenGL3.2又不是不能玩了嗷
date: 2021-04-09
---

**注意：**
因为使用了软件渲染，对 CPU 的负担会加重，所以游戏可能会变卡

过老的版本（远古版）在此方法设置后将无法启动
测试：

- rd-132211（目前可下载最老版本）：不可启动
- 4k（2009/12/1，old_alpha）：可启动但黑屏
- b1.4（2011/3/30 发布，old_beta）：可启动
- 1.12：可启动
- 21w13a：可启动

## 起因

在 Minecraft 的 1.17 快照开始，Minecraft 要求使用 OpenGL3.2 以上的版本 OpenGL2.0 且有正版还玩了 n 年的我笑了（虽然这是 n 年前的版本但是我~~也没钱换电脑~~现在已经换了）经过一个周末的研究后得到了这个方法

## 是否需要

若果你坚持只玩 1.17 以下的版本，很高兴，你可以离开了（除非你电脑 1.17 以下的版本都带不动，不过 GL2.0 都没有的电脑，加上这个也玩不了多流畅吧

如果能直接启动 1.17 以上的版本，不需要，如果不能，出现类似

> GLFW error 65543: WGL: OpenGL profile requested but WGL* ARB* create\_ context. profile is unavailable
> Please make sure you have up-to-date drivers (seeaka.ms/mcdriver for instructions).

的提示，~~很高兴~~，你需要本文

## 下载

### Windows

Mesa3D：[https://github.com/pal1000/mesa-dist-win/releases](https://github.com/pal1000/mesa-dist-win/releases)

打开之后下载[mesa3d-21.0.2-release-mingw.7z](https://github.com/pal1000/mesa-dist-win/releases/download/21.0.2/mesa3d-21.0.2-release-mingw.7z)（展开最新构建的 Assets 就能找到了）

!!! note

    大陆地区需要**魔法**

!!! note ""

    CPU若不支持SSE3指令集需要使用MSVC构建并配置额外的环境变量，64位需要AVX指令集支持（AVX指令集要求是不应该的，pal1000已向上游汇报此问题，在[mesa#4437](https://gitlab.freedesktop.org/mesa/mesa/-/issues/4437), [mesa#3860](https://gitlab.freedesktop.org/mesa/mesa/-/issues/3860), [MINGW-packages#7530](https://github.com/msys2/MINGW-packages/issues/7530)

#### 没有魔法？

用[https://d.serctl.com/](https://d.serctl.cn/)
打开后输入`https://github.com/pal1000/mesa-dist-win/releases/download/21.1.6/mesa3d-21.1.6-release-mingw.7z`并提交（这是 21.1.6 版 Mesa3D）（未测试，于 2021/7/31 发布，2021/8/5 更新本文），若无法使用可尝试发布时测试过的版本 21.0.2，对应 URL 为`https://github.com/pal1000/mesa-dist-win/releases/download/21.0.2/mesa3d-21.0.2-release-mingw.7z`

#### 安装

解压后进入 x64/x86 文件夹取出`opengl32.dll`和`libglapi.dll`并复制到 Java 的执行目录下（即 bin 目录，和 java.exe/javaw.exe 同一个目录）

##### 安装 Vulkan

然后安装 Vulkan：[下载](https://vulkan.lunarg.com/sdk/home)

点 Windows 的`Latest Runtime/Zip`然后点`Download Latest Runtime`（[一键直达最新版下载](https://sdk.lunarg.com/sdk/download/latest/windows/vulkan-runtime.exe)）
下载并运行 vulkan 安装器
然后按`Agree`按钮安装

### Linux

大多数发行版内置 Mesa3D，自行通过软件包管理器安装（还有 Vulkan）

### Mac

我不会，参见[官网](https://docs.mesa3d.org/download.html)自行编译（还有 Vulkan）

## 配置

修改三个环境变量（=前面是变量名，后面是变量值）：

```properties
LIBGL_ALWAYS_SOFTWARE=true
MESA_GL_VERSION_OVERRIDE=4.6COMPAT
MESA_GLSL_VERSION_OVERRIDE=460
```

**修改环境变量的方法自己百度：[比如这个](https://jingyan.baidu.com/article/00a07f3876cd0582d128dc55.html)，[或者这个](http://www.xitongcheng.com/jiaocheng/win10_article_30595.html)，没有的可以自己创建**

## 感谢

- Mesa3d：[https://mesa3d.org/](https://mesa3d.org/)
- pal1000 提供的 Mesa3d 在 Windows 上的预构建版本：[https://github.com/pal1000/mesa-dist-win](https://github.com/pal1000/mesa-dist-win)

- 不知名的 serctl 下载加速服务：[https://d.serctl.com/](https://d.serctl.com/)
- LunarG 的 Vulkan 运行时：[https://vulkan.lunarg.com/sdk/home](https://vulkan.lunarg.com/sdk/home)

#### 关于 Mesa3D

> 官方描述：Open source implementations of [OpenGL](https://www.opengl.org/), [OpenGL ES](https://www.khronos.org/opengles/), [Vulkan](https://www.khronos.org/vulkan/), [OpenCL](https://www.khronos.org/opencl/), and more!
> （翻译：开源的[OpenGL](https://www.opengl.org/)、[OpenGL ES](https://www.khronos.org/opengles/)、[Vulkan](https://www.khronos.org/vulkan/)、[OpenCL](https://www.khronos.org/opencl/)及更多的实现）

MESA3D 的开源协议：

> Copyright (C) 1999-2007 Brian Paul All Rights Reserved.
>
> Permission is hereby granted, free of charge, to any person obtaining a
> copy of this software and associated documentation files (the "Software"),
> to deal in the Software without restriction, including without limitation
> the rights to use, copy, modify, merge, publish, distribute, sublicense,
> and/or sell copies of the Software, and to permit persons to whom the
> Software is furnished to do so, subject to the following conditions:
>
> The above copyright notice and this permission notice shall be included
> in all copies or substantial portions of the Software.
>
> THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
> OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
> FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
> THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
> LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
> OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
> SOFTWARE.

## 本文历史

- 2021.4.9：在 MCBBS 发布（链接:[在不支持 OpenGL3.2 的设备上用软件渲染玩 1.17](https://www.mcbbs.net/thread-1189935-1-1.html)）

- 2021.8.23：转发到 Blog

- 2021.8.23：添加简介

### 你知道吗

- 本文在 MCBBS 上发布的时间比本 Blog 建立还要早
- 在本文在 MCBBS 发布两个月后我就换新电脑了
