# This is just an example to get you started. A typical binary package
# uses this file as the main entry point of the application.

import std/rdstdin
import std/enumerate
from std/strutils import parseUInt, splitLines, isUpperAscii
from std/sequtils import filter



proc max[T: SomeInteger](list: seq[T]): (int, T) =
        var m: T = 0
        var i: int

        for j, v in enumerate(list):
                if v > m:
                        m = v
                        i = j

        return (i, m)


proc rps_win(cand: char, opp: char): int =
        case opp:
        of 'A':
                case cand:
                of 'X':
                        return 0
                of 'Y':
                        return 1
                of 'Z':
                        return -1
                else:
                        discard
        of 'B':
                case cand:
                of 'X':
                        return -1
                of 'Y':
                        return 0
                of 'Z':
                        return 1
                else:
                        discard

        of 'C':
                case cand:
                of 'X':
                        return 1
                of 'Y':
                        return -1
                of 'Z':
                        return 0
                else:
                        discard
        else:
                discard

        raise newException(ValueError, "Invalid rock paper scissor move")

proc calc_score[T: SomeInteger](cand: char, opp: char, score: var T): void =
                case rps_win(cand, opp):
                of 1:
                        score += 6
                of 0:
                        score += 3
                of -1:
                        score += 0
                else:
                        raise newException(ValueError, "Invalid rock paper scissor move")

                case cand:
                of 'X':
                        score += 1
                of 'Y':
                        score += 2
                of 'Z':
                        score += 3
                else:
                        raise newException(ValueError, "Invalid rock paper scissor move")

proc calc_choice[T: SomeInteger](cand: char, opp: char, score: var T): void =
        case cand:
        of 'X':
                score += 0
                case opp
                of 'A':
                        score += 3
                of 'B':
                        score += 1
                of 'C':
                        score += 2
                else:
                        raise newException(ValueError, "Invalid rock paper scissor move")
        of 'Y':
                score += 3
                case opp:
                of 'A':
                        score += 1
                of 'B':
                        score += 2
                of 'C':
                        score += 3
                else:
                        raise newException(ValueError, "Invalid rock paper scissor move")

        of 'Z':
                score += 6
                case opp:
                of 'A':
                        score += 2
                of 'B':
                        score += 3
                of 'C':
                        score += 1
                else:
                        raise newException(ValueError, "Invalid rock paper scissor move")


        else:
                raise newException(ValueError, "Invalid rock paper scissor outcome")

proc taskThree(part: Natural): void =
    const letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
    let input = filter(splitLines(readAll(stdin)), proc(x: string): bool = x != "")
    var tot: uint64 = 0;

    case part:
    of 1:
        for r in input:
            for i, c in enumerate(letters):
                let c1 = r[0 .. r.len div 2 - 1]
                let c2 = r[r.len div 2 .. ^1]
                if c1.contains(c) and c2.contains(c):
                    let prio = cast[uint64](i) + 1
                    tot += prio
                    echo c, " in ", r, "tot: ", prio
    of 2:
        var i = 0
        while i < input.len:
            for k, c in enumerate(letters):
                if input[i].contains(c) and input[i+1].contains(c) and input[i+2].contains(c):
                    let prio = cast[uint64](k) + 1
                    tot += prio
                    break
            i += 3
    else:
        raise newException(ValueError, "Invalid task part")


    echo "Tot prio: ", tot


proc taskTwo(part: Natural): void =
    let input = splitLines(readAll(stdin))
    var score: uint64 = 0;
    for l in filter(input, proc(x: string): bool = x != ""):
            case part:
            of 1:
                    calc_score(l[2], l[0], score)
            of 2:
                    calc_choice(l[2], l[0], score)
            else:
                    raise newException(ValueError, "Invalid task part")

    echo "Total score: ", score

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


proc main(): int =
        echo("Hello, World!")
        var taskStr: string
        var partStr: string

        if not readLineFromStdin("Which task? ", taskStr):
                echo "Failed to read task"
                return -1

        if not readLineFromStdin("Which task part? ", partStr):
                echo "Failed to read task part"
                return -1

        case parseUInt(taskStr)
        of 1:
                taskOne()
        of 2:
                taskTwo(parseUInt(partStr))
        of 3:
                taskThree(parseUInt(partStr))
        else:
                echo("Not implemented!")

when isMainModule:
        discard main()
