#!/usr/bin/env python3

# uses ./truecolor2palette256.py to print truecolor canvas to terminal

import json
import numpy
from subprocess import Popen, PIPE
from truecolor2palette256 import rgb2short, rgb2short_legacy
import traceback

def action(name):
    return f'a "{name}"'
def canvas():
    return f'pc'
def quit():
    return f'q!'

commands = [
    action("cursors_up"),           #move cursors up by 1

    action("cursors_dup_up"),       #increase number of cursors
    action("cursors_dup_left"),
    action("cursors_dup_down"),
    action("cursors_dup_right"),

    action("pencil1"),              #draw with pencil1 at focus

    #action("noise"),               #draw noise with a factor of 0.3 
    #"0.3",                         # (requires lua action 'noise' from
                                    # pixylene-ui/src/std-actions.lua present in
                                    # config at XDG_CONFIG_DIR/pixylene/actions.lua)

    canvas(),                       #print canvas json

    quit(),                         #quit pixylene session
]

def draw(canvas):
    if "True" in canvas['layers']:
        scene = canvas['layers']['True']['layers'][0]['scene']
        color = lambda pixel: rgb2short(f"{(hex(pixel['r']) + '00')[2:4]}{(hex(pixel['g']) + '00')[2:4]}{(hex(pixel['b']) + '00')[2:4]}")[0]
    else:
        scene = canvas['layers']['Indexed']['layers'][0]['scene']
        color = lambda pixel: pixel
    grid = numpy.reshape(scene['grid'], (scene['dim']['x'], scene['dim']['y']))
    for row in grid:
        for pixel in row:
            if pixel != None:
                print(f"\x1b[48;5;{color(pixel)}m  \x1b[0m", end="")
            else:
                print("  ", end="")
        print()

def run(commands, png):
    process = Popen(['target/debug/pixylenecli', 'import', png], stdin=PIPE, stdout=PIPE, stderr=PIPE, text=True)
    out = process.communicate(input="".join(map(lambda s: s + "\n", commands)))[0]
    try:
        canvas = json.loads(out)
        draw(canvas)
    except Exception as e:
        print(traceback.format_exc())
        print(out)
    #draw(print(json.loads()))

print()
run(commands, 'assets/images/rgb_8bit_16x16.png')
run(commands, 'assets/images/indexed_8bit_33x33.png')
print()
