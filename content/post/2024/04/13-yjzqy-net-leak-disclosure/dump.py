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
