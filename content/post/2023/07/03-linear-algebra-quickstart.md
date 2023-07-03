---
title: "线性代数"
date: 2023-07-03T20:29:05+08:00
---

> 列向量渲染效果不好可以看[codeberg](https://codeberg.org/xtex/blog/src/branch/main/content/post/2023/07/03-linear-algebra-quickstart.md)

## 前言

## 向量

- 行向量：$\begin{bmatrix}1 \ 2 \ 3 \ \dots \ n \end{bmatrix}$
  
  在Octave中写作`[1 2 3]`或`[1, 2, 3]`
  
- 列向量：$\begin{bmatrix}1 \\ 2 \\ 3 \\ \vdots \\ n \end{bmatrix}$

  在Octave中写作`[1; 2; 3]`

### 零向量

$$
o = \begin{bmatrix}0 \ \dots \ 0\end{bmatrix}^T
$$

### 基本算术

#### 转置

$$
transpose(\begin{bmatrix} x_1 \ x_2 \ x_3 \ \dots \ x_n \end{bmatrix}) = \begin{bmatrix} x_1 \ x_2 \ x_3 \ \dots \ x_n \end{bmatrix}^T = \begin{bmatrix} x_n \ \dots \ x_3 \ x_2 \ x_1 \end{bmatrix}
$$

#### 加法

$$
\begin{bmatrix} x_1 \ x_2 \ x_3 \ \dots \ x_n \end{bmatrix} + \begin{bmatrix} y_1 \ y_2 \ y_3 \ \dots \ y_n \end{bmatrix} = \begin{bmatrix} (x_1+y_1) \ (x_2+y_2) \ (x_3+y_3) \ \dots \ x_n+y_n \end{bmatrix}
$$

$$
\begin{bmatrix} x_1 \\ x_2 \\ x_3 \\ \dots \\ x_n \end{bmatrix} + \begin{bmatrix} y_1 \\ y_2 \\ y_3 \\ \dots \\ y_n \end{bmatrix} = \begin{bmatrix} x_1+y_1 \\ x_2+y_2 \\ x_3+y_3 \\ \dots \\ x_n+y_n \end{bmatrix} \\
$$

#### 标量乘法

aka. 纯量乘法
$$
c\begin{bmatrix} x_1 \ x_2 \ x_3 \ \dots \ x_n \end{bmatrix}=\begin{bmatrix} cx_1 \ cx_2 \ cx_3 \ \dots \ cx_n \end{bmatrix}
$$

### 线性空间

aka. 向量空间

向量没有长度、角度，不能比较大小或旋转

> 内积 $x \cdot y = x_1y_1 + x_2y_2$
>
> 点乘（点积）是内积的特殊情况
>
> 三维外积 $x \cdot y = ((x_2y_3 - x_3y_2), (x_3y_1 - x_1y_3), (x_1y_2 - x_2y_1))^T$
>
> 线性空间去掉原点=仿射空间

### 基底

基底（基向量）由两个向量组成，$(e_1, e_2)$，$e_1$与$e_2$不一定垂直，
