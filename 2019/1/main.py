def module_mass(val):
    mass =  val / 3 - 2
    return max(0, mass)

def main():
    f_in = open("input", "rw")
    f_out = open("output", "rw")
    
    content = f_in.readlines()
    tot = 0
    while True:
        content = map(module_mass, map(int, content))
        tmp = sum(content)
        tot += tmp
        if tmp <= 0:
            break

    print tot

if __name__ == '__main__':
    main()
