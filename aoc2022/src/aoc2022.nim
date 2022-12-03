# This is just an example to get you started. A typical binary package
# uses this file as the main entry point of the application.

import std/rdstdin
import std/enumerate
from std/strutils import parseUInt, splitLines



proc max[T: SomeInteger](list: seq[T]): (int, T) =
        var m: T = 0
        var i: int

        for j, v in enumerate(list):
                if v > m:
                        m = v
                        i = j

        return (i, m)


proc taskOne(): void =
        var list: seq[uint64]
        var i: uint64 = 0

        var curr: uint64 = 0;
        let input = splitLines(readAll(stdin))

        for l in input:
                try:
                        curr += parseUInt(l)
                except ValueError:
                        if curr > 0:
                                list.add(curr)
                                curr = 0
                                i += 1;


        var tot: uint64 = 0
        for i in 0..2:
                let (j, v) = max(list)
                list.delete(j)
                echo "Ask elf ", i, ": ", v
                tot += v

        echo "Total: ", tot

when isMainModule:
        echo("Hello, World!")
        var taskStr: string
        var ok = readLineFromStdin("Which task? ", taskStr)

        case parseUInt(taskStr)
        of 1:
                taskOne()
        else:
                echo("Not implemented!")


