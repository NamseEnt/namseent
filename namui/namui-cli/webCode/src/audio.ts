let audioContext: AudioContext | null = null;

function getAudioContext(): AudioContext {
    if (!audioContext) {
        audioContext = new AudioContext();
    }
    return audioContext;
}

const audioBufferMap = new Map<number, AudioBuffer>();
const playbackMap = new Map<
    number,
    { source: AudioBufferSourceNode; gain: GainNode; panner?: PannerNode }
>();

let gainNode: GainNode | null = null;
let limiterNode: DynamicsCompressorNode | null = null;

function getLimiterNode(): DynamicsCompressorNode {
    if (!limiterNode) {
        const ctx = getAudioContext();
        limiterNode = ctx.createDynamicsCompressor();
        limiterNode.threshold.value = -3;
        limiterNode.knee.value = 0;
        limiterNode.ratio.value = 20;
        limiterNode.attack.value = 0.001;
        limiterNode.release.value = 0.01;
        limiterNode.connect(ctx.destination);
    }
    return limiterNode;
}

function getGainNode(): GainNode {
    if (!gainNode) {
        const ctx = getAudioContext();
        gainNode = ctx.createGain();
        gainNode.connect(getLimiterNode());
    }
    return gainNode;
}

let debugCanvas: HTMLCanvasElement | null = null;
let debugAnimFrame: number | null = null;
let leftAnalyser: AnalyserNode | null = null;
let rightAnalyser: AnalyserNode | null = null;
let debugSplitter: ChannelSplitterNode | null = null;

function startDebugOverlay() {
    if (debugCanvas) return;
    if (typeof document === "undefined") return;

    const ctx = getAudioContext();
    const gn = getGainNode();

    debugSplitter = ctx.createChannelSplitter(2);
    leftAnalyser = ctx.createAnalyser();
    rightAnalyser = ctx.createAnalyser();
    leftAnalyser.fftSize = 256;
    rightAnalyser.fftSize = 256;

    gn.connect(debugSplitter);
    debugSplitter.connect(leftAnalyser, 0);
    debugSplitter.connect(rightAnalyser, 1);

    debugCanvas = document.createElement("canvas");
    debugCanvas.style.position = "fixed";
    debugCanvas.style.top = "0";
    debugCanvas.style.left = "0";
    debugCanvas.style.width = "100%";
    debugCanvas.style.height = "100%";
    debugCanvas.style.pointerEvents = "none";
    debugCanvas.style.zIndex = "99999";
    debugCanvas.width = window.innerWidth;
    debugCanvas.height = window.innerHeight;
    document.body.appendChild(debugCanvas);

    const leftData = new Uint8Array(leftAnalyser.frequencyBinCount);
    const rightData = new Uint8Array(rightAnalyser.frequencyBinCount);

    function drawDebug() {
        if (!debugCanvas || !leftAnalyser || !rightAnalyser) return;

        const w = window.innerWidth;
        const h = window.innerHeight;
        if (debugCanvas.width !== w || debugCanvas.height !== h) {
            debugCanvas.width = w;
            debugCanvas.height = h;
        }

        const canvasCtx = debugCanvas.getContext("2d")!;
        canvasCtx.clearRect(0, 0, w, h);

        leftAnalyser.getByteTimeDomainData(leftData);
        rightAnalyser.getByteTimeDomainData(rightData);

        let leftRms = 0;
        let rightRms = 0;
        for (let i = 0; i < leftData.length; i++) {
            const lv = (leftData[i] - 128) / 128;
            const rv = (rightData[i] - 128) / 128;
            leftRms += lv * lv;
            rightRms += rv * rv;
        }
        leftRms = Math.sqrt(leftRms / leftData.length);
        rightRms = Math.sqrt(rightRms / rightData.length);

        const barWidth = 30;
        const barMaxH = h * 0.6;
        const barY = h * 0.2;

        canvasCtx.globalAlpha = 0.6;

        const leftH = leftRms * barMaxH * 3;
        canvasCtx.fillStyle = "#00cc66";
        canvasCtx.fillRect(10, barY + barMaxH - leftH, barWidth, leftH);

        const rightH = rightRms * barMaxH * 3;
        canvasCtx.fillStyle = "#00cc66";
        canvasCtx.fillRect(w - 10 - barWidth, barY + barMaxH - rightH, barWidth, rightH);

        canvasCtx.globalAlpha = 0.3;
        canvasCtx.strokeStyle = "#ffffff";
        canvasCtx.lineWidth = 1;
        canvasCtx.strokeRect(10, barY, barWidth, barMaxH);
        canvasCtx.strokeRect(w - 10 - barWidth, barY, barWidth, barMaxH);

        canvasCtx.globalAlpha = 1.0;
        canvasCtx.font = "14px monospace";

        const leftDb = leftRms > 0 ? 20 * Math.log10(leftRms) : -Infinity;
        const rightDb = rightRms > 0 ? 20 * Math.log10(rightRms) : -Infinity;

        const leftDbStr = "L: " + (isFinite(leftDb) ? leftDb.toFixed(1) + " dB" : "-∞ dB");
        const rightDbStr = "R: " + (isFinite(rightDb) ? rightDb.toFixed(1) + " dB" : "-∞ dB");

        const lm = canvasCtx.measureText(leftDbStr);
        canvasCtx.fillStyle = "rgba(0,0,0,0.7)";
        canvasCtx.fillRect(8, barY + barMaxH + 18, lm.width + 8, 20);
        canvasCtx.fillStyle = "#ffffff";
        canvasCtx.fillText(leftDbStr, 12, barY + barMaxH + 33);

        const rm = canvasCtx.measureText(rightDbStr);
        canvasCtx.fillStyle = "rgba(0,0,0,0.7)";
        canvasCtx.fillRect(w - 12 - rm.width - 8, barY + barMaxH + 18, rm.width + 8, 20);
        canvasCtx.fillStyle = "#ffffff";
        canvasCtx.textAlign = "right";
        canvasCtx.fillText(rightDbStr, w - 12, barY + barMaxH + 33);
        canvasCtx.textAlign = "left";

        debugAnimFrame = requestAnimationFrame(drawDebug);
    }

    drawDebug();
}

