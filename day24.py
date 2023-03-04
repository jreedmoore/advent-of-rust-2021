import math

def neql(a,b):
    if a == b:
        return 0
    else:
        return 1

def f(a,b,c,d,z):
    nz = math.floor(z/a)
    comp = neql(z % 26 + b, d)
    return (25*comp+1)*nz + (c+d)*comp

def digits(d):
    l = []
    while d != 0:
        l.append(d % 10)
        d = math.floor(d / 10)
    l.reverse()
    return l

def g(d):
    inp = [
        (1,13,8),
        (1,13,8),
        (1,12,8),
        (1,10,10),
        (26,-11,12),
        (1,15,13),
        (1,10,5),
        (26,-2,10),
        (26,-6,3),
        (1,14,2),
        (26,0,2),
        (26,-15,12)
    ]
    ds = digits(d)
    z = 0
    for ((a,b,c), d) in zip(inp,ds):
        nz = f(a,b,c,d,z)
        print(f"{d} - ({a},{b},{c}) => {nz}")
        z = nz
    return z