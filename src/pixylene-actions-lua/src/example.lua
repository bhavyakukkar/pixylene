action.name = 'echo'
function action:perform()
    local p1 = Pixel(255)
    local p2 = Pixel.hex("#69426942")
    print(p1.r .. "," .. p1.g .. "," .. p1.b .. "," .. p1.a)
    print(p2.r .. "," .. p2.g .. "," .. p2.b .. "," .. p2.a)
end
