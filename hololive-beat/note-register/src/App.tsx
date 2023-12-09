import { useEffect, useState } from "react";
import { Player } from "./Player";
import { Note, instruments } from "./types";

export function App() {
    const [notes, setNotes] = useState<Note[]>();

    useEffect(() => {
        (async () => {
            const notes: Note[] = [];
            const defaultDirectionMap = {
                kick: "down",
                snare: "left",
                cymbals: "up",
            } as const;
            await Promise.all(
                instruments.map(async (instrument) => {
                    const res = await fetch(`/${instrument}.txt`);
                    const text = await res.text();

                    const lines = text.split("\n");
                    lines.forEach((line) => {
                        const timeSec = line.trim();
                        notes.push({
                            timeSec: parseFloat(timeSec),
                            direction: defaultDirectionMap[instrument],
                            instrument,
                        });
                    });
                }),
            );

            setNotes(notes);
        })();
    }, []);

    if (!notes) {
        return <div>Loading...</div>;
    }

    return (
        <>
            <Player notes={notes} />
        </>
    );
}
