import sys
import re

def is_asc(pad):
    is_asc = True 
    for i in range(len(pad) - 1):
        is_asc = is_asc and int(pad[i-1]) <= int(pad[i])
    return is_asc

def two_adj(pad):
    for i in range(len(pad) - 1):
        if pad[i-1] == pad[i]:
            return i;
    return False

def ok_pw(pw):
    adj = False
    adj_cand = None
    pad =  pw +"0"
    return is_asc(pad) and two_adj(pad)

def two_adj_exact(pw):
    pw = "xxxx" + pw + "xxxx"
    for c in range(len(pw)):
        if pw[c-1] != pw[c] and pw[c] == pw[c+1] and pw[c]!= pw[c+2]:
            return pw
    return False
def main():
    f_in = sys.stdin
    interval = list(map(int, f_in.read().split('-')))
    print(interval)
    tot = 0
    print("Calculating passwords in", interval)
    ok_pws = []
    for pw in range(interval[0], interval[1]):
        is_ok = ok_pw(str(pw))
        if is_ok:
            ok_pws.append(str(pw))
            tot += 1
    print(len(ok_pws))
    ok_pws_2 = list(filter(bool, map(two_adj_exact, ok_pws)))
    tot_2 = len(ok_pws_2)
    print(ok_pws_2)
    print("Task 1:", tot)
    print("Task 2:", tot_2)

if __name__ == '__main__':
    main()
    print(two_adj_exact("112233"))
    print(two_adj_exact("123444"))
    print(two_adj_exact("111122"))
