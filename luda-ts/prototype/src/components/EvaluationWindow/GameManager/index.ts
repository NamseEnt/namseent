import ParticleManager from "./ParticleManager";
import Color from "./ColorManager/Color";
import Vector2D from "./ParticleManager/Vector2D";
import AudioManager from "./AudioManager";
import ColorManager from "./ColorManager";
import { EvaluationRenderingContext, SimonKey, keys, Turn } from "./type";
import emitSimonPose from "./ParticleManager/emitters/emitSimonPose";
import renderFace from "./render/renderFace";
import emitTextPoppingEffect from "./ParticleManager/emitters/emitTextPoppingEffect";
import beepW from "../../../../public/sound/evaluation-simon/beepW.mp3";
import beepA from "../../../../public/sound/evaluation-simon/beepA.mp3";
import beepS from "../../../../public/sound/evaluation-simon/beepS.mp3";
import beepD from "../../../../public/sound/evaluation-simon/beepD.mp3";
import shuffleList from "../../../util/shuffleList";

type GameFinishHandler = (success: boolean) => void;

type GameManagerProps = {
  gameFinishHandler?: GameFinishHandler;
  volume: number;
  maxStep: number;
  keyIncreasePerStep: number;
  initialKeyCount: number;
};

function convertStringToKey(keyString: string) {
  switch (keyString) {
    case "w": {
      return "w";
    }
    case "a": {
      return "a";
    }
    case "s": {
      return "s";
    }
    case "d": {
      return "d";
    }
    default: {
      return undefined;
    }
  }
}

export default class GameManager {
  private currentTime: number = 0;
  private startedAt: number;
  private lastRenderTime: number = 0;
  private turnChangedAt: number = 0;
  private turn: Turn = "trainer";
  private timeoutIds: Set<NodeJS.Timeout> = new Set();
  private sequence: SimonKey[] = [];
  private preservedSequence: SimonKey[] = [];
  private userSequenceIndex: number = 0;
  private step: number = 0;
  private maxStep: number;
  private keyIncreasePerStep: number;
  private readonly gameFinishHandler: Function;
  private readonly particleManager: ParticleManager = new ParticleManager();
  private readonly audioManager = new AudioManager({
    masterVolume: 0,
    audioData: {
      beepW: {
        src: beepW,
        multiple: true,
      },
      beepA: {
        src: beepA,
        multiple: true,
      },
      beepS: {
        src: beepS,
        multiple: true,
      },
      beepD: {
        src: beepD,
        multiple: true,
      },
    },
  });
  private readonly colorManager = new ColorManager({
    primary: new Color({ r: 25, g: 181, b: 254 }),
    secondary: new Color({ r: 240, g: 52, b: 52 }),
    line: new Color({ r: 255, g: 255, b: 255 }),
    great: new Color({ r: 0, g: 230, b: 64 }),
    perfect: new Color({ r: 255, g: 255, b: 204 }),
    hayeon: new Color({ r: 255, g: 255, b: 204 }),
    trainer: new Color({ r: 165, g: 55, b: 253 }),
  });
  private renderingContext?: EvaluationRenderingContext;

  constructor(props: GameManagerProps) {
    const {
      gameFinishHandler,
      volume,
      initialKeyCount,
      maxStep,
      keyIncreasePerStep,
    } = props;

    this.maxStep = maxStep;
    this.keyIncreasePerStep = keyIncreasePerStep;

    this.gameFinishHandler = gameFinishHandler || (() => undefined);
    this.startedAt = Date.now();

    this.audioManager.setVolume(volume);
    this.audioManager.load().then(() => {
      this.startedAt = Date.now();
      this.changeTurn("trainer");
    });

    for (let i = 0; i < initialKeyCount; i++) {
      this.pushNewKey();
    }

    this.keyDownHandler = this.keyDownHandler.bind(this);
  }

