---
title: "Blog重构"
date: 2023-06-11T20:03:39+08:00
---

诶嘿，新的blog来喽

## Why？

旧的blog使用的是[hexo](https://hexo.io/zh-cn/)，主题是自制的[hexo-theme-flavor](https://codeberg.org/xtex/hexo-theme-flavor)，由Vercel部署。

这套方案主要有以下问题

1. nodejs在build时吃内存&慢，非常慢
2. `node_modules`，****
3. ~~咋地我就是不喜欢node虽然我以前也经常用现在也经常用~~
4. Vercel虽然免费~~但是感觉不是很Libre（cc @wikipedia_zh_cs~~
5. 旧的主题不支持深色模式

## New!

新的blog使用[hugo](https://gohugo.io/)，Golang yes！

主题也是自己造的，[hugo-theme-arc](https://codeberg.org/xtex/hugo-theme-arc/)，从开项目到写本文经过了大约7个地球日，~~实际折合写代码的时间3h~~，

这套主题受cgit启发，支持自适应颜色，更加简易
