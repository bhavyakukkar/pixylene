-- Greets the user (only once) after asking for their name
actions['greet'] = {
    count = 0,
    perform = function(self, project, console)
        if self.count == 0 then
            local name = console:cmdin("Your Name? ")
            if (name == nil or name == '') then
                name = "there"
            end
            console:cmdout("Hello, " .. name .. ".", LogType.INFO)
            self.count = self.count + 1
        else
            console:cmdout("I only greet once.", LogType.WARNING)
        end
    end
}

-- Zooms in, i.e., increments the project multiplier by 1
actions['zoomin'] = {
    perform = function(self, project, console)
        project:set_mul(project.mul + 1)
        console:cmdout("zoomed in", LogType.INFO)
    end
}

-- Zooms out, i.e., decrements the project multiplier by 1
actions['zoomout'] = {
    perform = function(self, project, console)
        project:set_mul(project.mul - 1)
        console:cmdout("zoomed out", LogType.INFO)
    end
}

-- Duplicates the Current Layer & moves focus to it
actions['dupcurlayer'] = {
    perform = function(self, project, console)
        project.canvas:add(project.canvas:layer(project:focus().layer))
        project:focus(project:focus().coord, project:focus().layer + 1)
    end
}

actions['noise'] = {
    perform = function(self, project, console)
        local f = tonumber(console:cmdin("Noise Factor (0.00 - 1.00): "))
        local d = project.canvas.dim
        for i=0, (d.x - 1) do
            for j=0, (d.y - 1) do
                if (math.random() < f) then
                    local c = project.canvas:layer(project:focus().layer).scene:get(UC(i, j))
                    project.canvas:layer(project:focus().layer).scene:set(
                        UC(i, j), 
                        BlendMode.NORMAL:blend(P.hex("#ffffff66"), c)
                    )
                end
            end
        end
    end
}
