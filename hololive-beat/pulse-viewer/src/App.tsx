import "./App.css";
import { useEffect, useRef, useState } from "react";
import opus from "/cymbals.opus";
import Peaks, { PeaksInstance } from "peaks.js";
import txt from "/cymbals.txt";

function App() {
    const [audioBuffer, setAudioBuffer] = useState<AudioBuffer>();
    const [peaksInstance, setPeaksInstance] = useState<PeaksInstance>();
    const zoomviewRef = useRef<HTMLDivElement>(null);
    const overviewRef = useRef<HTMLDivElement>(null);
    const mediaElementRef = useRef<HTMLAudioElement>(null);
    const [onsetText, setOnsetText] = useState<string>("");
    const [timeInSeconds, setTimeInSeconds] = useState<number>();

    useEffect(() => {
        (async () => {
            const response = await fetch(opus);
            const arrayBuffer = await response.arrayBuffer();

            const audioContext = new AudioContext();
            const audioBuffer = await audioContext.decodeAudioData(arrayBuffer);

            setAudioBuffer(audioBuffer);
        })();
    }, []);

    useEffect(() => {
        (async () => {
            const response = await fetch(txt);
            const text = await response.text();
            setOnsetText(text);
        })();
    }, []);

    useEffect(() => {
        if (!audioBuffer) {
            return;
        }
        Peaks.init(
            {
                zoomview: {
                    container: zoomviewRef.current,
                    playheadPadding: 8,
                },
                overview: {
                    container: overviewRef.current,
                    playheadPadding: 8,
                },
                webAudio: {
                    audioBuffer,
                },
                mediaElement: mediaElementRef.current ?? undefined,
                zoomLevels: [16, 32, 64, 128, 256, 512, 1024],
                keyboard: true,
            },
            (err, peaks) => {
                if (err) {
                    throw err;
                }
                setPeaksInstance(peaks);
                peaks?.views;
            },
        );
    }, [audioBuffer]);

    const onWheel = (e: React.WheelEvent<HTMLDivElement>) => {
        if (!peaksInstance) {
            return;
        }

        peaksInstance.player.seek;
        peaksInstance.zoom.setZoom(
            peaksInstance.zoom.getZoom() + (e.deltaY < 0 ? -1 : 1),
        );
    };

    useEffect(() => {
        if (!peaksInstance) {
            return;
        }

        peaksInstance.on("player.timeupdate", () => {
            setTimeInSeconds(peaksInstance.player.getCurrentTime());
        });

        const onKeyDown = (e: KeyboardEvent) => {
            switch (e.code) {
                case "Space":
                    {
                        const isPlaying = !mediaElementRef.current?.paused;
                        if (isPlaying) {
                            peaksInstance.player.pause();
                        } else {
                            peaksInstance.player.play();
                        }
                    }
                    break;
                case "Enter":
                    {
                        setOnsetText((prev) => {
                            return [
                                ...prev.split("\n").map((x) => Number(x)),
                                peaksInstance.player.getCurrentTime(),
                            ]
                                .sort((a, b) => a - b)
                                .join("\n");
                        });
                        peaksInstance.player.getCurrentTime;
                    }
                    break;
            }
        };
        window.addEventListener("keydown", onKeyDown);

        return () => {
            window.removeEventListener("keydown", onKeyDown);
        };
    }, [peaksInstance]);

    useEffect(() => {
        if (!peaksInstance || !onsetText) {
            return;
        }

        peaksInstance.points.removeAll();

        onsetText.split("\n").forEach((value) => {
            peaksInstance.points.add({
                time: Number(value),
                color: "red",
            });
        });
    }, [onsetText, peaksInstance]);

    return (
        <div>
            <div
                ref={zoomviewRef}
                style={{
                    width: "80vw",
                    height: "40vh",
                }}
                onWheel={onWheel}
            />
            <div
                ref={overviewRef}
                style={{
                    width: "80vw",
                    height: "20vh",
                }}
                onWheel={onWheel}
            />
            <audio ref={mediaElementRef} src={opus} controls />
            <span>Time in seconds: {timeInSeconds}</span>
            <textarea
                value={onsetText}
                onChange={(e) => {
                    setOnsetText(e.target.value);
                }}
            />
        </div>
    );
}

export default App;
