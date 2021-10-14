import AudioBufferPlayer, { AudioBufferPlayerProps } from "./AudioBufferPlayer";

type AudioData = {
  [name: string]: Partial<AudioBufferPlayerProps> &
    Pick<AudioBufferPlayerProps, "src">;
};

type AudioManagerProps<Data extends AudioData> = {
  audioData: Data;
  masterVolume: number;
};

export default class AudioManager<Data extends AudioData> {
  private readonly audioBufferPlayers: Map<keyof Data, AudioBufferPlayer> =
    new Map();
  private readonly audioContext: AudioContext = new AudioContext();
  private readonly gainNode: GainNode = this.audioContext.createGain();

  constructor(props: AudioManagerProps<Data>) {
    const { audioData, masterVolume } = props;

    this.gainNode.connect(this.audioContext.destination);
    this.setVolume(masterVolume);

    Object.entries(audioData).forEach(([name, data]) => {
      const { src, multiple, playbackRate } = data;
      const audioBufferPlayer = new AudioBufferPlayer({
        audioContext: this.audioContext,
        nextNode: this.gainNode,
        src: src,
        multiple: multiple ?? true,
        playbackRate: playbackRate ?? 1,
      });
      this.audioBufferPlayers.set(name, audioBufferPlayer);
    });
  }

  public setVolume(volume: number) {
    this.gainNode.gain.value = volume;
  }

  public setPlaybackRate(playbackRate: number, name?: keyof Data) {
    if (name) {
      this.audioBufferPlayers.get(name)?.setPlaybackRate(playbackRate);
      return;
    }

    this.audioBufferPlayers.forEach((audioBufferPlayer) =>
      audioBufferPlayer.setPlaybackRate(playbackRate),
    );
  }

  public async load() {
    const loadingPromises: Promise<void>[] = [];
    this.audioBufferPlayers.forEach((audioBufferPlayer) => {
      loadingPromises.push(audioBufferPlayer.load());
    });
    await Promise.all(loadingPromises);
    return;
  }

  public play(name: keyof Data) {
    const audioBufferPlayer = this.audioBufferPlayers.get(name);
    if (!audioBufferPlayer) {
      return;
    }
    audioBufferPlayer.play();
  }

  public pause(name: keyof Data) {
    const audioBufferPlayer = this.audioBufferPlayers.get(name);
    if (!audioBufferPlayer) {
      return;
    }
    audioBufferPlayer.pause();
  }

  public stop(name: keyof Data) {
    const audioBufferPlayer = this.audioBufferPlayers.get(name);
    if (!audioBufferPlayer) {
      return;
    }
    audioBufferPlayer.stop();
  }
}
