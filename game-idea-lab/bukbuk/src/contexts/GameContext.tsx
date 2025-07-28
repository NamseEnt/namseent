import React, { createContext, useContext, useState, useCallback, ReactNode } from 'react';
import { GameState, GameScene } from '../types';
import { INITIAL_SHOWER_TEMP, INITIAL_BODY_TEMP } from '../constants/gameData';
import { canTransitionToScene } from '../utils/sceneValidation';

interface GameContextType {
  gameState: GameState;
  updateGameState: (updates: Partial<GameState> | ((prev: GameState) => Partial<GameState>)) => void;
  changeScene: (scene: GameScene) => void;
  safeChangeScene: (scene: GameScene) => boolean;
  resetGame: () => void;
}

const initialState: GameState = {
  scene: GameScene.BEDROOM_START,
  showerTemp: INITIAL_SHOWER_TEMP,
  bodyTemp: INITIAL_BODY_TEMP,
  perfectTempTime: 0,
  wetParts: [],
  soapedParts: [],
  rinsedParts: [],
  driedParts: [],
  hairDryParts: [],
  clothes: [],
  currentSoapPart: 0,
  soapProgress: {},
  clickCount: 0,
  isTransitioning: false
};

const GameContext = createContext<GameContextType | undefined>(undefined);

export const GameProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [gameState, setGameState] = useState<GameState>(initialState);

  const updateGameState = useCallback((updates: Partial<GameState> | ((prev: GameState) => Partial<GameState>)) => {
    if (typeof updates === 'function') {
      setGameState(prev => ({ ...prev, ...updates(prev) }));
    } else {
      setGameState(prev => ({ ...prev, ...updates }));
    }
  }, []);

  const changeScene = useCallback((scene: GameScene) => {
    setGameState(prev => ({ ...prev, scene }));
  }, []);
  
  const safeChangeScene = useCallback((scene: GameScene) => {
    const validation = canTransitionToScene(scene, gameState);
    if (validation.valid) {
      changeScene(scene);
      return true;
    } else {
      console.warn(`Cannot transition to ${scene}: ${validation.reason}`);
      return false;
    }
  }, [gameState, changeScene]);

  const resetGame = useCallback(() => {
    setGameState(initialState);
  }, []);

  const value = {
    gameState,
    updateGameState,
    changeScene,
    safeChangeScene,
    resetGame
  };

  return <GameContext.Provider value={value}>{children}</GameContext.Provider>;
};

export const useGame = () => {
  const context = useContext(GameContext);
  if (!context) {
    throw new Error('useGame must be used within GameProvider');
  }
  return context;
};