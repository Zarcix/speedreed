
import init, { SpeedReed } from "./pkg/speedreed.js";

async function run() {
    await init();
    return new SpeedReed();
}


// Initial Getters
var speedreed = run();
var display = document.getElementById("changing-text")
var input_text = document.getElementById("current-text")
var speed_text = document.getElementById("current-speed")

// Functions
function handle_text_change() {
    let input_text = this.value;
    speedreed.then((reader) => {
        reader.set_current_text(input_text)
        console.debug("New Text: " + reader.get_current_text())
    });
}

function handle_speed_change() {
    let speed = this.value;
    speedreed.then((reader) => {
        reader.set_current_speed(speed)
        console.debug("New Speed: " + reader.get_current_speed())
    });
}

// Modifications
input_text.addEventListener("input", handle_text_change, false)
speed_text.addEventListener("input", handle_speed_change, false)
