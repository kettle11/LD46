export function loadImage(src) {
    return new Promise((resolve, reject) => {
        const img = new Image();
        img.addEventListener("load", () => resolve(img));
        img.addEventListener("error", err => reject(err));
        img.src = src;
    });
}

export function download(filename, text) {
    var element = document.createElement('a');
    element.setAttribute('href', 'data:text/plain;charset=utf-8,' + encodeURIComponent(text));
    element.setAttribute('download', filename);

    element.style.display = 'none';
    document.body.appendChild(element);

    element.click();

    document.body.removeChild(element);
}

var audio_context = null;

export function setup() {
    if (audio_context == null) {
        window.AudioContext = window.AudioContext || window.webkitAudioContext;
        // Fix up for prefixing
        audio_context = new AudioContext();
    } else {
        audio_context.resume();
    }
}
export function loadAudio(src) {

    return new Promise((resolve, reject) => {
        var request = new XMLHttpRequest();
        request.open('GET', src, true);
        request.responseType = 'arraybuffer';

        console.log("HERE");

        request.addEventListener("load", () => {
            var buffer = null;
            audio_context.decodeAudioData(request.response, function (buffer) {
                console.log("SOUND EFFECT LOADED");
                resolve(buffer)
            }, () => {
                console.log("FAILED TO DECODE SOUND EFFECT");
                reject("Could not decode");
            });
        });
        request.addEventListener("error", err => {
            console.log("ERROR");
            reject(err);
        });
        request.send();
    });
}

export function playAudio(buffer, rate, gain) {
    var source = audio_context.createBufferSource();
    var g = audio_context.createGain();
    source.buffer = buffer;
    source.playbackRate.value = rate;
    source.start(0);
    g.gain.value = gain;
    source.connect(g);
    g.connect(audio_context.destination);
}

var ball_audio_gain = null;
var ball_audio_source = null;

export function playBallAudio(buffer, rate, gain) {
    var source = audio_context.createBufferSource();
    var g = audio_context.createGain();
    source.buffer = buffer;
    source.playbackRate.value = rate;
    source.start(0);
    g.gain.value = gain;
    source.connect(g);
    g.connect(audio_context.destination);
    ball_audio_gain = g;
    ball_audio_source = source;
    source.loop = true;
}

export function ballAudio(gain, rate) {
    ball_audio_gain.gain.value = gain;
    ball_audio_source.playbackRate.value = rate;
}