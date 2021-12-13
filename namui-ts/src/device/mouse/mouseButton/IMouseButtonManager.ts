export interface IMouseButtonManager {
  readonly isLeftMouseButtonDown: boolean;
  readonly isRightMouseButtonDown: boolean;
}

export interface IMouseButtonManagerInternal extends IMouseButtonManager {
  set isLeftMouseButtonDown(isDown: boolean);
  set isRightMouseButtonDown(isDown: boolean);
}
