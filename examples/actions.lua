-- Greets the user (only once) after asking for their name
actions['greet'] = {
    count = 0,
    perform = function(self, project, console)
        if self.count == 0 then
            local name = console:cmdin("Your Name? ")
            if (name == nil or name == '') then
                name = "there"
            end
            console:cmdout("Hello, " .. name .. ".")
            self.count = self.count + 1
        else
            console:cmdout("I only greet once.")
        end
    end
}

-- go to the next layer
actions['layernext'] = {
    perform = function(self, project, console)
        if (project.focus.layer < project.canvas.num_layers - 1) then
            project.focus = { ['layer'] = project.focus.layer + 1, ['coord'] = project.focus.coord }
        else
            console:cmdout("this is the last layer", LogType.WARNING)
        end
    end
}

-- go to the previous layer
actions['layerprev'] = {
    perform = function(self, project, console)
        if (project.focus.layer > 0) then
            project.focus = { ['layer'] = project.focus.layer - 1, ['coord'] = project.focus.coord }
        else
            console:cmdout("this is the first layer", LogType.WARNING)
        end
    end
}

-- zooms in, i.e., increments the project multiplier by 1
actions['zoomin'] = {
    perform = function(self, project, console)
        project:set_mul(project.mul + 1)
        console:cmdout("zoomed in")
    end
}

-- zooms out, i.e., decrements the project multiplier by 1
actions['zoomout'] = {
    perform = function(self, project, console)
        if (project.mul > 1) then
            project:set_mul(project.mul - 1)
            console:cmdout("zoomed out")
        end
    end
}

-- duplicates the Current Layer & moves focus to it
actions['dupcurlayer'] = {
    perform = function(self, project, console)
        project.canvas:add(project.canvas:layer(project.focus.layer))
        project.focus = { ['coord'] = project.focus.coord, ['layer'] = project.focus.layer + 1 }
    end
}

actions['noise'] = {
    perform = function(self, project, console)
        local f = tonumber(console:cmdin("Noise Factor (0.00 - 1.00): "))
        local d = project.canvas.dim
        for i=0, (d.x - 1) do
            for j=0, (d.y - 1) do
                if (math.random() < f) then
                    local c = project.canvas:layer(project.focus.layer).scene:get(UC(i, j))
                    project.canvas:layer(project.focus.layer).scene:set(
                        UC(i, j), 
                        BlendMode.NORMAL:blend(P.hex("#ffffff66"), c)
                    )
                end
            end
        end
    end
}

actions['circularoutline'] = {
    -- https://rosettacode.org/wiki/Bitmap/Midpoint_circle_algorithm?oldid=358330
    perform = function(self, project, console)
        if (project.num_cursors ~= 1) then
            console:cmdout("need exactly one cursor at center of circle", LogType.ERROR)
            return
        end

        local rad = tonumber(console:cmdin("Radius? "))
        if (rad == 0) then
            console:cmdout("radius cannot be 0", LogType.ERROR)
            return
        end

        local col = project.canvas.palette:get()

        local cen = project.cursors[1]
        local plot = function(x, y)
            if (x >= 0 and x < project.canvas.dim.x and y >= 0 and y < project.canvas.dim.y) then
                project.canvas:layer(cen.layer).scene:set(
                    UC(x, y),
                    BlendMode.NORMAL:blend(col, project.canvas:layer(cen.layer).scene:get(UC(x, y)))
                )
            end
        end

        local x0 = cen.coord.x
        local y0 = cen.coord.y

        local f = 1 - rad
        local ddf_x = 1
        local ddf_y = -2 * rad
        local x = 0
        local y = rad
        plot(x0, y0 + rad)
        plot(x0, y0 - rad)
        plot(x0 + rad, y0)
        plot(x0 - rad, y0)
        while (x < y) do
            if (f >= 0) then
                y = y - 1
                ddf_y = ddf_y + 2
                f = f + ddf_y
            end
            x = x + 1
            ddf_x = ddf_x + 2
            f = f + ddf_x
            plot(x0 + x, y0 + y)
            plot(x0 - x, y0 + y)
            plot(x0 + x, y0 - y)
            plot(x0 - x, y0 - y)
            plot(x0 + y, y0 + x)
            plot(x0 - y, y0 + x)
            plot(x0 + y, y0 - x)
            plot(x0 - y, y0 - x)
        end
    end
}

actions['equip'] = {
    perform = function(self, project, console)
        local input = console:cmdin("id: ")
        if (input == "" or input == nil) then
            return
        end
        project.canvas.palette:equip(tonumber(input))
    end
}

actions['fill'] = {
    -- https://www.geeksforgeeks.org/flood-fill-algorithm-implement-fill-paint
    floodFillUtil = function(self, scene, point, prevC, newC)
        local x = point.x
        local y = point.y
        --Console:cmdin("x: " .. x .. ", y: " .. y .. ", dx: " .. scene.dim.x .. ", dy: " .. scene.dim.y)
        if (x < 0 or x >= scene.dim.x or y < 0 or y >= scene.dim.y) then
            return nil
        end
        local color = scene:get(point)
        if (color.red ~= prevC.red or color.green ~= prevC.green or color.blue ~= prevC.blue or color.alpha ~= prevC.alpha) then
            return nil
        end
        if (color.red == newC.red and color.green == newC.green and color.blue == newC.blue and color.alpha == newC.alpha) then
            return nil
        end
        scene:set(point, newC)

        self:floodFillUtil(scene, UC(x+1, y), prevC, newC)
        if (x > 0) then
            self:floodFillUtil(scene, UC(x-1, y), prevC, newC)
        end
        self:floodFillUtil(scene, UC(x, y+1), prevC, newC)
        if (y > 0) then
            self:floodFillUtil(scene, UC(x, y-1), prevC, newC)
        end
    end,

    floodFill = function(self, scene, point, newC)
        local prevC = scene:get(point)
        if (prevC == newC) then return end
        self:floodFillUtil(scene, point, prevC, newC)
    end,

    perform = function(self, project, console)
        local start = project.cursors[1]
        self:floodFill(
            project.canvas:layer(start.layer).scene,
            start.coord,
            project.canvas.palette:get()
        )
    end
}
