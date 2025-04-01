import json
from pathlib import Path

pkg = Path("./pkg")
target = "japan_geoid.js"

with open(pkg / "package.json", encoding="utf-8") as f:
    package = json.load(f)

package["type"] = "module"
package["files"].append("japan_geoid_bg.wasm.d.ts")
package["main"] = "japan_geoid.js"

with open(pkg / "package.json", "w", encoding="utf-8") as f:
    json.dump(package, f, indent=2)


with open(pkg / target, encoding="utf-8") as f:
    lines = f.readlines()

patched = False
with open(pkg / target, "w", encoding="utf-8") as f:
    for line in lines:
        if line.strip() == "module_or_path = fetch(module_or_path);":
            f.write(
                """try {
            module_or_path = await fetch(module_or_path);
        } catch (e) {
            if (!(e instanceof TypeError)) {
                throw e;
            }
            module_or_path = await (await import("node:fs/promises")).readFile(module_or_path);
        }"""
            )
            patched = True
        else:
            f.write(line)

assert patched
