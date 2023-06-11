---
title: 我的自制Hexo主题
#excerpt: 一个由忘写Front-Matter导致的主题
description: 一个由忘写Front-Matter导致的主题
date: 2021-08-23
---

<!-- **直到目前，本 Blog 还在使用这个主题** -->

**本主题已被弃用，见[blog 重构]({{< relref "post/2023/06/11-new-blog" >}})**

[xtexChooser/hexo-theme-flavor on GitHub](https://github.com/xtexChooser/hexo-theme-flavor)

## 起因

今天中午补档了一个 4 月 9 日那篇关于 OpenGL3.2 软件渲染的文章，然后 GitHub Actions 和 Vercel 构建都失败了

> ~Hexo 构建失败打印完错误还会输出空白文件并正常退出，然后还上传到了腾讯云的 COS 上面~

这个主题做到一半才突然发现是因为没写 Front-Matter，但是既然做了，那就做吧

而且旧的 Hermes 主题用的是 SCSS，但是 Hexo 目前仅有的两个 SCSS 插件用的都是[node-sass](https://github.com/sass/node-sass)，这个 SCSS 编译库已经弃用了，虽然也能用，但是它包含了一个非常旧的 node-gyp，我当时还在 node-gyp 仓库的 Issue 整了好久：[node-gyp#2451](https://github.com/nodejs/node-gyp/issues/2451)，发现它根本不走全局安装，导致我在新电脑上根本本地构建不了 Blog，所以趁机丢掉 SCSS

## 经过

没啥好说的，从下午写到晚上 10 点，写完推送，然后写本文

一开始打算叫 Fluid，写到一半突然想起 Hexo 主题中心好像已经有同名的了，看了下真有就改名成了 Flavor

## 结果

就这么出来了，EJS+Stylus，移动端的体验还可以？

## 使用

见[README#use](https://github.com/xtexChooser/hexo-theme-flavor#use)

简而言之：

1. `npm install --save hexo-theme-flavor hexo-renderer-ejs hexo-renderer-stylus`
2. 改`_config.yml`的`theme`成`flavor`
3. 写`theme_config`

## 本文历史

- 2021.8.23：编写本文
