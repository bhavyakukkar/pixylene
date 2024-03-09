
action.name = 'example'
function action:perform()
    --local scene1 = Scene(PCoord(2, 2), { Pixel(), Pixel(), Pixel(), Pixel() })
    --local layer1 = Layer(scene1, 255, false, BlendMode.NORMAL)

    --local scene2 = Scene(PCoord(2, 2), { Pixel(255), Pixel(255), Pixel(255), Pixel(255) })
    --local layer2 = Layer(scene2, 127, false, BlendMode.NORMAL)

    ----scene1 = layer1.scene
    --print(layer2.scene:get(UCoord(0, 0)).red)
    --local merged_scene = Layer.merge(PCoord(2, 2), layer2, layer1, BlendMode.NORMAL)
    --print(merged_scene:get(UCoord(0, 0)).red)
    local color = { ["red"] = 1, ["green"] = 200 }

    local palette = Palette{ [color.red] = '#dd5544', [color.green] = '#55dd44' }

    local white1 = palette:get_color(color.red)
    local white2 = palette:get_color(color.red)
    local black = palette:get_color(color.green)
    print(white1.red)
    print(white2.red)
    print(black.red)

    --local s = "#69426942"
    --local color = Pixel.hex(s)
    --print(color.red .. ' ' .. color.green .. ' ' .. color.blue .. ' ' .. color.alpha)
    --print(s)
end
