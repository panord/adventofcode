import re
import sys

instr = {
    1 : {"op":lambda x,y: x+y, "ip": 4, "argc":2 },
    2 : {"op":lambda x,y: x*y, "ip": 4, "argc":2 },
    99 : {"op":lambda : 99, "ip": 1, "ip": 0, "argc":0},
}

def get_instr(op_code):
    cmd = instr.get(op_code, None)
    if not cmd:
        return {"op" : lambda : 99,#sys.stdout.write("Unknown Operation %d" % op_code),
                "ip"  : 0,
                "argc": 0}
    return cmd

def exec_op(ip, prog):
    val = lambda i : prog[i]
    op_code = prog[ip]
    cmd = get_instr(op_code)
    fn = cmd["op"]
    argc = cmd["argc"]
    ip_step = cmd["ip"]

    arg_l = map(val, prog[ip+1:ip+argc+1])
    out = prog[ip + argc + 1]
    if not fn:
        return None, None, -1
    if op_code == 99:
        return None, None, 0
    return out, fn(*arg_l), ip + ip_step

def run(prog):
    ip = 0
    while ip < len(prog):
        #print(prog)
        #print(prog[ip:ip+4])
        #print(prog)
        out, val, ip = exec_op(ip, prog)
        if ip == -1:
        #    print("Halt, Error")
            break
        if ip == 0:
        #    print("Halt, Exit")
            break
        #print("ip:%d prog[%d] = %d" %(ip, out, val))
        prog[out] = val

    return prog[0], ','.join(map(str, prog))

def decode(prog):
    return map(int, prog.split(','))

def main():
    f_in = open("input", "rw")
    f_out = open("output", "w")
    prog = decode(f_in.read())
    val = 0
    out = []
    for i in range(100):
        for j in range(100):
            prog[1] = i
            prog[2] = j
            try:
                val, out = run(prog[:])
                print(val)
                if val == 19690720:
                    print("noun: %d verb: %d ans: %d" % (i, j, i * 100 + j))
                    f_out.write(out)
                    return
            except:
                pass
    f_out.write(out)

if __name__ == '__main__':
    main()