  public keyDownHandler(keyboardEvent: KeyboardEvent) {
    const { repeat, key: keyString } = keyboardEvent;
    if (repeat) {
      return;
    }
    if (keyString === "Backspace") {
      this.finishGame(false);
    }
    if (this.turn !== "hayeon") {
      return;
    }
    const key = convertStringToKey(keyString);
    if (!key) {
      return;
    }
    const correctKey = this.sequence[this.userSequenceIndex++];
    if (!correctKey) {
      throw new Error(
        "Index error. User pressed more than length of key sequence",
      );
    }

    this.popUpKeyEffect(key, "hayeon");

    if (key != correctKey) {
      this.changeTurn("gameover");
      emitTextPoppingEffect({
        particleManager: this.particleManager,
        text: "Miss",
        unitSize: this.renderingContext?.unitSize || 0,
        fillColor: this.colorManager.getColor("secondary"),
        strokeColor: this.colorManager.getColor("line"),
        fontSize: (this.renderingContext?.unitSize || 0) * 40,
        lineWidth: (this.renderingContext?.unitSize || 0) * 1,
        position: new Vector2D({
          x: (this.renderingContext?.canvasSize.width || 0) / 2,
          y: (this.renderingContext?.canvasSize.height || 0) / 2,
        }),
        power: 4,
      });
      return;
    }

    if (this.sequence.length > this.userSequenceIndex) {
      return;
    }

    this.step++;
    if (this.step >= this.maxStep) {
      this.changeTurn("gameover");
      emitTextPoppingEffect({
        particleManager: this.particleManager,
        text: "Clear",
        unitSize: this.renderingContext?.unitSize || 0,
        fillColor: this.colorManager.getColor("primary"),
        strokeColor: this.colorManager.getColor("line"),
        fontSize: (this.renderingContext?.unitSize || 0) * 40,
        lineWidth: (this.renderingContext?.unitSize || 0) * 1,
        position: new Vector2D({
          x: (this.renderingContext?.canvasSize.width || 0) / 2,
          y: (this.renderingContext?.canvasSize.height || 0) / 2,
        }),
        power: 4,
      });
      return;
    }

    for (let i = 0; i < this.keyIncreasePerStep; i++) {
      this.pushNewKey();
    }
    this.changeTurn("trainer");
  }

  public render(context: CanvasRenderingContext2D) {
    const { width, height } = context.canvas;
    const unitSize = Math.min(width, height) / 100;
    this.currentTime = this.startedAt + (Date.now() - this.startedAt);
    this.particleManager.tick(this.currentTime);
    this.lastRenderTime = this.currentTime;

    this.renderingContext = {
      context,
      unitSize,
      canvasSize: {
        width,
        height,
      },
      turn: this.turn,
      turnChangedAt: this.turnChangedAt,
      currentTime: this.currentTime,
    };

    renderFace(this.renderingContext);
    this.particleManager.render(context);
  }

  private finishGame(success?: boolean) {
    this.timeoutIds.forEach((id) => clearTimeout(id));
    this.gameFinishHandler(success);
  }

  private showDemonstrationAndTurnOverToUser() {
    const term = 666;
    this.sequence.forEach((key, index) => {
      const timeoutId = setTimeout(() => {
        this.popUpKeyEffect(key, "trainer");
        this.timeoutIds.delete(timeoutId);
      }, term * (1 + index));
      this.timeoutIds.add(timeoutId);
    });

    const timeoutId = setTimeout(() => {
      this.changeTurn("hayeon");
      this.timeoutIds.delete(timeoutId);
    }, term * (this.sequence.length + 1));
    this.timeoutIds.add(timeoutId);
  }

  private changeTurn(to: Turn) {
    this.turn = to;
    this.turnChangedAt = this.currentTime;
    switch (to) {
      case "hayeon": {
        this.userSequenceIndex = 0;
        break;
      }

      case "trainer": {
        this.showDemonstrationAndTurnOverToUser();
        break;
      }

      case "gameover": {
        const timeoutId = setTimeout(() => {
          this.finishGame(this.step >= this.maxStep);
          this.timeoutIds.delete(timeoutId);
        }, 3000);
        this.timeoutIds.add(timeoutId);
        break;
      }

      default:
        break;
    }
  }

  private popUpKeyEffect(key: SimonKey, turn: Turn) {
    if (turn === "gameover") {
      return;
    }
    emitSimonPose({
      particleManager: this.particleManager,
      key,
      color: this.colorManager.getColor(turn),
      position: new Vector2D({
        x: (this.renderingContext?.canvasSize.width || 0) / 2,
        y: (this.renderingContext?.canvasSize.height || 0) / 2,
      }),
      width: (this.renderingContext?.unitSize || 0) * 80,
    });
    emitTextPoppingEffect({
      particleManager: this.particleManager,
      text: key.toUpperCase(),
      unitSize: this.renderingContext?.unitSize || 0,
      fillColor: this.colorManager.getColor(turn),
      strokeColor: this.colorManager.getColor("line"),
      fontSize: (this.renderingContext?.unitSize || 0) * 40,
      lineWidth: (this.renderingContext?.unitSize || 0) * 1,
      position: new Vector2D({
        x: (this.renderingContext?.canvasSize.width || 0) / 2,
        y: (this.renderingContext?.canvasSize.height || 0) / 2,
      }),
      power: 4,
    });
    switch (key) {
      case "w": {
        this.audioManager.play("beepW");
        break;
      }
      case "a": {
        this.audioManager.play("beepA");
        break;
      }
      case "s": {
        this.audioManager.play("beepS");
        break;
      }
      case "d": {
        this.audioManager.play("beepD");
        break;
      }
      default:
        break;
    }
  }

  private pushNewKey() {
    if (this.preservedSequence.length < 1) {
      this.preservedSequence.push(...shuffleList(keys));
    }
    const newKey = this.preservedSequence.pop();
    if (!newKey) {
      throw new Error("Random key generation failed. Check preservedSequence.");
    }
    this.sequence.push(newKey);
  }
}
