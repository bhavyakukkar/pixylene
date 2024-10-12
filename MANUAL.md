# Reference Manual


## File Management
The filing of the Project so that it may be saved to disk and loaded back, was
fragmented into three different types: the Canvas file, the Project file and the
PNG file. The table shows the 4 ways a new Project may be loaded.

| Manner            | Canvas data       | Project data      | Command-Line (outside pixylene)     | Command (inside pixylene) |
|-------------------|-------------------|-------------------|-------------------------------------|---------------------------|
| New Project       | Empty             | Empty             | `$ pixylene new`                    | `:new`                    |
| Import PNG        | From PNG          | Empty             | `$ pixylene import image.png`       | `:import`                 |
| Load Canvas File  | From Canvas File  | Empty             | `$ pixylene canvas first.toml`      | `:open-canvas` or `:e`    |
| Load Project File | From Project File | From Project File | `$ pixylene project first.pixylene` | `:open-project` or `:ep`  |

The `Canvas` file is a plaintext readable format of the artwork layers & pixels, guaranteed to work across minor versions between major releases with the advantages that you can track it using version-control or collaborate on it.
The `Project` file is a binary format of the session state with no guarantees of working across minor versions (for the time being) with the advnatage of preserving project data including but not limited the Lua actions read at its runtime and the positions of cursors.


## Inputs
There are two ways to communicate with Pixylene, once it has been started:

1. **Commands:** Press ':', type the command, press Enter
2. **Keys:** Press any key or combination of keys besides ':'

Note: ':' is not hardcoded and can be changed by setting `required_keys.start_command` in `config.toml`


### Commands
Commands can be entered from the command-line which is a textual way of communicating with the editor (through built-in commands) as well as the artwork (through the built-in `run-action` command).

The following are the details of all the built-in commands:

| Command           | Alias | Argument (Example) | Description                                                                                                                                                                                               |
|-------------------|-------|--------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| new               | n     | [w] [h]            | Starts a new session with an empty project with an empty canvas of dimensions `w`x`h` (defaulting to `defaults.dimensions.x`x`defaults.dimensions.y` defined in the configuration if present, else 32x32) |
| open-canvas       | e     | mycanvas.toml      | Starts a new session with an empty project with the saved canvas file `mycanvas.toml`                                                                                                                     |
| open-project      | ep    | myproject.pixylene | Starts a new session with the saved project file `myproject.pixylene`                                                                                                                                     |
| import            | imp   | image.png [w h]    | Starts a new session with an empty project with a canvas containing a single layer containing the pixels of `image.png` after being resized to dimensions `w`x`h`                                         |
| quit              | q     | -                  | Quits the editor if no unsaved session is open                                                                                                                                                            |
| force-quit        | q!    | -                  | Quits the editor, closing any unsaved sessions                                                                                                                                                            |
| session           | ses   | i                  | Goes to the open session at the provided index `i`                                                                                                                                                        |
| next-session      | nses  | -                  | Goes to the next session index                                                                                                                                                                            |
| prev-session      | pses  | -                  | Goes to the previous session index                                                                                                                                                                        |
| save-canvas       | w     | [new.toml]         | Saves the current canvas to canvas file `new.toml` or saves an already open canvas or prompts to enter a new file-name; if `new.toml` not specified                                                       |
| save-project      | wp    | [new.pixylene]     | Saves the current project to project file `new.pixylene` or saves an already open project or prompts to enter a new file-name; if `new.pixylene` not specified                                            |
| export            | exp   | [new.png]          | Exports the current canvas with all of its layers blended, into png file `new.png` or prompts to enter a a new file-name if `new.png` not specified                                                       |
| undo              | undo  | -                  | Restores the previous state of the canvas                                                                                                                                                                 |
| redo              | redo  | -                  | Restores the future state of the canvas after an undo                                                                                                                                                     |
| enter-namespace   | ns    | [filters]          | Tries to enter the provided keybinding namespace `filters` or the default namespace if not provided                                                                                                       |
| default-namespace | dns   | -                  | Enters the default namespace                                                                                                                                                                              |
| run-key           | key   | alt-f4             | Simulates pressing the provided single keystroke `Alt+F4`                                                                                                                                                 |
| run-command       | cmd   | undo               | Runs any of the commands specified in this table (except run-command) without any arguments                                                                                                               |
| run-native-action | an    | pencil             | Runs the native action associated to the provided action-name `pencil`                                                                                                                                    |
| run-lua-action    | al    | snowflake          | Runs the lua action associated to the provided action-name `snowflake`                                                                                                                                    |
| run-action        | a     | erase              | Runs the native or Lua action associated to the provided action-name `erase`, giving precendence to the native action if the action exists as both native and Lua                                         |
| run-last-action   | la    | -                  | Runs the last invoked action that modified the canvas                                                                                                                                                     |
| run-lua           | l     | "cmdout('Hey')"    | Run lua code directly using the same API as provided to Lua actions                                                                                                                                       |
| draw-layer        | dl    | -                  | Draws the currently focussed layer of the canvas onto the screen                                                                                                                                          |
| draw-canvas       | dc    | -                  | Draws the overall merged canvas after blending all of its layers onto the screen                                                                                                                          |
| draw-statusline   | ds    | -                  | Draws the statusline onto the screen                                                                                                                                                                      |
| print-canvas-json | pc    | -                  | Prints the JSON serialization of the canvas to stdout (mostly for debug purposes)                                                                                                                         |
| list-commands     | lc    | -                  | Lists & describes all the commands, much like this table                                                                                                                                                  |
| list-keybind-map  | lk    | [filters]          | Lists the currently activated keybinds in the `filters` namespace, or the required keybings, the overlay keybinds, or the keybinds in the default namespace if `filters` not specified                    |
| list-namespaces   | ln    | -                  | List all the active keybind namespaces                                                                                                                                                                    |

