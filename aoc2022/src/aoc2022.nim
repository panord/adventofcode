# This is just an example to get you started. A typical binary package
# uses this file as the main entry point of the application.

import std/rdstdin
import std/enumerate
from std/strutils import parseInt, parseUInt, splitLines, isUpperAscii, split, replace, isDigit, multiReplace
from std/sequtils import filter, map
import std/strformat
from std/algorithm import sort

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

proc isStartSeq(s: seq[char]): bool =
    for i, e in enumerate(s):
      for j, p in enumerate(s):
        if i == j:
          continue
        if e == p:
          return false
    return true

type
  File = object
    size: uint64
    name: string

type
  Directory = object
    name: string
    files: seq[File]
    dirs: seq[ref Directory]
    parent: ref Directory


proc dirSize(d: ref Directory, sum: ref uint64): uint64 =
  var lsum: uint64 = 0
  for f in d.files:
    lsum += f.size

  for d in d.dirs:
    lsum += dirSize(d, sum)

  return sum[] + lsum


proc dirSizeThreshold(d: ref Directory, threshold: uint64, sum: ref uint64): uint64 =
  var lsum: uint64 = 0
  for f in d.files:
    lsum += f.size

  for d in d.dirs:
    lsum += dirSizeThreshold(d, threshold, sum)

  if lsum >= threshold:
    return sum[]

  sum[] += lsum
  return lsum

proc closestDirsize(d: ref Directory, closest: ref uint64, target: uint64): void =
  var sum: ref uint64 = new(uint64)
  let csz = dirSize(d, sum)
  if csz > target:
    echo d.name, ": ", csz
  if csz < closest[] and csz > target:
    echo d.name, ": ", csz, " closer than ", closest[], " to ", target
    closest[] = csz

  for d in d.dirs:
    closestDirsize(d, closest, target)

proc getdir(curr: ref Directory, name: string): ref Directory =
  if name == "..":
      return curr.parent
  for d in curr.dirs:
    if d.name == name:
      return d

  raise newException(ValueError, fmt"No such directory {name=}")


proc scenicScore(map: seq[string], x0: int, y0: int, print = false): uint =
  var thresh = parseInt(map[y0][x0..x0])
  var viewmap: seq[seq[int]]

  for y in map:
    viewmap.add(newSeq[int](y.len))

  for i,x in enumerate(viewmap):
    for j,y in enumerate(x):
      viewmap[i][j] = 0

  viewmap[y0][x0] = thresh

  var scenic: uint = 1
  var dist: uint = 0
  var up,down,left,right: uint= 0
  # Look to the right
  for x in x0 + 1..map[0].len - 1:
      let h = parseInt(map[y0][x..x])
      viewmap[y0][x] = h
      right += 1
      if h >= thresh:
        break

  # Look to the left
  for xr in map[0].len - x0 + 1..map.len:
      let x = ^xr
      let h = parseInt(map[y0][^xr..^xr])
      viewmap[y0][^xr] = h
      left += 1
      if h >= thresh:
        break

  # Look down
  for y in y0 + 1..map.len - 1:
      let h = parseInt(map[y][x0..x0])
      viewmap[y][x0] = h
      down += 1
      if h >= thresh:
        break

  # Look up
  # y0..0
  for yr in map.len - y0 + 1..map.len:
      let h = parseInt(map[map.len - yr][x0..x0])
      viewmap[^yr][x0] = h
      up += 1
      if h >= thresh:
        break

  if print:
    for r in viewmap:
      for c in r:
        stdout.write c
      stdout.write '\n'
  scenic = up * down * left * right
  echo "scenic (", x0, ", ", y0, "): ", scenic, " ", up, " * ", down, " * ", left, " * ", right
  return scenic

proc advanceLink(link: var tuple[x: int, y: int], prev: tuple[x: int, y: int]): tuple[x: int, y: int] =
  if abs(prev.x - link.x) > 1:
    # to the right
    if prev.x > link.x:
      link = prev
      link.x -= 1
    elif prev.x < link.x:
      link = prev
      link.x += 1
  elif abs(prev.y - link.y) > 1:
    if prev.y > link.y:
      link = prev
      link.y -= 1
    elif prev.y < link.y:
      link = prev
      link.y += 1

  return link

