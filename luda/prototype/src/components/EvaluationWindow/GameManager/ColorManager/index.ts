import Color from "./Color";

type Colors = {
  primary: Color;
  secondary: Color;
  line: Color;
  great: Color;
  perfect: Color;
  hayeon: Color;
  trainer: Color;
};

export default class ColorManager {
  private colors: Colors;

  constructor(colors: Colors) {
    this.colors = colors;
  }

  public getColor<Name extends keyof Colors>(name: Name) {
    return this.colors[name];
  }
}
