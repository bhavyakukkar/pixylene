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
        if (project.focus.layer < project.canvas.len - 1) then
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
        project.mul = project.mul + 1
        console:cmdout("zoomed in")
    end
}

-- zooms out, i.e., decrements the project multiplier by 1
actions['zoomout'] = {
    perform = function(self, project, console)
        if (project.mul > 1) then
            project.mul = project.mul - 1
            console:cmdout("zoomed out")
        end
    end
}

-- duplicates the Current Layer & moves focus to it
actions['dupcurlayer'] = {
    perform = function(self, project, console)
        if project.canvas.indexed then
            layers = project.canvas.layers.indexed
            layers:add(layers:get(project.focus.layer))
        else
            layers = project.canvas.layers['true']
            layers:add(layers:get(project.focus.layer))
        end
        project.focus = { ['coord'] = project.focus.coord, ['layer'] = project.focus.layer + 1 }
    end
}

actions['noise'] = {
    perform = function(self, project, console)
        if project.canvas.indexed then
            console:cmdout("sorry, this action only works on true-color canvases", LogType.ERROR)
            return
        end

        local f = tonumber(console:cmdin("Noise Factor (0.00 - 1.00): "))
        if f == nil then return end

        local d = project.canvas.dim
        for i=0, (d.x - 1) do
            for j=0, (d.y - 1) do
                if (math.random() < f) then
                    local layer = project.canvas.layers['true']:get(project.focus.layer)
                    local c = layer.scene:get(UC(i, j))
                    layer.scene:set(UC(i, j), BlendMode.NORMAL:blend(TP.hex("#ffffff66"), c))
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

        local indexed = project.canvas.indexed
        local cen = project.cursors[1]

        local col = indexed
            and IP(project.canvas.palette.equipped)
            or  project.canvas.palette:get()
        local layer = indexed
            and project.canvas.layers['indexed']:get(cen.layer)
            or  project.canvas.layers['true']:get(cen.layer)

        local plot = function(x, y)
            if (x >= 0 and x < project.canvas.dim.x and y >= 0 and y < project.canvas.dim.y) then
                layer.scene:set(UC(x, y), indexed
                    and col
                    or BlendMode.NORMAL:blend(col, layer.scene:get(UC(x, y))))
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
        project.canvas.palette.equipped = tonumber(input)
    end
}

actions['fill'] = {
    -- https://www.geeksforgeeks.org/flood-fill-algorithm-implement-fill-paint
    equal = function(c1, c2)
        if c1.red ~= nil then
            return c1.red == c2.red and c1.green == c2.green and c1.blue == c2.blue and c1.alpha == c2.alpha
        else
            return c1.index == c2.index
        end
    end,

    floodFillUtil = function(self, scene, point, prevC, newC)
        local x = point.x
        local y = point.y
        --Console:cmdin("x: " .. x .. ", y: " .. y .. ", dx: " .. scene.dim.x .. ", dy: " .. scene.dim.y)
        if (x < 0 or x >= scene.dim.x or y < 0 or y >= scene.dim.y) then
            return nil
        end
        local color = scene:get(point)
        if not self.equal(color, prevC) then
        --if (color.red ~= prevC.red or color.green ~= prevC.green or color.blue ~= prevC.blue or color.alpha ~= prevC.alpha) then
            return nil
        end
        if self.equal(color, newC) then
        --if (color.red == newC.red and color.green == newC.green and color.blue == newC.blue and color.alpha == newC.alpha) then
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
        if project.canvas.indexed then
            self:floodFill(
                project.canvas.layers['indexed']:get(start.layer).scene,
                start.coord,
                IP(project.canvas.palette.equipped)
            )
        else
            self:floodFill(
                project.canvas.layers['true']:get(start.layer).scene,
                start.coord,
                project.canvas.palette:get()
            )
        end
    end
}

actions['grayscale'] = {
    grayScale = function(col)
        avg = (col.red + col.green + col.blue)/3
        return TP(avg, avg, avg, 255)
    end,

    perform = function(self, Project, Console)
        if Project.canvas.indexed then
            Console:cmdout("sorry, this action only works on true-color canvases", LogType.ERROR)
            return
        end

        input = Console:cmdin("1: All cursors, 2: Focussed scene > ")

        if tonumber(input) == 1 then
            -- Iterate over all Project cursors (which may be across different layers)
            for _, cursor in pairs(Project.cursors) do
                -- Convert to grayscale and replace
                scene = Project.canvas.layers['true']:get(cursor.layer).scene
                scene:set(cursor.coord, self.grayScale(scene:get(cursor.coord)))
            end

        elseif tonumber(input) == 2 then
            scene = Project.canvas.layers['true']:get(Project.focus.layer).scene

            -- Iterate over all Scene rows
            for i=0, (scene.dim.x - 1) do
                -- Iterate over all row cells
                for j=0, (scene.dim.y - 1) do
                    -- Convert to grayscale and replace
                    scene:set(UC(i,j), self.grayScale(scene:get(UC(i,j))))
                end
            end
        end
    end
}

--deprecated until TrueLayers.from and IndexedLayers.from
--[[actions['crop'] = {
    start = nil,
    perform = function(self, project, console)
        local ncurs = project.num_cursors
        if (self.start == nil) then
            if (ncurs == 1) then
                self.start = project.cursors[1]
                console:cmdout("First corner selected, go to 2nd corner and run crop again")
            elseif (ncurs > 1) then
                local dim = project.canvas.dim
                local found
                local topleft
                local bottomright

                for i=0, (dim.x - 1) do
                    found = false
                    for j=0, (dim.y - 1) do
                        if (project:is_cursor_at(UC(i, j), project.focus.layer)) then
                            topleft = UC(i, j)
                            found = true
                            break
                        end
                    end
                    if (found) then
                        break
                    end
                end

                for i=(dim.x - 1), 0, -1 do
                    found = false
                    for j=(dim.y - 1), 0, -1 do
                        if (project:is_cursor_at(UC(i, j), project.focus.layer)) then
                            bottomright = UC(i, j)
                            found = true
                            break
                        end
                    end
                    if (found) then
                        break
                    end
                end

                local newdim = PC(
                    (bottomright.x - topleft.x) + 1,
                    (bottomright.y - topleft.y) + 1
                )

                local layers = {}
                for k=1, project.canvas.len do
                    layers[k] = project.canvas.indexed
                        and project.canvas.layers['indexed']:get(k)
                        or  project.canvas.layers['true']:get(k)
                end
                for k=1, project.canvas.len do
                    local newgrid = {}
                    for i=topleft.x, bottomright.x do
                        for j=topleft.y, bottomright.y do
                            table.insert(newgrid, layers[k].scene:get(UC(i,j)))
                        end
                    end
                    layers[k].scene = Scene(newdim, newgrid)
                end
                project.canvas = project.canvas.indexed
                    and Canvas.indexed(newdim, layers, project.canvas.palette)
                    or  Canvas.true(newdim, layers, project.canvas.palette)
            else
                console:cmdout("Need at least 1 cursor for context")
            end
        else
            if (ncurs == 1) then
                
                self.start = nil
            else
                console:cmdout("Need exactly 1 cursor to select 2nd corner")
            end
        end
    end
}--]]