type
  Oper = object
    lhs: string
    rhs: string
    oper: string

type
  Test = object
    factor: int

type
  Monkey = object
    id: int
    items: seq[int]
    oper: Oper
    test: tuple[d: int, t: uint, f: uint]

proc updateWorry(x: int, oper: Oper): int =
  let lhs: int = (if oper.lhs == "old": x else: parseInt(oper.lhs))
  let rhs: int = (if oper.rhs == "old": x else: parseInt(oper.rhs))
  case oper.oper:
  of "*":
    return lhs * rhs
  of "+":
    return lhs + rhs
  else:
    raise newException(ValueError, "Invalid operation")

proc parseMonkey(str: seq[string]): Monkey =
  var m: Monkey
  m.id = parseInt(str[0].split({' ', ':'})[1])
  m.items = filter(str[1].split({' ', ','}),
              proc(x: string): bool = x != "")[2..^1].map(parseInt)
  var tmp = str[2].split(' ').filter(proc(x: string): bool = x != "")[^3..^1]
  m.oper = Oper(lhs: tmp[0], rhs: tmp[2], oper: tmp[1])
  m.test.d = parseInt(filter(str[3].split(' '), proc(x: string): bool = x != "")[^1])
  m.test.t = parseUInt(filter(str[4].split(' '), proc(x: string): bool = x != "")[^1])
  m.test.f = parseUInt(filter(str[5].split(' '), proc(x: string): bool = x != "")[^1])
  return m

proc monkeyThrow(m: var Monkey): (uint, int) =
  let item = m.items[0]
  var uitem = updateWorry(item, m.oper) div 3
  var to: uint = if uitem mod m.test.d == 0: m.test.t else: m.test.f
  m.items.delete(0)
  echo "Monkey ", m.id ," threw ", item, " to ", to, " with worry ", uitem
  return (to, uitem)

proc taskEleven(part: Natural): void =
  let input = filter(splitLines(readAll(stdin)), proc(x: string): bool = x != "")
  let rounds = 20
  var monkeys: seq[Monkey]
  var active: seq[int]

  var r = 0
  while r < input.len - 1:
    monkeys.add(parseMonkey(input[r..r+5]))
    r += 6
  active = newSeq[int](monkeys.len)

  for r in 1..rounds:
    for i in 0..monkeys.len - 1:
      active[i] += monkeys[i].items.len
      for e in 0..monkeys[i].items.len - 1:
        let (to, item) = monkeys[i].monkeyThrow()
        monkeys[to].items.add(item)
  sort(active, system.cmp[int])
  echo "Monkey business: ", active[^1] * active[^2]

proc taskNine(part: Natural): void =
  let input = filter(splitLines(readAll(stdin)), proc(x: string): bool = x != "")
  var rope: seq[tuple[x: int, y: int]]
  let size = 1000
  var print: bool = false

  var viewmap: seq[seq[char]]
  for i in 0..size - 1:
    viewmap.add(newSeq[char](size))

  case part:
  of 1:
      rope = newSeq[tuple[x: int, y: int]](2)
  of 2:
      rope = newSeq[tuple[x: int, y: int]](10)
  else:
    raise newException(ValueError, "Invalid part number")

  for i in 0..size - 1:
    for j in 0..size - 1:
      viewmap[i][j] = '.'

  viewmap[size div 2][size div 2] = 's'
  for l in 0..rope.len - 1:
    rope[l] = (size div 2, size div 2)

  for r in input:
    let move = r.split(' ')
    let dir = move[0]
    let dist = parseInt(move[1])
    for i in 1..dist:
      case dir:
      of "R":
        rope[0].x += 1
      of "L":
        rope[0].x -= 1
      of "D":
        rope[0].y += 1
      of "U":
        rope[0].y -= 1
      else:
        raise newException(ValueError, "Invalid direction")

      for j in 1..rope.len - 1:
        rope[j] = advanceLink(rope[j], rope[j-1])
      viewmap[rope[^1].y][rope[^1].x] = '#'

  var sum: uint64 = 0
  for y, r in enumerate(viewmap):
    for x, c in enumerate(r):
        if c == '#' or c == 's':
          sum += 1
  echo "Visited ", sum, " places"


