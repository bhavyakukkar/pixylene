import init, { start, tick, OPixelJS } from './pkg/pixyleneweb.js';

var running = true;
var ticker;
async function run() {
    const ui = await init();
    //console.log(ui);
    start();
    tick();
    document.addEventListener('keydown', () => {
        if(running) {
            running = tick();
        } else {
            clearInterval(ticker);
        }
    });
    //ticker = setInterval(, /*30 FPS*/33);
}

run();

//const keySer = (event) => event.key

//function get_key_press() {
//    document.addEventListener('keypress', (e) => console.log("keypress: ", keySer(e)));
//}
//document.addEventListener('keydown', (e) => console.log("keydown: ", e));
//document.addEventListener('keyup', (e) => console.log("keyup: ", e));
//const keyState = {}
//document.addEventListener('keydown', e => {
//    console.log(e.code)
//    keyState[e.code] = true
//})
//document.addEventListener('keyup', e => {
//    keyState[e.code] = false
//})

//setInterval(() => console.log(keyState), 500)