function stopDebugOverlay() {
    if (debugAnimFrame !== null) {
        cancelAnimationFrame(debugAnimFrame);
        debugAnimFrame = null;
    }
    if (debugCanvas) {
        debugCanvas.remove();
        debugCanvas = null;
    }
    if (debugSplitter) {
        debugSplitter.disconnect();
        debugSplitter = null;
    }
    if (leftAnalyser) {
        leftAnalyser.disconnect();
        leftAnalyser = null;
    }
    if (rightAnalyser) {
        rightAnalyser.disconnect();
        rightAnalyser = null;
    }
}

export async function loadAudioAssets(
    list: { id: number; path: string }[],
) {
    const ctx = getAudioContext();
    await Promise.all(
        list.map(async ({ id, path }) => {
            const response = await fetch(path);
            if (!response.ok) {
                throw new Error(
                    `Failed to fetch audio ${id} from ${path}: ${response.statusText}`,
                );
            }
            const arrayBuffer = await response.arrayBuffer();
            const buffer = await ctx.decodeAudioData(arrayBuffer);
            audioBufferMap.set(id, buffer);
        }),
    );
}

export function createAudioImports({}: { memory: WebAssembly.Memory }) {
    return {
        _audio_play(audioId: number, playbackId: number, repeat: boolean) {
            const buffer = audioBufferMap.get(audioId);
            if (buffer) {
                playAudioBuffer(buffer, playbackId, repeat);
            }
        },

        _audio_play_spatial(
            audioId: number,
            playbackId: number,
            repeat: boolean,
        ) {
            const buffer = audioBufferMap.get(audioId);
            if (buffer) {
                playAudioBufferSpatial(buffer, playbackId, repeat);
            }
        },

        _audio_playback_drop(playbackId: number) {
            const entry = playbackMap.get(playbackId);
            if (entry) {
                const ctx = getAudioContext();
                const now = ctx.currentTime;
                entry.gain.gain.cancelScheduledValues(now);
                entry.gain.gain.setValueAtTime(entry.gain.gain.value, now);
                entry.gain.gain.linearRampToValueAtTime(0, now + 0.01);
                setTimeout(() => {
                    entry.source.stop();
                    entry.source.disconnect();
                    entry.gain.disconnect();
                    if (entry.panner) {
                        entry.panner.disconnect();
                    }
                }, 15);
                playbackMap.delete(playbackId);
            }
        },

        _audio_playback_set_volume(playbackId: number, volume: number) {
            const entry = playbackMap.get(playbackId);
            if (entry) {
                const ctx = getAudioContext();
                entry.gain.gain.setTargetAtTime(volume, ctx.currentTime, 0.005);
            }
        },

        _audio_playback_set_position(
            playbackId: number,
            x: number,
            y: number,
            z: number,
        ) {
            const entry = playbackMap.get(playbackId);
            if (entry?.panner) {
                entry.panner.positionX.value = x;
                entry.panner.positionY.value = y;
                entry.panner.positionZ.value = z;
            }
        },

        _audio_set_listener_position(x: number, y: number, z: number) {
            const ctx = getAudioContext();
            const listener = ctx.listener;
            if (listener.positionX !== undefined) {
                listener.positionX.value = x;
                listener.positionY.value = y;
                listener.positionZ.value = z;
                listener.forwardX.value = 0;
                listener.forwardY.value = 0;
                listener.forwardZ.value = 1;
                listener.upX.value = 0;
                listener.upY.value = -1;
                listener.upZ.value = 0;
            } else {
                listener.setPosition(x, y, z);
                listener.setOrientation(0, 0, 1, 0, -1, 0);
            }
        },

        _audio_set_volume(volume: number) {
            getGainNode().gain.value = volume;
        },
    };
}

