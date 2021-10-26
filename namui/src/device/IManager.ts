export interface IManagerInternal {
  resetBeforeRender?: () => void;
  destroy?: () => void;
  afterRender?: () => void;
}
