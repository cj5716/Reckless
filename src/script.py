PARAMETERS = "parameters.rs"

file = open(PARAMETERS, "r")
params = []

for line in file.readlines():
    if (
        line.startswith("    i32")
        or line.startswith("    usize")
        or line.startswith("    f32")
        or line.startswith("    f64")
    ):
        (type, name, default) = line.split()

        params.append(
            {
                "type": type,
                "name": name[:-1],
                "default": float(default[:-1]),
            }
        )

for v in params:
    (name, default, mn, mx) = (v["name"], v["default"], 1, v["default"] * 2.0)

    type = "int" if v["type"] in ["i32", "usize"] else "float"
    step = min((mx - mn) / 30, 200)

    if v["type"] in ["i32", "usize"] == "usize":
        step = round(step)
    else:
        step = round(step, 3)

    if v["type"] in ["i32", "usize"] and "depth" in v["name"]:
        step = 0.5

    print(f"{name}, {type}, {default}, {mn}, {mx}, {step}, 0.002")