#!/usr/bin/env python

from urllib import request
from bs4 import BeautifulSoup
import re

errorcodes = request.urlopen("http://switchbrew.org/index.php?title=Error_codes").read()
error_codes_tree = BeautifulSoup(errorcodes, 'html.parser')
table = error_codes_tree.find_all("table")[1]

print("enum Module {")

def rustify(s):
    return re.sub(r'\(.*\)', '', s.strip().replace(' ', ''))

for child in table.find_all("tr"):
    if len(child.find_all("th")) > 0:
        continue
    i = child.find_all("td")[0]
    name = child.find_all("td")[1]
    i, name = ("".join(i.contents).strip(), "".join(name.strings).strip())

    if name == "":
        continue

    name = rustify(name)
    print(f'\t{name} = {i},')

print("}")
