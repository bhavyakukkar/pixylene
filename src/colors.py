a = "#ffffff00" #white
b = "#00000000" #black
c = "#f44336"
d = "#ff9800"
e = "#ffc107"
f = "#ffeb3b"
g = "#cddc39"
h = "#8bc348"
i = "#4caf50"
j = "#009688"
k = "#00bcd4"
l = "#03a9f4"
m = "#2196f3"
n = "#3f51b5"
o = "#673ab7"
p = "#9c27b0"
q = "#f06292"
r = "#ff1744"
s = "#ff5722"
t = "#3e2723"


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

for i in range(len(grids)):
    grid = grids[i]
    print(f"let layer{i} = Layer {{ scene: Scene::new(Coord{{ x: 3, y: 3 }}, vec![")
    for row in grid:
        for x in row:
            print(
                "Some(Pixel4{{r:{},g:{},b:{},a:{}}}),"
                .format(
                    *list(map(
                        lambda hx: int(f"0x{hx}", 16), [x[1:3], x[3:5], x[5:7], x[7:]]
                    ))
                ),
                end=''
            )
        print()
    print("]).unwrap(), opacity: 255 };")
