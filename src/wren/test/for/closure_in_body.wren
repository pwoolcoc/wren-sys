var list = []

for (i in [1, 2, 3]) {
  var j = i + 1
  list.add(new Fn { IO.print(j) })
}

for (f in list) f.call
// expect: 2
// expect: 3
// expect: 4
