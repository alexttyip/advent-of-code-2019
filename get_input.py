#!/usr/bin/env python
import os
import subprocess
import sys

import requests


def create_dir():
    subprocess.run(["cargo", "new", day_dir])


def download_input():
    if not os.path.isdir(day_dir):
        os.mkdir(day_dir)

    url = f"https://adventofcode.com/2019/day/{day}/input"
    cookie = {
        "session": "53616c7465645f5f66aba92f012b2adf844c3ad43df6d4ce06535638e807c83fc4a61a15dd8bd72643aa2b30313bd55ced529ba68f0e1312a33696289e598764 "
    }
    resp = requests.get(url, cookies=cookie)

    open(f"{day_dir}/input.txt", "wb").write(resp.content)


if __name__ == '__main__':
    day = int(sys.argv[1])

    day_dir = f"day{day:02}"

    create_dir()
    download_input()
