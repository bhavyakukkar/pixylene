actions['example'] = {
    perform = function(self, project, console)
        --[[
        ----scene1 = layer1.scene
        --print(layer2.scene:get(U(0, 0)).red)
        --local merged_scene = Layer.merge(P(2, 2), layer2, layer1, BlendMode.NORMAL)
        --print(merged_scene:get(U(0, 0)).red)
    
        --local s = "#69426942"
        --local color = Pixel.hex(s)
        --print(color.red .. ' ' .. color.green .. ' ' .. color.blue .. ' ' .. color.alpha)
        --print(s)
    
        local color = { ["red"] = 1, ["green"] = 2, ["blue"] = 3 }
        local palette2 = Palette{ [color.green] = '#000000', [color.red] = '#4455dd' }
    
        --local scene3 = canvas:merge()
        --print(scene3:get(U(0,0)).red)
        --local scene4 = canvas:merge()
        --print(scene4:get(U(0,0)).red)
    
        --print(canvas.palette:get(1).red)
        --canvas.palette = palette2
        --print(canvas.palette:get(1).red)
        --canvas.palette = palette
        --print(canvas.palette:get(1).red)
    
        print(canvas:get(0).scene:get(U()).red)
        canvas:move(0,1)
        print(canvas:get(0).scene:get(U()).red)
        --]]
    
        local scene1 = Scene(P(2, 2), { Pixel(), Pixel(), Pixel(), Pixel() })
        local layer1 = Layer(scene1, 255, false, BlendMode.NORMAL)
    
        local scene2 = Scene(P(2, 2), { Pixel(255), Pixel(255), Pixel(255), Pixel(255) })
        local layer2 = Layer(scene2, 127, false, BlendMode.NORMAL)
    
        local palette = Palette{ [1] = '#dd5544ff', [2] = '#55dd44ff' }
        local canvas = Canvas(P(2, 2), {layer1, layer2}, palette)
        local project = Project(canvas)
    
        --print(canvas:get(0).scene:get(U()).red)
        print(project:focus().coord.x)
        project:focus(C(69,420), 0)
        print(project:focus().coord.x)

        print(canvas:get(1).scene:get(U()).red)

        local canvas = project.canvas
        canvas:set(1, layer1)
        project.canvas = canvas
        print(canvas:get(1).scene:get(U()).red)
    end
}

actions['echo'] = {
    perform = function(self, project, console)
    end
}
