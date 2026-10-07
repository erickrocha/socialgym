#!/usr/bin/env python3
"""Lists every REST route of `workout` as `METHOD /full/path`, read from the route files and lib.rs.

Used by tool/check_rest_inventory.sh (C-010 SYS-C010-001). The route files nest under the prefixes
set in workout/application/src/lib.rs; this reads both, so a new route shows up without a manual list.
"""
import re, sys, pathlib

root = pathlib.Path(__file__).resolve().parent.parent / "workout" / "application" / "src"
METHODS = ("get", "post", "put", "delete", "patch")


def call_args(src, start):
    """Text of the balanced parentheses that open at src[start] == '('."""
    depth, i = 0, start
    while i < len(src):
        c = src[i]
        if c == "(":
            depth += 1
        elif c == ")":
            depth -= 1
            if depth == 0:
                return src[start + 1:i]
        i += 1
    raise ValueError("unbalanced parentheses")


def routes_in(src):
    out = []
    for m in re.finditer(r"\.route\(\s*\"([^\"]+)\"\s*,", src):
        args = call_args(src, src.index("(", m.start()))
        rest = args[args.index(",", args.index(m.group(1)) + len(m.group(1))) + 1:]
        for mm in re.finditer(r"(?<![A-Za-z_])(%s)\s*\(" % "|".join(METHODS), rest):
            out.append((m.group(1), mm.group(1).upper()))
    return out


def join(*parts):
    path = "/".join(p.strip("/") for p in parts if p.strip("/"))
    return "/" + path


lib = (root / "lib.rs").read_text()
files = {
    "person_routes": "/people", "friend_routes": "/friends", "team_member_routes": "/team-members",
    "business_profile_routes": "/business-profiles", "workout_routes": "/workouts", "exercise_routes": "/exercises",
    "settings_routes": "/settings", "media_routes": "/media", "address_search_routes": "/address",
}
found = set()
base = "/workout/api"
for fn_name, prefix in files.items():
    if fn_name in ("media_routes", "address_search_routes"):
        src = lib[lib.index("fn %s" % fn_name):]
        src = src[:src.index("\n}\n")]
    else:
        src = (root / "routes" / (fn_name + ".rs")).read_text()
    for path, method in routes_in(src):
        found.add(f"{method} {join(base, prefix, path)}")
# resource_routes is merged under the API base without a prefix
src = lib[lib.index("fn resource_routes"):]
for path, method in routes_in(src[:src.index("\n}\n")]):
    found.add(f"{method} {join(base, path)}")
# routes mounted without the API base: public and authentication routes
src = lib[lib.index("fn welcome_route"):]
for path, method in routes_in(src[:src.index("\n}\n")]):
    found.add(f"{method} {join(path)}")
for path, method in routes_in(lib[lib.index("let app = Router::new()"):]):
    found.add(f"{method} {join(path)}")
for path, method in routes_in((root / "routes" / "authentication_routes.rs").read_text()):
    found.add(f"{method} {join(path)}")
for line in sorted(found, key=lambda s: (s.split(" ", 1)[1], s)):
    print(line)
