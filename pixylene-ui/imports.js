
export function name() {
    return 'Rust';
}

export class PixyleneWebJS {
    constructor() {
        window.pixylene = this
        /*this.keyState = {
            Digit0: false, Digit1: false, Digit2: false, Digit3: false, Digit4: false,
            Digit5: false, Digit6: false, Digit7: false, Digit8: false, Digit9: false,

            F1: false, F2: false, F3: false, F4: false, F5: false, F6: false,
            F7: false, F8: false, F9: false, F10: false, F11: false, F12: false,

            KeyA: false, KeyB: false, KeyC: false, KeyD: false, KeyE: false, KeyF: false,
            KeyG: false, KeyH: false, KeyI: false, KeyJ: false, KeyK: false, KeyL: false,
            KeyM: false, KeyN: false, KeyO: false, KeyP: false, KeyQ: false, KeyR: false,
            KeyS: false, KeyT: false, KeyU: false, KeyV: false, KeyW: false, KeyX: false,
            KeyY: false, KeyZ: false,

            ShiftLeft: false, ControlLeft: false, AltLeft: false,
            ShiftRight: false, ControlRight: false, AltRight: false,

            Backquote: false, Minus: false, Equal: false, Backspace: false,
            Tab: false, BracketLeft: false, BracketRight: false, Backslash: false,
            CapsLock: false, Semicolon: false, Quote: false, Enter: false,
            Comma: false, Period: false, Slash: false, 
            ArrowUp: false, ArrowLeft: false, ArrowDown: false, ArrowRight: false
        }*/
        document.addEventListener('keydown', e => {
            //this.keyState[e.code] = true
            this.key = e.key
        })
        document.addEventListener('keyup', e => {
            //this.keyState[e.code] = false
            this.key = null
        })
    }

    initialize() {
        console.log("initialize");
    }
    finalize() {
        console.log("finalized");
        window.close();
    }
    refresh() {
        return true;
    }
    get_key() {
        console.log("get_key");
        return this.key;
    }
    get_size() {
        console.log("get_size")
        let camera = document.getElementById("camera");
        return [camera.height/10 + 3, camera.width/10];
    }
    draw_camera(
        dim_x,
        dim_y,
        buffer,
        show_cursors,
        boundary,
    ) {
        console.log("draw_camera", dim_x, dim_y, buffer, show_cursors, boundary);
        const ctx = document.getElementById("camera").getContext("2d");
        for(let i = 0; i < dim_x; i++) {
            for(let j = 0; j < dim_y; j++) {
                let pixel = buffer[i*dim_y + j];
                ctx.fillStyle =
                    //`rgba(${pixel.color_r} ${pixel.color_g} ${pixel.color_b} ${pixel.color_a})`;
                    `rgb(${pixel.color_r} ${pixel.color_g} ${pixel.color_b})`;
                ctx.fillRect(j*10, i*10, (j+1)*10, (i+1)*10);

                if (pixel.has_cursor) {
                    ctx.beginPath();
                    ctx.arc(j*10 + 5, i*10 + 5, 3, 0, 2 * Math.PI);
                    ctx.fillStyle = "white";
                    ctx.fill();
                    //ctx.lineWidth = 4;
                    //ctx.strokeStyle = "white";
                    //ctx.stroke();
                }
            }
        }
    }
    draw_paragraph(
        paragraph,
        boundary,
    ) {
        console.log("draw_paragraph", paragraph, boundary)
        const statusline = document.getElementById("statusline");
        statusline.innerText = paragraph;
    }
    console_in(
        message,
        discard_key,
        boundary
    ) {
        console.log("console_in", message, discard_key, boundary)
        this.key = null;
        return prompt(message);
    }
    clear(boundary) {
        console.log("clear", boundary)
    }
    clear_all() {
        console.log("clear_all")
    }
}
