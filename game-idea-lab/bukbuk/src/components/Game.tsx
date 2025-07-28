import React from 'react';
import { GameProvider } from '../contexts/GameContext';
import { SceneRenderer } from './SceneRenderer';

export const Game: React.FC = () => {
  return (
    <GameProvider>
      <div className="game-container">
        <div className="header">
          <h1>아침 준비 시뮬레이터</h1>
        </div>
        <div className="content">
          <SceneRenderer />
        </div>
      </div>
    </GameProvider>
  );
};