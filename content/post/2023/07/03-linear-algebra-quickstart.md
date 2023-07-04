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

平面上基底（基向量）由两个向量组成，$(e_1, e_2)$，$e_1$与$e_2$不一定垂直，方向不能相同
$$
x_1e_1+\dots+x_ne_n=x^{'}_1e_1+\dots+x^{'}_ne_n \\
(x_1,\dots,x_n)^T=(x^{'}_1,\dots,x^{'}_n)^T
$$
基向量的个数=维数

## 矩阵

矩阵用于将向量映射到另一个向量，“mXn矩阵”可以把$m$维向量映射到$n$维

“nXn矩阵”为先列后行，如$\begin{bmatrix}1 2 3 \\ 1 2 3 \end{bmatrix}$是一个2X3矩阵

矩阵的$(i, j)$元素为第$i$行第$j$列

$A=(a_ij)$时表示
$$
A=\begin{bmatrix}a_{11} \ a_{12} \ \dots \ a_{1i} \\ a_{21} \ a_{22} \ \dots \ a_{2i} \\ \vdots \\ a_{j1} \ a_{j2} \ \dots \ a_{ji} \end{bmatrix}
$$
行与列数量相同的矩阵称为正方矩阵（aka. 方阵）

### 基本算术

#### 加法

$$
\begin{bmatrix}a_{11} \ \dots \ a_{1n} \\ \vdots \\ a_{m1} \ \dots \ a_{mn}\end{bmatrix} + \begin{bmatrix}b_{11} \ \dots \ b_{1n} \\ \vdots \\ b_{m1} \ \dots \ b_{mn}\end{bmatrix} = \begin{bmatrix}a_{11}+b_{11} \ \dots \ a_{1n} + b_{1n} \\ \vdots \\ a{m1} + b_{m1} \ \dots \ a_{mn} + b_{mn}\end{bmatrix}
$$

#### 标量乘法

$$
c\begin{bmatrix}a_{11} \ \dots \ a_{1n} \\ \vdots \\ a_{m1} \ \dots \ a_{mn}\end{bmatrix} = \begin{bmatrix}ca_{11} \ \dots \ ca_{1n} \\ \vdots \\ ca_{m1} \ \dots \ ca_{mn}\end{bmatrix}
$$

#### 乘积

$$
\begin{bmatrix}a_{11} \ \dots \ a_{1n} \\ \vdots \\ a_{m1} \ \dots \ a_{mn}\end{bmatrix} \begin{bmatrix}x_1 \\ \vdots \\ x_n\end{bmatrix} = \begin{bmatrix}a_{11}x_1 + \dots + a_{1n}x_n \\ \vdots \\ a_{m1}x_1 + \dots + a_{mn}x_n\end{bmatrix}
$$

