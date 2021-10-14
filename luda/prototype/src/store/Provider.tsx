import React, { Component, ReactNode } from "react";
import { initialState, State } from "./State";
import produce from "immer";
import { context } from "./context";

type ProviderProps = {
  children: ReactNode;
};

type ProviderState = State;

export default class Provider extends Component<ProviderProps, ProviderState> {
  private stateBuffer: State = initialState;

  constructor(props: ProviderProps) {
    super(props);
    this.state = initialState;
    this.update = this.update.bind(this);
  }

  private update(updater: (state: State) => void) {
    const newState = produce(this.stateBuffer, (draft) => {
      updater(draft);
      return draft;
    });
    this.stateBuffer = newState;
    this.setState(newState);
  }

  render() {
    return (
      <context.Provider value={[this.state, this.update]}>
        {this.props.children}
      </context.Provider>
    );
  }
}
