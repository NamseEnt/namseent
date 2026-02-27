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

function getGainNode(): GainNode {
    if (!gainNode) {
        const ctx = getAudioContext();
        gainNode = ctx.createGain();
        gainNode.connect(ctx.destination);
    }
    return gainNode;
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
                entry.source.stop();
                entry.source.disconnect();
                entry.gain.disconnect();
                if (entry.panner) {
                    entry.panner.disconnect();
                }
                playbackMap.delete(playbackId);
            }
        },

        _audio_playback_set_volume(playbackId: number, volume: number) {
            const entry = playbackMap.get(playbackId);
            if (entry) {
                entry.gain.gain.value = volume;
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
            } else {
                listener.setPosition(x, y, z);
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

    source.onended = () => {
        playbackMap.delete(playbackId);
        source.disconnect();
        gain.disconnect();
        panner.disconnect();
    };
    source.start();
    playbackMap.set(playbackId, { source, gain, panner });
}
