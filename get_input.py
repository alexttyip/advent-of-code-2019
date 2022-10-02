#!/usr/bin/env python
import os
import subprocess
import sys

import requests


def create_dir():
    subprocess.run(["cargo", "new", day_dir])
    subprocess.run(["rm", "-rf", f"{day_dir}/.git"])

    open(f"{day_dir}/.gitignore", "w").write("/target\n")


def download_input():
    if not os.path.isdir(day_dir):
        os.mkdir(day_dir)

    url = f"https://adventofcode.com/2019/day/{day}/input"
    cookie = {
        "session": "53616c7465645f5f697cbf87ea19d83af146a2b572b5ef2a7374c290f79f8c615f9c359b56489a566bfdf7013caa22f5c1470bbc3b3089962565219fd4e1d7cc"
    }
    resp = requests.get(url, cookies=cookie)

    open(f"{day_dir}/input.txt", "wb").write(resp.content)


if __name__ == '__main__':
    day = int(sys.argv[1])

    day_dir = f"day{day:02}"

    create_dir()
    download_input()
