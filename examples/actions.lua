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
            project.canvas:layer(cen.layer).scene:set(
                UC(x, y),
                BlendMode.NORMAL:blend(project.canvas:layer(cen.layer).scene:get(UC(x, y)), col)
            )
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
