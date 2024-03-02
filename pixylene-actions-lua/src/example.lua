
action.name = 'echo'
function action:perform()
    local n = BlendMode.COMPOSITE(255,0)
    local bg = Pixel(0,0,0,255)
    local fg = Pixel(255,255,255,127)

    local merged = n:blend(fg, bg)
    print(merged.red .. "," .. merged.green .. "," .. merged.blue .. "," .. merged.alpha)
end
