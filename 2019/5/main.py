import re
import sys

instr = {
    1 : {"op":lambda x,y: x+y, "argc":2, "ret": 1},
    2 : {"op":lambda x,y: x*y, "argc":2, "ret": 1},
    3 : {"op":lambda x: x,     "argc":0, "ret": 1 },
    4 : {"op":lambda x: print(x), "argc":1, "ret": 0 },
    99 : {"op":lambda : 99, "argc":0, "ret": 0},
}

def get_instr(op_code):
    cmd = instr.get(op_code, None)
    if not cmd:
        print("Op %d not implemented"  % op_code)
        return None
    return cmd

def exec_op(ip, prog):
    indir = lambda i : prog[i]
    direc = lambda i : i
    op_code = prog[ip]
    print("op", op_code)
    mode_l = []
    if len(str(op_code)) >= 2:
        op_code = int(str(op_code)[-2:])
        print("decoded opcode", op_code)
        mode_l = list(map(int, list(str(op_code)[:-2])))
        print("modes", mode_l)

    cmd = get_instr(op_code)
    if not cmd:
        op_code = 99
        cmd = get_instr(99)
    fn = cmd["op"]
    argc = cmd["argc"]
    ret = cmd["ret"]
    mode_l = mode_l if mode_l else [0]*argc
    arg_l = [None]*argc
    _tmp = argc - 1
    for arg in prog[ip + 1:ip + argc + 1]:
        _get = direc if mode_l[_tmp] else indir
        arg_l[_tmp] = _get(_tmp)
        _tmp -= 1

    if ret:
        if op_code == 3:
            out = prog[ip]
        else:
            out = prog[ip + 1]
            prog[out] = fn(*arg_l)

    if op_code == 99:
        return 0
    return ip + argc + ret + 1

def run(prog):
    ip = 0
    while ip < len(prog):
        print("Ip", ip)
        ip = exec_op(ip, prog)
        if ip == 0:
            print("Halt, Exit")
            break
    return prog[0], ','.join(map(str, prog))

def decode(prog):
    return map(int, prog.split(','))

def main():
    f_in = sys.stdin
    f_out = open("output", "w")
    prog = list(decode(f_in.read()))
    run(prog)

if __name__ == '__main__':
    main()
