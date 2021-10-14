type ColorProps = {
  r?: number;
  g?: number;
  b?: number;
  a?: number;
};

export default class Color {
  public r: number;
  public g: number;
  public b: number;
  public a: number;

  constructor(props: ColorProps = {}) {
    this.r = props.r || 255;
    this.g = props.g || 255;
    this.b = props.b || 255;
    this.a = props.a || 1;
  }

  public toRgbaString(alpha?: number) {
    return `rgba(${this.r}, ${this.g}, ${this.b}, ${
      typeof alpha === "number" ? alpha : this.a
    })`;
  }

  public toRgbString() {
    return `rgb(${this.r}, ${this.g}, ${this.b})`;
  }

  public blendTo(color: Color, ratio: number) {
    const reverseRatio = 1 - ratio;
    return new Color({
      r: this.r * ratio + color.r * reverseRatio,
      g: this.g * ratio + color.g * reverseRatio,
      b: this.b * ratio + color.b * reverseRatio,
      a: this.a * ratio + color.a * reverseRatio,
    });
  }

  get hsl() {
    const normalizedR = this.r / 255;
    const normalizedG = this.g / 255;
    const normalizedB = this.b / 255;
    const max = Math.max(normalizedR, normalizedG, normalizedB);
    const min = Math.min(normalizedR, normalizedG, normalizedB);
    const delta = max - min;

    let hue = 0;
    if (delta !== 0) {
      switch (max) {
        case normalizedR: {
          hue = (normalizedG - normalizedB) / delta;
          break;
        }
        case normalizedG: {
          hue = (normalizedB - normalizedR) / delta + 2;
          break;
        }
        case normalizedB: {
          hue = (normalizedR - normalizedG) / delta + 4;
          break;
        }
        default: {
          throw new Error("Can not calculate hue.");
        }
      }
    }
    hue = (hue < 0 ? hue + 6 : hue) * 360;

    const lightness = (max + min) / 2;
    const saturation =
      delta === 0 ? 0 : delta / (1 - Math.abs(2 * lightness - 1));
    return {
      h: hue,
      s: saturation,
      l: lightness,
    };
  }
}