proc taskEight(part: Natural): void =
  let input = filter(splitLines(readAll(stdin)), proc(x: string): bool = x != "")
  var vismap: seq[seq[uint8]]
  var vision: bool
  var thresh: int
  case part:
  of 1:
    for y, r in enumerate(input):
      let vr: seq[uint8] = newSeq[uint8](r.len)
      vismap.add(vr)
      for x, c in enumerate(r):
        vismap[y][x] = 0

    echo "Forest of size ", input.len, ", ", input[0].len
    echo "From the left"
    # row forward
    for y in 0..input.len-1:
      thresh = -1
      for x in 0..input[0].len-1:
        let h = parseInt(input[y][x..x])
        if h > thresh:
          vismap[y][x] = 1
          thresh = cast[int](h)

    echo "From the right"
    # row backward
    for y in 0..input.len - 1:
      thresh = -1
      for xr in 1..input[0].len:
        let x = ^xr
        let h = parseInt(input[y][x..x])
        if h > thresh:
          vismap[y][x] = 1
          thresh = cast[int](h)

    echo "From the top"
    # Column first
    for x in 0..input[0].len-1:
      thresh = -1
      for y in 0..input.len-1:
        let h = parseInt(input[y][x..x])
        if h > thresh:
          vismap[y][x] = 1
          thresh = cast[int](h)

    echo "From the bottom"
    # Column reverse
    for x in 0..input[0].len-1:
      thresh = -1
      for yr in 1..input.len:
        let y = ^yr
        let h = parseInt(input[y][x..x])
        if h > thresh:
          echo "Height ", h, " > ", thresh
          vismap[y][x] = 1
          thresh = cast[int](h)

    for r in vismap:
      for c in r:
        stdout.write c
      stdout.write '\n'

    var sum: uint = 0
    for e in vismap:
      for i in e:
        sum += i
    echo "Total visable trees: ", sum
  of 2:
    discard scenicScore(input, 2, 1, true)
    discard scenicScore(input, 2, 3, true)
  of 3:
    var m: uint = 0
    var mx: int = 0
    var my: int = 0
    for y in 1..input.len - 2:
      for x in 1..input[0].len - 2:
        let ss = scenicScore(input, x, y)
        if ss > m:
          mx = x
          my = y
          m = ss

    discard scenicScore(input, mx, my, true)
    echo "Scenic score max (", mx, ", ", my, ", ", input[my][mx], "): ", m

    #echo "Maximum scenic score: ", m
  else:
    raise newException(ValueError, "Invalid task part")



proc taskSeven(part: Natural): void =
  let input = filter(splitLines(readAll(stdin)), proc(x: string): bool = x != "")
  var root: ref Directory = new(Directory)
  root.name = "/"
  root.parent = nil
  var curr: ref Directory = root

  # Skip cd /
  var i = 1
  while i < input.len:
    var r = input[i]
    let cmd = split(r, ' ')
    if r.contains('$'):
      case cmd[1]:
      of "cd":
          curr = getdir(curr, cmd[2])
      of "ls":
        i += 1
        continue
      else:
        echo "unknown command ", cmd[1]
    else:
      case cmd[0]:
      of "dir":
        var dir = new(Directory)
        dir.name = cmd[1]
        dir.parent = curr
        curr.dirs.add(dir)
      else:
        var file: File
        file.size = parseUInt(cmd[0])
        file.name = cmd[1]
        curr.files.add(file)
    i += 1


  var sum: ref uint64 = new(uint64)
  var closest: ref uint64 = new(uint64)
  discard dirSizeThreshold(root, 100000, sum)
  echo "Tot size of '/' under 100000 ", sum[]
  sum[] = 0
  closest[] = dirSize(root, sum)
  let free = 30000000 - (70000000 - closest[])
  echo "Need free space of ", free, " using ", closest[], " out of ", 70000000
  closestDirsize(root, closest, free)
  echo "Closest dir to update '/' ", closest[]

