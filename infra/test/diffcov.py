#!/usr/bin/env python3
"""Line coverage of the lines a change added: an lcov report intersected with `git diff`.

    diffcov.py LCOV BASE PATH [--lcov-root DIR] [--exclude REGEX]

LCOV       the lcov report (cargo llvm-cov report --lcov, flutter test --coverage)
BASE       the commit the change starts from; every line added since then counts, including files git
           does not track yet (new files count in full)
PATH       the directory of the repository the change touched (workout, socialgym_mobile, ...)
--lcov-root  what relative `SF:` paths in the report are relative to (flutter writes them relative to the
           package directory); absolute paths are made relative to the repository root
--exclude  files left out of the measurement (generated code, entrypoints), as a regex on the repo path
Run it from the repository root. Prints one line per file and the total.
"""
import argparse, collections, os, re, subprocess, sys

ap = argparse.ArgumentParser()
ap.add_argument('lcov'); ap.add_argument('base'); ap.add_argument('path')
ap.add_argument('--lcov-root', default=None); ap.add_argument('--exclude', default=None)
a = ap.parse_args()
root = os.getcwd()
exclude = re.compile(a.exclude) if a.exclude else None

cov = collections.defaultdict(dict); f = None
for line in open(a.lcov):
    line = line.strip()
    if line.startswith('SF:'):
        p = line[3:]
        p = os.path.relpath(p, root) if os.path.isabs(p) else os.path.join(a.lcov_root or '.', p)
        f = os.path.normpath(p)
    elif line.startswith('DA:'):
        n, h = line[3:].split(',')[:2]
        cov[f][int(n)] = cov[f].get(int(n), 0) + int(h)

added = collections.defaultdict(set); cur = None
diff = subprocess.run(['git', 'diff', '-U0', '-M', a.base, '--', a.path], capture_output=True, text=True).stdout
for line in diff.splitlines():
    if line.startswith('+++ b/'): cur = line[6:]
    elif line.startswith('@@'):
        m = re.search(r'\+(\d+)(?:,(\d+))?', line)
        start, count = int(m[1]), int(m[2] if m[2] is not None else 1)
        if cur: added[cur].update(range(start, start + count))
untracked = subprocess.run(['git', 'ls-files', '--others', '--exclude-standard', '--', a.path], capture_output=True, text=True).stdout.split()
for path in untracked:
    try:
        added[path].update(range(1, sum(1 for _ in open(path, errors='ignore')) + 1))
    except OSError:
        pass

rows = []; total = hit = 0
for path, lines in sorted(added.items()):
    if exclude and exclude.search(path): continue
    if path not in cov: continue
    executable = [n for n in lines if n in cov[path]]
    if not executable: continue
    h = sum(1 for n in executable if cov[path][n] > 0)
    total += len(executable); hit += h
    rows.append((h / len(executable) * 100, path, h, len(executable), sorted(n for n in executable if cov[path][n] == 0)))
for pct, path, h, t, miss in sorted(rows):
    print(f"{pct:6.1f}% {h}/{t} {path}  miss:{miss[:10]}")
print(f"TOTAL {hit}/{total} = {hit / total * 100:.2f}%" if total else "TOTAL no executable added lines")
