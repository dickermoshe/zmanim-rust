import json

with open("a.json", "r") as file:
    data = json.load(file)

zmanim_methods = set()
zmanim_calc_methods = set()
other_methods = set()

for i in data["types"][0]["bodyDeclarations"]:
    name = i.get("name", {}).get("identifier", "")
    if name.startswith("set"):
        continue
    if name.startswith("get"):
        name = name[3:]
    if i["node"] != "MethodDeclaration":
        continue

    is_public = False
    is_deprecated = False

    for modifier in i.get("modifiers", []):
        if (
            modifier.get("node", "") == "Modifier"
            and modifier.get("keyword", "") == "public"
        ):
            is_public = True
        if (
            modifier.get("node", "") == "MarkerAnnotation"
            and modifier.get("typeName", {}).get("identifier", "") == "Deprecated"
        ):
            is_deprecated = True
            break
    if not is_public or is_deprecated:
        continue
    if i.get("returnType2") is None:
        continue
    if (
        i.get("returnType2", {}).get("node", "") == "SimpleType"
        and i.get("returnType2", {}).get("name", {}).get("identifier", "") == "Date"
    ):
        if len(i.get("parameters", [1])) == 0:
            zmanim_calc_methods.add(name)
        else:
            zmanim_methods.add(name)
        continue
    else:
        other_methods.add(name)
# sort by alphabet
zmanim_calc_methods = sorted(zmanim_calc_methods)
zmanim_methods = sorted(zmanim_methods)
other_methods = sorted(other_methods)
print("pub enum Calculations {")
for i in zmanim_methods:
    print(f"    {i},")
print("}")
# print(len(zmanim_calc_methods))
# print(len(zmanim_methods))
# print(len(other_methods))
