export type AudioBufferPlayerProps = {
  audioContext: AudioContext,
  src: string,
  nextNode: AudioNode,
  multiple: boolean,
  playbackRate: number,
}

type PlayInfo = {
  contextTime: number,
  sourceTime: number,
}

export default class AudioBufferPlayer {
  private readonly src: string;
  private readonly multiple: boolean;
  private buffer?: AudioBuffer;
  private audioContext: AudioContext;
  private nextNode: AudioNode;
  private ready: boolean = false;
  private playing: boolean = false;
  private playInfo: PlayInfo = {
    contextTime: 0,
    sourceTime: 0,
  }
  private lastBufferSource?: AudioBufferSourceNode;
  private playbackRate: number;

  constructor(props: AudioBufferPlayerProps) {
    const {
      audioContext,
      multiple,
      nextNode,
      playbackRate,
      src,
    } = props;
    this.audioContext = audioContext;
    this.multiple = multiple;
    this.nextNode = nextNode;
    this.playbackRate = playbackRate;
    this.src = src;
  }

  public async load() {
    if (this.ready) {
      return;
    }

    this.buffer = await this.audioContext.decodeAudioData(await (await fetch(this.src)).arrayBuffer());
    this.ready = true;
    return;
  }

  public play() {
    if (this.lastBufferSource) {
      if (this.multiple) {
        this.lastBufferSource.onended = null;
      } else {
        this.pause();
      }
    }

    if (!this.buffer?.length) {
      return;
    }

    const startOffset = this.multiple ? 0 : this.getStartOffset();
    const audioBufferSource = this.audioContext.createBufferSource();
    audioBufferSource.onended = () => this.stop();
    audioBufferSource.buffer = this.buffer;
    audioBufferSource.playbackRate.value = this.playbackRate;
    audioBufferSource.connect(this.nextNode);
    audioBufferSource.start(0, startOffset);
    this.lastBufferSource = audioBufferSource;
    this.playInfo = {
      contextTime: this.audioContext.currentTime,
      sourceTime: startOffset,
    };

    this.playing = true;
  }

  public pause() {
    if (this.lastBufferSource) {
      this.lastBufferSource.onended = null;
      try {
        this.lastBufferSource.stop();
      } catch (error) {
        if (error instanceof Error && error.name === 'InvalidStateError') {
          return;
        }
        throw error;
      }
    }
    this.playInfo = {
      contextTime: this.audioContext.currentTime,
      sourceTime: this.getStartOffset(),
    };

    this.playing = false;
  }

  public stop() {
    this.pause();
    this.playInfo = {
      contextTime: this.audioContext.currentTime,
      sourceTime: 0,
    }
  }

  public setPlaybackRate(playbackRate: number) {
    this.playbackRate = playbackRate;
    if (this.lastBufferSource) {
      this.lastBufferSource.playbackRate.value = playbackRate;
    }
  }

  private getStartOffset() {
    const {
      contextTime,
      sourceTime,
    } = this.playInfo;

    if (!this.playing) {
      return sourceTime;
    }

    return sourceTime + (this.audioContext.currentTime - contextTime);
  }
}
