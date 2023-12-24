from dataclasses import dataclass
from sys import stdin


@dataclass
class Hailstone:
    p: (int, int, int)
    v: (int, int, int)


def parse_3tuple(s: str) -> (int, int, int):
    return tuple(int(x.strip()) for x in s.split(', '))


def parse_hailstone(hailstone: str) -> Hailstone:
    (p, v) = hailstone.split(' @ ')
    return Hailstone(parse_3tuple(p), parse_3tuple(v))


hs = [parse_hailstone(line) for line in stdin.readlines()]

variables = var('t0, t1, t2, t3, d1, d2')
ts = variables[:4]
ds = variables[4:]

assume(ts[0] > 0)

equations = [
        hs[j + 2].p[i] + ts[j + 2] * hs[j + 2].v[i] - (hs[j].p[i] + ts[j] * hs[j].v[i])
        == ds[j] * (hs[j + 1].p[i] + ts[j + 1] * hs[j + 1].v[i] - (hs[j].p[i] + ts[j] * hs[j].v[i]))
        for i in range(3) for j in range(2)
]

solutions = solve(equations, *ts, *ds)

# assume all times are >= 0
s = next(filter(lambda s: all(map(lambda v: v.subs(s) >= 0, ts)), solutions))

t0 = t0.subs(s)
t1 = t1.subs(s)
dt = t1 - t0
vs = [(hs[1].p[i] + hs[1].v[i] * t1 - (hs[0].p[i] + hs[0].v[i] * t0)) / dt for i in range(3)]
x, y, z = (hs[0].p[i] + hs[0].v[i] * t0 - vs[i] * t0 for i in range(3))

print(x + y + z)