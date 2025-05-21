import { sendMessageToMainThread } from "./interWorkerProtocol";

export function audioImports({ memory }: { memory: WebAssembly.Memory }) {
    return {
        _audio_init(audioId: number, bufferPtr: number, bufferLen: number) {
            const buffer = new Uint8Array(bufferLen);
            buffer.set(new Uint8Array(memory.buffer, bufferPtr, bufferLen));

            sendMessageToMainThread(
                {
                    type: "audio-init",
                    audioId,
                    buffer: buffer.buffer,
                },
                [buffer.buffer],
            );
        },
        _audio_drop(audioId: number) {
            sendMessageToMainThread({
                type: "audio-drop",
                audioId,
            });
        },
        _audio_play(audioId: number, playbackId: number, repeat: boolean) {
            sendMessageToMainThread({
                type: "audio-play",
                audioId,
                playbackId,
                repeat,
            });
        },
        _audio_play_and_forget(audioId: number) {
            sendMessageToMainThread({
                type: "audio-play_and_forget",
                audioId,
            });
        },
        _audio_playback_drop(playbackId: number) {
            sendMessageToMainThread({
                type: "audio-playback_drop",
                playbackId,
            });
        },
        _audio_context_volume_set(
            volume: number,
            requestSequenceNumber: number,
        ) {
            sendMessageToMainThread({
                type: "audio-context-volume-set",
                volume,
                requestSequenceNumber,
            });
        },
    };
}

export function audioHandleOnMainThread() {
    const audioContext = new AudioContext();
    const gainNode = audioContext.createGain();
    gainNode.gain.value = 1;
    gainNode.connect(audioContext.destination);

    const audioBufferMap = new Map<number, AudioBuffer>();
    const playbackMap = new Map<number, AudioBufferSourceNode>();

    let lastRequestSequenceNumber = -1;

    /**
     * NOTE: Several timing of message arrival issue can be happened here.
     * For example, when the audio is played and the audio is dropped immediately,
     * maybe the messages are not arrived in the correct order because each messages
     * can be sent to the main thread from separated worker threads.
     * One of the trial is `requestSequenceNumber` at `audioContextVolumeSet` function.
     */

    return {
        audioInit({
            audioId,
            buffer,
        }: {
            audioId: number;
            buffer: ArrayBuffer;
        }) {
            const f32View = new Float32Array(buffer);
            const audioBuffer = new AudioBuffer({
                length: f32View.length / 2,
                numberOfChannels: 2,
                sampleRate: 48000,
            });
            audioBuffer.copyToChannel(
                f32View.subarray(0, f32View.length / 2),
                0,
            );
            audioBuffer.copyToChannel(f32View.subarray(f32View.length / 2), 1);
            audioBufferMap.set(audioId, audioBuffer);
        },
        audioDrop({ audioId }: { audioId: number }) {
            if (!audioBufferMap.delete(audioId)) {
                throw new Error(`Audio ${audioId} not found`);
            }
        },
        audioPlay({
            audioId,
            playbackId,
            repeat,
        }: {
            audioId: number;
            playbackId: number;
            repeat: boolean;
        }) {
            const audioBuffer = audioBufferMap.get(audioId);
            if (!audioBuffer) {
                throw new Error(`Audio ${audioId} not found`);
            }
            const audioSource = new AudioBufferSourceNode(audioContext, {
                buffer: audioBuffer,
                loop: repeat,
            });
            audioSource.connect(gainNode);
            audioSource.start();
            playbackMap.set(playbackId, audioSource);
        },
        audioPlayAndForget({ audioId }: { audioId: number }) {
            const audioBuffer = audioBufferMap.get(audioId);
            if (!audioBuffer) {
                throw new Error(`Audio ${audioId} not found`);
            }
            const audioSource = new AudioBufferSourceNode(audioContext, {
                buffer: audioBuffer,
            });
            audioSource.connect(gainNode);
            audioSource.start();
        },
        audioPlaybackDrop({ playbackId }: { playbackId: number }) {
            const audioSource = playbackMap.get(playbackId);
            if (!audioSource) {
                throw new Error(`Playback ${playbackId} not found`);
            }
            audioSource.stop();
            playbackMap.delete(playbackId);
        },
        audioContextVolumeSet({
            volume,
            requestSequenceNumber,
        }: {
            volume: number;
            requestSequenceNumber: number;
        }) {
            if (requestSequenceNumber <= lastRequestSequenceNumber) {
                return;
            }
            lastRequestSequenceNumber = requestSequenceNumber;
            gainNode.gain.value = volume;
        },
    };
}
