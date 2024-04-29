# Pixylene Manual

## Lua
- The Lua state when performing an action has access to the Lua standard libraries
- All actions are entries in the global `actions` table
- Any action 'foo' when called, is invoked by calling `actions['foo']:perform(Project, Console)`
- Any action can invoke any other action known to exist by invoking it in the same manner as it is invoked internally, shown above, replacing 'foo' with the desired action's name