### Keys
None of the keys intercepted are ignored, in fact the application operates in a
turn-based manner until the user presses any key, or an event like a window
resize is triggered. Every combination of keys can be configured in the
configuration file to invoke a list of commands in sequence, except for the few reverse mappings that pixylene requires to be known

#### Keybinding Categories
| Category      | Mapping Direction    | Key in `config.toml`       | Description                                                                          |
|---------------|----------------------|----------------------------|--------------------------------------------------------------------------------------|
| Required Keys | Setting to Keystroke | `[required-keys]` (table)  | Key mappings mandatorily required to be known by pixylene                            |
| Overlay Keys  | Keystroke to Command | `[overlay-keys]` (map)     | Key mappings that are always active and invokable                                    |
| Namespaces    | Keystroke to Command | `[keys.<namespace>]` (map) | Key mappings that are only active when its parent namespace is the current namespace |

##### Required Keys
At current, there are only three settings that require a key to be defined as required-keys:
| Setting           | Description                                                            |
|-------------------|------------------------------------------------------------------------|
| `force-quit`      | The key that will force-quit the editor, ignoring any unsaved sessions |
| `start-command`   | The key that will open the command-line to enter commands              |
| `discard-command` | The key that will discard writing a command in the command-line        |

Coincidentally, all these settings are also available as commands so different mappings might be overwritten onto them, but this is not a feature and may not be the case in future versions.

##### Overlay Keys
Overlay keys define the keybindings that will always be active, no matter what namespace is open, and will overwrite any keybindings in the namespace with the same key.
If the opposite behavior is desired, namespace keys can be given a higher precedence than overlay keys with the `namespace-before-overlay` switch in `config.toml`.

##### Namespace Keys
Namespaces enable the user to model their editor similar to moded text editors like vim, except that users can configure their own modes into existence.
They provide a grouping of keybindings that are all active together and help you organize your keybindings by category or function.

###### Default Namespace
The default namespace is the namespace that Pixylene starts in when a new session is started. The default namespace can be named whatever you desire but must be registered using the `default-namespace` field in `config.toml` as shown in the example. The command `default-namespace` can be used at any time to enter the default namespace.

#### Example Configuration
```toml
[required-keys]
force-quit = "alt-f4"
start-command = ":"
discard-command = "esc"

# These keys will be always be active no matter the namespace
[overlay-keys]
ctrl-z = ["undo"] 
ctrl-y = ["redo"]

# The default namespace that pixylene will begin with
default-namespace = "Normal"

[keys.Normal]
v = [{ enter-namespace = "View" }]
s = [{ enter-namespace = "Shapes" }]

# Keys in the 'View' namespace
[keys.View]
i = [{ action = "zoomin" }]
o = [{ action = "zoomout" }]
# Esc to go back to the default namespace
esc = ["default-namespace"]

# Keys in the 'Shapes' namespace
# Here, we make every keybind automatically return back to the default namespace after doing its job instead of an explicit key to go back like in the 'View' namespace
[keys.Shapes]
c = [{ action = "circularfill" }, "default-namespace"]
"*" = [{ action = "star" }, "default-namespace"]
# We can shorten long command-names into their aliases
f = [{ a = "fill" }, "dns"]
v = [{ ns = "view" }]
```
Note: Table keys (left-hand side) in TOML can contain hyphens without the need to wrap them in double quotes. Other symbols however will require double-quotes around, note "*" in the Shapes namespace
