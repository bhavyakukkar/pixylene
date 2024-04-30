import pytest
import json
import numpy
from subprocess import Popen, PIPE
import traceback

class Start:
    new = "new"
    def canvas(name):
        return ['canvas', name]
    def project(name):
        return ['project', name]
    def _import(name):
        return ['import', name]

class Cmd:
    def new(x, y):
        return f'new {x} {y}'
    canvas = 'pc'
    quit = 'q'
    force_quit = 'q!'
    def action(name):
        return f'a "{name}"'
    
class Msg:
    canvas_modified = "canvas has been modified since last change, force quit (:q!) to discard modifications\n"
    splash = """

    ____  _ ___  ____  _ _     _____ _      _____
   /  __\\/ \\\\  \\//\\  \\/// \\   /  __// \\  /|/  __/
   |  \\/|| | \\  /  \\  / | |   |  \\  | |\\ |||  \\  
   |  __/| | /  \\  / /  | |_/\\|  /_ | | \\|||  /_ 
   \\_/   \\_//__/\\\\/_/   \\____/\\____\\\\_/  \\|\\____\\


 Welcome to
 Pixylene,
 the extensible Pixel Art Editor


 type  :new 16 16     - to create a new 16x16 canvas
 type  :pk            - to print the current keybindings
 type  :q             - to quit

"""
    def remove_splash(out):
        assert out[0:len(Msg.splash)] == Msg.splash
        return out[len(Msg.splash):]

def run(commands, args = ["new"]):
    process = Popen(['target/debug/pixylenecli', *args], stdin=PIPE, stdout=PIPE, stderr=PIPE, text=True)
    out = process.communicate(input="".join(map(lambda s: s + "\n", commands)))[0]
    return out

class Test_Cli:
    def test_command_aliases(self):
        #over here
        pass

    def test_start_and_quit(self):
        args = [Start.new]
        out = run([Cmd.quit], args)
        assert out == ""
    
    def test_start_and_quit_canvas_modified(self):
        args = [Start.new]
        out = run([Cmd.action("pencil1"), Cmd.quit, Cmd.force_quit], args)
        assert out == Msg.canvas_modified
    
    def test_start_new_dims(self):
        for i in range(1, 64, 4):
            for j in range(1, 64, 4):
                out = run([Cmd.new(i, j), Cmd.canvas, Cmd.quit], [])
                canvas = json.loads(Msg.remove_splash(out))
                assert len(canvas['layers']) == 1
                assert canvas['layers'][0]['scene']['dim']['x'] == j
                assert canvas['layers'][0]['scene']['dim']['y'] == i
                assert len(canvas['layers'][0]['scene']['grid']) == i*j
    
    def test_cursors(self):
        args = Start._import('assets/images/mushroom.png')
        cu, cd, cl, cr = "cursors_up", "cursors_down", "cursors_left", "cursors_right"
        cdu, cdd, cdl, cdr = "cursors_dup_up", "cursors_dup_down", "cursors_dup_left", "cursors_dup_right"
        p1 = "pencil1"
        out = run([
            Cmd.action(cu),
            Cmd.action(cdu), Cmd.action(cdl), Cmd.action(cdd), Cmd.action(cdr),
            Cmd.action(p1), Cmd.canvas, Cmd.quit,
        ], args)
        canvas = json.loads(out)
