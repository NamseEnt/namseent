export type Note = {
    timeSec: number;
    direction: "left" | "right" | "up" | "down";
    instrument: "kick" | "snare" | "cymbals";
};

export const instruments = ["kick", "snare", "cymbals"] as const;
