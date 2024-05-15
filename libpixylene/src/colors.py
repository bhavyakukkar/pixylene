a = "#ffffff00" #white
b = "#00000000" #black
c = "#f44336ff"
d = "#ff9800ff"
e = "#ffc107ff"
f = "#ffeb3bff"
g = "#cddc39ff"
h = "#8bc348ff"
i = "#4caf50ff"
j = "#009688ff"
k = "#00bcd4ff"
l = "#03a9f4ff"
m = "#2196f3ff"
n = "#3f51b5ff"
o = "#673ab7ff"
p = "#9c27b0ff"
q = "#f06292ff"
r = "#ff1744ff"
s = "#ff5722ff"
t = "#3e2723ff"


grid = [
[a, a, a, a, a, b, b, b, b, b, b, a, a, a, a, a],
[a, a, a, b, b, o, p, q, r, a, a, b, b, a, a, a],
[a, a, b, a, a, n, o, p, q, a, a, a, a, b, a, a],
[a, b, a, a, l, m, n, o, p, q, a, a, a, a, b, a],
[a, b, a, j, k, a, a, a, a, p, q, a, a, a, b, a],
[b, g, h, i, a, a, a, a, a, a, p, q, r, s, t, b],
[b, f, g, h, a, a, a, a, a, a, o, p, a, a, s, b],
[b, a, f, g, a, a, a, a, a, a, n, a, a, a, a, b],
[b, a, a, f, g, a, a, a, a, l, m, a, a, a, a, b],
[b, a, a, e, f, g, h, i, j, k, l, m, a, a, p, b],
[b, a, c, d, b, b, b, b, b, b, b, b, m, n, o, b],
[a, b, b, b, a, a, b, a, a, b, a, a, b, b, b, a],
[a, a, b, a, a, a, b, a, a, b, a, a, a, b, a, a],
[a, a, b, a, a, a, b, a, a, b, a, a, a, b, a, a],
[a, a, a, b, a, a, a, a, a, a, a, a, b, a, a, a],
[a, a, a, a, b, b, b, b, b, b, b, b, a, a, a, a]
]

grids = [

[
[a, b, b],
[b, b, b],
[b, b, b],
],

[
[b, a, b],
[b, b, b],
[b, b, b],
],

[
[b, b, a],
[b, b, b],
[b, b, b],
],

[
[b, b, b],
[a, b, b],
[b, b, b],
],

[
[b, b, b],
[b, a, b],
[b, b, b],
],

[
[b, b, b],
[b, b, a],
[b, b, b],
],

[
[b, b, b],
[b, b, b],
[a, b, b],
],

[
[b, b, b],
[b, b, b],
[b, a, b],
],

[
[b, b, b],
[b, b, b],
[b, b, a],
],

]

#for i in range(len(grids)):
for i in range(1):
    #grid = grids[i]
    #print(f"let layer{i} = Layer {{ scene: Scene::new(Coord{{ x: 3, y: 3 }}, vec![")
    print(f"Scene::new(PCoord::new(3, 3).unwrap(), vec![")
    for row in grid:
        for x in row:
            print(
                "Some(Pixel{{r:{},g:{},b:{},a:{}}}),"
                .format(
                    *list(map(
                        lambda hx: int(f"0x{hx}", 16), [x[1:3], x[3:5], x[5:7], x[7:]]
                    ))
                ),
                end=''
            )
        print()
    print("]).unwrap()")
