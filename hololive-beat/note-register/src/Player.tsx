import { useEffect, useRef, useState } from "react";
import { Note, instruments } from "./types";

type NoteAndAudioBuffers = {
    kick: { note: Note; audioBuffer: AudioBuffer }[];
    snare: { note: Note; audioBuffer: AudioBuffer }[];
    cymbals: { note: Note; audioBuffer: AudioBuffer }[];
};

export function Player({ notes }: { notes: Note[] }) {
    const [state, setState] = useState<
        | {
              type: "stop";
          }
        | {
              type: "play";
              startTimeMs: number;
          }
    >({
        type: "stop",
    });
    const [fullAudioBuffers, setFullAudioBuffers] = useState<{
        kick: AudioBuffer;
        snare: AudioBuffer;
        cymbals: AudioBuffer;
        others: AudioBuffer;
    }>();
    const audioContextRef = useRef<AudioContext>(new AudioContext());
    const [noteAndAudioBuffers, setNoteAndAudioBuffers] =
        useState<NoteAndAudioBuffers>();
    const canvasRef = useRef<HTMLCanvasElement>(null);

    useEffect(() => {
        const audioContext = audioContextRef.current;
        return () => {
            audioContext.close();
        };
    }, []);

    useEffect(() => {
        (async () => {
            const [kick, snare, cymbals, others] = (await Promise.all(
                [...instruments, "others"].map(async (part) => {
                    const res = await fetch(`/${part}.opus`);
                    const arrayBuffer = await res.arrayBuffer();
                    return await audioContextRef.current.decodeAudioData(
                        arrayBuffer,
                    );
                }),
            )) as [AudioBuffer, AudioBuffer, AudioBuffer, AudioBuffer];

            setFullAudioBuffers({
                kick,
                snare,
                cymbals,
                others,
            });
        })();
    }, []);

    useEffect(() => {
        if (state.type === "stop" || !fullAudioBuffers) {
            return;
        }

        const noteAndAudioBuffers: NoteAndAudioBuffers = {
            kick: [],
            snare: [],
            cymbals: [],
        };

        instruments.forEach((instrument) => {
            const instNotes = notes.filter((x) => x.instrument === instrument);
            const instFullAudioBuffer = fullAudioBuffers[instrument];
            instNotes.forEach((note, index) => {
                const nextNote = instNotes[index + 1];
                const nextNoteTimeSec =
                    nextNote?.timeSec ??
                    instFullAudioBuffer.length / instFullAudioBuffer.sampleRate;
                const sampleLength = Math.floor(
                    instFullAudioBuffer.sampleRate *
                        (nextNoteTimeSec - note.timeSec),
                );

                const noteAudioBuffer = audioContextRef.current.createBuffer(
                    instFullAudioBuffer.numberOfChannels,
                    sampleLength,
                    instFullAudioBuffer.sampleRate,
                );

                const sampleOffset = Math.floor(
                    instFullAudioBuffer.sampleRate * note.timeSec,
                );
                for (
                    let channel = 0;
                    channel < instFullAudioBuffer.numberOfChannels;
                    channel++
                ) {
                    instFullAudioBuffer.copyFromChannel(
                        noteAudioBuffer.getChannelData(channel),
                        channel,
                        sampleOffset,
                    );
                }

                noteAndAudioBuffers[instrument].push({
                    note,
                    audioBuffer: noteAudioBuffer,
                });
            });
        });

        setNoteAndAudioBuffers(noteAndAudioBuffers);
    }, [state.type, notes, fullAudioBuffers]);

    useEffect(() => {
        if (canvasRef.current === null || state.type === "stop") {
            return;
        }

        const canvas = canvasRef.current;
        const ctx = canvas.getContext("2d")!;

        let id = requestAnimationFrame(function tick() {
            id = requestAnimationFrame(tick);

            ctx.clearRect(0, 0, canvas.width, canvas.height);

            const timingZeroX = 200;

            ctx.beginPath();
            ctx.strokeStyle = "black";
            ctx.moveTo(timingZeroX, 0);
            ctx.lineTo(timingZeroX, canvas.height);
            ctx.stroke();

            const nowMs = new Date().getTime();
            const dtSec = (nowMs - state.startTimeMs) / 1000;
            const velocityPxPerSec = 100;
            const radius = 20;

            const fillStyles = {
                kick: "red",
                snare: "blue",
                cymbals: "green",
            };
            const noteSigns = {
                up: "↑",
                down: "↓",
                left: "←",
                right: "→",
            };
            const yMap = {
                up: 100 - radius * 2,
                right: 100 - radius * 2,
                left: 100,
                down: 100,
            };

            instruments.forEach((instrument) => {
                notes
                    .filter((note) => note.instrument === instrument)
                    .forEach((note) => {
                        const noteCenterX =
                            timingZeroX +
                            (note.timeSec - dtSec) * velocityPxPerSec;

                        if (
                            noteCenterX < -radius ||
                            canvas.width + radius < noteCenterX
                        ) {
                            return;
                        }

                        const y = yMap[note.direction];

                        ctx.beginPath();
                        ctx.arc(noteCenterX, y, radius, 0, 2 * Math.PI);
                        ctx.fillStyle = fillStyles[note.instrument];
                        ctx.fill();
                        ctx.stroke();

                        const sign = noteSigns[note.direction];
                        ctx.font = `${radius}px serif`;
                        ctx.textAlign = "center";
                        ctx.textBaseline = "middle";
                        ctx.fillStyle = "white";
                        ctx.fillText(sign, noteCenterX, y);
                    });
            });
        });

        return () => cancelAnimationFrame(id);
    }, [state, notes]);

    useEffect(() => {
        if (state.type === "stop") {
            return;
        }
        const audioContext = audioContextRef.current;

        const onKeydown = (e: KeyboardEvent) => {
            const direction = (() => {
                switch (e.key) {
                    case "ArrowUp":
                        return "up";
                    case "ArrowDown":
                        return "down";
                    case "ArrowLeft":
                        return "left";
                    case "ArrowRight":
                        return "right";
                    default:
                        return null;
                }
            })();

            if (!direction) {
                return;
            }

            const nowMs = new Date().getTime();
            const dtSec = (nowMs - state.startTimeMs) / 1000;
            const mostCloseNote = notes.reduce((acc, note) => {
                if (!acc) {
                    return note;
                }
                if (note.direction !== direction) {
                    return acc;
                }
                return Math.abs(note.timeSec - dtSec) <
                    Math.abs(acc.timeSec - dtSec)
                    ? note
                    : acc;
            }, undefined as Note | undefined);

            if (!mostCloseNote) {
                return;
            }

            const noteDtAbsSec = Math.abs(mostCloseNote.timeSec - dtSec);
            if (noteDtAbsSec > 0.5) {
                return;
            }

            const noteAndAudioBuffer = noteAndAudioBuffers?.[
                mostCloseNote.instrument
            ].find((x) => x.note === mostCloseNote);

            if (!noteAndAudioBuffer) {
                return;
            }

            const source = audioContext.createBufferSource();
            source.buffer = noteAndAudioBuffer.audioBuffer;
            source.connect(audioContext.destination);
            source.start(0);
        };
        window.addEventListener("keydown", onKeydown);

        return () => window.removeEventListener("keydown", onKeydown);
    }, [state, notes, noteAndAudioBuffers]);

    useEffect(() => {
        if (state.type === "stop") {
            return;
        }

        audioContextRef.current.close();
        audioContextRef.current = new AudioContext();
    }, [state, fullAudioBuffers]);

    useEffect(() => {
        if (state.type === "stop" || !fullAudioBuffers) {
            return;
        }

        const nowMs = new Date().getTime();
        const dtSec = (nowMs - state.startTimeMs) / 1000;

        const audioContext = audioContextRef.current;
        const source = audioContext.createBufferSource();
        source.buffer = fullAudioBuffers.others;

        const gainNode = audioContext.createGain();
        gainNode.gain.value = 0.5;

        source.connect(gainNode);
        gainNode.connect(audioContext.destination);

        source.start(0, dtSec);
    }, [state, fullAudioBuffers]);

    const togglePlay = () => {
        setState(
            state.type === "stop"
                ? { type: "play", startTimeMs: new Date().getTime() }
                : { type: "stop" },
        );
    };
    return (
        <>
            <button onClick={togglePlay}>Play / Stop</button>
            <canvas ref={canvasRef} width={800} height={600} />
        </>
    );
}
