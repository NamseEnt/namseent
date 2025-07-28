import React, { useCallback } from 'react';
import { GameScene } from '../../types';
import { useGame } from '../../contexts/GameContext';
import { BaseSceneComponent } from '../BaseSceneComponent';

export const BathroomSceneComponent: React.FC = () => {
  const { changeScene } = useGame();

  const getNarrative = useCallback(() => 
    `욕실에 도착했습니다.
샤워기와 수도꼭지가 보입니다.`,
  []);

  const getChoices = useCallback(() => [
    {
      text: '샤워기를 켠다',
      action: () => changeScene(GameScene.SHOWER_TEMP)
    }
  ], [changeScene]);

  return <BaseSceneComponent getNarrative={getNarrative} getChoices={getChoices} />;
};