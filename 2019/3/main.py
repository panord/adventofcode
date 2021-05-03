import re
import math
def mk_point_diff(word):
    code = word[0];
    off = int(word[1:])
    op = {
        "R": lambda x,y : (x + off, y),
        "L": lambda x,y : (x - off, y),
        "U": lambda x,y : (x, y + off),
        "D": lambda x,y : (x, y - off),
    }
    return op[code]

def rd_paths(prog):
    paths = list(filter(bool, re.split('\n', prog)))
    def decode_path(path):
        return list(map(mk_point_diff, re.split(',', path)))
    return list(map(decode_path, paths))

def steps_from(last, curr):
    return abs(last[0] - curr[0]) + abs(last[1] - curr[1])

def mk_points_2(path):
    extend = lambda x : [x,[0,0]]
    fctor_ex = lambda fct : fct[0](*fct[1])
    path = list(map(extend, path))
    pts = [(0,0)]
    l_costs = [0]
    for i in range(len(path)):
        curr = path[i-1][1]
        p = fctor_ex((path[i][0], curr))
        path[i][1] = p
        l_costs.append(l_costs[i] + steps_from(path[i-1][1], path[i][1]))
        pts.append(p)
    return pts,l_costs

def mk_points(path):
    extend = lambda x : [x,[0,0]]
    fctor_ex = lambda fct : fct[0](*fct[1])
    path = list(map(extend, path))
    pts = [(0,0)]
    for i in range(len(path)):
        curr = path[i-1][1]
        p = fctor_ex((path[i][0], curr))
        path[i][1] = p
        pts.append(p)
    return pts 

def mk_intsc(pts):
    def app_if(l, i, b):
        if not b:
           return
        l.append(i)

    intsecs = []
    l = pts[0]
    o = pts[1]
    for i in range(len(l) - 2):
        for j in range(len(o) - 2):
            li  = l[i:i+2]
            oj  = o[j:j+2]
            b,p = intersec(li, oj)
            app_if(intsecs, p, b)
    return intsecs

def mk_intsc_2(pts):
    def app_if(l, i, b):
        if not b:
           return
        l.append(i)

    intsecs = []
    l = pts[0][0]
    o = pts[1][0]
    lc = pts[0][1]
    oc = pts[1][1]
    for i in range(len(l) - 2):
        for j in range(len(o) - 2):
            li  = l[i:i+2]
            oj  = o[j:j+2]
            b,p = intersec(li, oj)
            app_if(intsecs, (p, (li[1], lc[i]), (oj[1], oc[j])), b)
    return intsecs

def manhattan_dist(pt):
    return pt,sum(map(abs, pt))

def intersec(l, o):
    if not is_seg_intersec(l, o):
        return False, (0,0)
    
    def line(p1, p2):
        A = (p1[1] - p2[1])
        B = (p2[0] - p1[0])
        C = (p1[0]*p2[1] - p2[0]*p1[1])
        return A, B, -C

    def do_intersection(L1, L2):
        D  = L1[0] * L2[1] - L1[1] * L2[0]
        Dx = L1[2] * L2[1] - L1[1] * L2[2]
        Dy = L1[0] * L2[2] - L1[2] * L2[0]
        if D != 0:
            x = Dx / D
            y = Dy / D
            return True, (x,y)
        else:
            return False, (0,0)  
    res = do_intersection(line(*l), line(*o))
    if res[0]:
        print("Hit!:", l, o, "at\t\t",  res[1])
    return res

def is_seg_intersec(l, o):
    x = lambda p : p[0]
    y = lambda p : p[1]
    xd = lambda p1, p2 : p1[0] - p2[0]
    yd = lambda p1, p2 : p1[1] - p2[1]
    def on_seg(p, q, r):
        return min(x(p), x(r)) <= x(q) and x(q) <= max(x(p), x(r)) \
           and min(y(p), y(r)) <= y(q) and y(q) <= max(y(p), y(r))

    def orient(p, q, r):
        val = yd(q, p) * xd(r, q) \
            - xd(q, p) * yd(r, q)
        if val == 0:
            return 0
        return 1 if val > 0 else 2

    p1 = l[0]
    q1 = l[1]
    p2 = o[0]
    q2 = o[1]

    o1 = orient(p1, q1, p2)
    o2 = orient(p1, q1, q2)
    o3 = orient(p2, q2, p1)
    o4 = orient(p2, q2, q1)
    # Only first case seems to matter for input 
    return o1 != o2 and o3 != o4 #\
        #or o1 == 0 and on_seg(p1, p2, q1) \
        #or o2 == 0 and on_seg(p1, q2, q1) \
        #or o3 == 0 and on_seg(p2, p1, q2) \
        #or o4 == 0 and on_seg(p2, q1, q2)

def main():
    f_in = open("input", "r")
    f_out = open("output", "w")
    paths = rd_paths(f_in.read())
    task1 = False
    if task1:
        points = list(filter(bool, map(mk_points, paths)))
        l_intersec = mk_intsc(points)
        manhattan = list(map(manhattan_dist, l_intersec))
        nearest = manhattan[0]
        for m in manhattan:
            nearest = m if m[1] < nearest[1] else nearest
        print("Nearest:", nearest)
    else:
        strip_cost = lambda x : x[:-1]
        points = list(map(mk_points_2, paths))
        # returns intsec, p1, p1steps, p2, p2steps
        l_intersec = mk_intsc_2(points)
        print (l_intersec)
        steps = []
        for i in l_intersec:
            cost_to = 0
            p  = i[0]
            _l = i[1]
            _o = i[2]
            cost_prev = _l[1] + _o[1]
            cost_to += steps_from(_l[0], p)
            cost_to += steps_from(_o[0], p)
            steps.append(cost_prev + cost_to)

        #manhattan = list(map(manhattan_dist, l_intersec))
        #nearest = manhattan[0]
        #for m in manhattan:
        #    nearest = m if m[1] < nearest[1] else nearest

if __name__ == '__main__':
    main()
