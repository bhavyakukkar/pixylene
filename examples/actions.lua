actions['greet'] = {
    perform = function(self, project, console)
        console:cmdout("haii " .. console:cmdin("your name? "), LogType.INFO)
    end
}

actions['zoomin'] = {
    perform = function(self, project, console)
        project:set_mul(project.mul + 1)
        console:cmdout("zoomed in", LogType.INFO)
    end
}

actions['zoomout'] = {
    perform = function(self, project, console)
        project:set_mul(project.mul - 1)
        console:cmdout("zoomed out", LogType.INFO)
    end
}

-- Duplicates the Current Layer
actions['dupcurlayer'] = {
    perform = function(self, project, console)
        local canvas = project.canvas
        local layer = canvas:get(project:focus().layer)
        canvas:add(layer)
        project:focus(project:focus().coord, project:focus().layer + 1)
        project.canvas = canvas
    end
}