function playAudioBuffer(
    buffer: AudioBuffer,
    playbackId: number,
    loop: boolean,
) {
    const ctx = getAudioContext();
    const source = ctx.createBufferSource();
    const gain = ctx.createGain();
    source.buffer = buffer;
    source.loop = loop;
    source.connect(gain);
    gain.connect(getGainNode());
    gain.gain.value = 0;
    source.onended = () => {
        playbackMap.delete(playbackId);
        source.disconnect();
        gain.disconnect();
    };
    source.start();
    playbackMap.set(playbackId, { source, gain });
}

function playAudioBufferSpatial(
    buffer: AudioBuffer,
    playbackId: number,
    loop: boolean,
) {
    const ctx = getAudioContext();
    const source = ctx.createBufferSource();
    const gain = ctx.createGain();
    const panner = ctx.createPanner();

    panner.panningModel = "HRTF";
    panner.distanceModel = "inverse";
    panner.refDistance = 100;
    panner.maxDistance = 10000;
    panner.rolloffFactor = 1;

    source.buffer = buffer;
    source.loop = loop;
    source.connect(gain);
    gain.connect(panner);
    panner.connect(getGainNode());
    gain.gain.value = 0;

    source.onended = () => {
        playbackMap.delete(playbackId);
        source.disconnect();
        gain.disconnect();
        panner.disconnect();
    };
    source.start();
    playbackMap.set(playbackId, { source, gain, panner });
}

const AUDIO_DEBUG_KEY = "namui_audio_debug";

let audioDebugEnabled = false;

function setAudioDebug(enabled: boolean) {
    audioDebugEnabled = enabled;
    try { localStorage.setItem(AUDIO_DEBUG_KEY, enabled ? "1" : "0"); } catch {}
    if (enabled) {
        startDebugOverlay();
    } else {
        stopDebugOverlay();
    }
}

(globalThis as any).audioDebug = (enabled?: boolean) => {
    if (enabled === undefined) {
        return audioDebugEnabled;
    }
    setAudioDebug(enabled);
    return enabled;
};

try {
    if (localStorage.getItem(AUDIO_DEBUG_KEY) === "1") {
        setAudioDebug(true);
    }
} catch {
}