proc taskSix(part: Natural): void =
  let input = readAll(stdin)
  var start: seq[char]
  var startmsg: seq[char]
  var first: bool = true
  var j = 1
  var k = 1

  for i, c in enumerate(input):
    start.add(c)
    startmsg.add(c)
    if start.len == 5:
      start.delete(0)

    if startmsg.len == 15:
      startmsg.delete(0)

    if start.len == 4 and isStartSeq(start) and first:
        echo "Start packet Index: ", j, " seq ", @start
        first = false

    if startmsg.len == 14 and isStartSeq(startmsg):
      echo "Start msg Index: ", k, " seq ", @startmsg
      break

    j += 1
    k += 1


proc taskFive(part: Natural): void =
  let input = splitLines(readAll(stdin))
  var tot: uint64 = 0
  var stacks: seq[seq[string]]

  var p = 0
  echo "Building stacks"
  for r in input:
    if r == "":
        break

    var tops: seq[string]
    var i = 0
    while i < r.len:
        if i > r.len - 4:
          tops.add(r[i..^1])
        else:
          tops.add(r[i..i+3])
        i += 4

    if r.find('1') > 0:
        p += 2
        break

    let crates = tops.len
    for i,t in enumerate(tops):
        if i >= stacks.len:
          var st: seq[string]
          stacks.add(st)

        if tops[i] != "":
          stacks[i].insert(t)
    p += 1

  stacks = map(stacks,
    proc(x: seq[string]): seq[string] =
      filter(x,
        proc(y: string): bool =
          for c in y:
            if c != ' ':
              return true
      )
  )
  echo "Processing moves"
  for i in p .. input.len - 1:
    let r = split(input[i], ' ')
    if r.len < 5:
        break

    let cnt = parseUInt(r[1])
    let frm = parseUInt(r[3]) - 1
    let to = parseUInt(r[5]) - 1

    case part:
    of 1:
        for j in 1..cnt:
          stacks[to].add(stacks[frm][^1])
          stacks[frm].delete(stacks[frm].len - 1)
    of 2:
        for j in 1..cnt:
          stacks[to].add(stacks[frm][stacks[frm].len - cast[int]((cnt - j + 1))])

        for j in 1..cnt:
          stacks[frm].delete(stacks[frm].len - 1)
    else:
        raise newException(ValueError, "Invalid task part")


  var res = ""
  for i in stacks:
    res.add(i[^1].multiReplace( ("[", ""), ("]", "")))

  echo res.replace(" ", "")






proc taskFour(part: Natural): void =
  let input = filter(splitLines(readAll(stdin)), proc(x: string): bool = x != "")
  var tot: uint64 = 0;
  case part:
  of 1:
    for r in input:
      let ranges = split(r, ',')
      let r1 = split(ranges[0], '-')
      let r2 = split(ranges[1], '-')

      let s1 = parseUInt(r1[0])
      let e1 = parseUInt(r1[1])
      let s2 = parseUInt(r2[0])
      let e2 = parseUInt(r2[1])

      if s1 <= s2 and e2 <= e1 or s2 <= s1 and e1 <= e2:
        tot += 1
  of 2:
    for r in input:
      let ranges = split(r, ',')
      let r1 = split(ranges[0], '-')
      let r2 = split(ranges[1], '-')

      let s1 = parseUInt(r1[0])
      let e1 = parseUInt(r1[1])
      let s2 = parseUInt(r2[0])
      let e2 = parseUInt(r2[1])

      if s1 <= e2 and e1 >= s2 or s2 <= e1 and e2 >= s1:
        tot += 1
  else:
    raise newException(ValueError, "Invalid task part")

  echo "Tot: ", tot

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
  var taskStr: string
  var partStr: string

  if not readLineFromStdin("Which task? ", taskStr):
    echo "Failed to read task"
    return -1

  if not readLineFromStdin("Which task part? ", partStr):
    echo "Failed to read task part"
    return -1

  var task: uint64
  var part: uint64

  try:
      task = parseUint(taskStr)
      part = parseUint(partStr)
  except ValueError:
    echo "Invalid task or part number"

  case task
  of 1:
    taskOne()
  of 2:
    taskTwo(part)
  of 3:
    taskThree(part)
  of 4:
    taskFour(part)
  of 5:
    taskFive(part)
  of 6:
    taskSix(part)
  of 7:
    taskSeven(part)
  of 8:
    taskEight(part)
  of 9:
    taskNine(part)
  of 11:
    taskEleven(part)
  else:
    echo("Not implemented!")


when isMainModule:
  discard main()
