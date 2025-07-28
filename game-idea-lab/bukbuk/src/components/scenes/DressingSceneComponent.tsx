import React, { useCallback } from 'react';
import { GameScene } from '../../types';
import { useGame } from '../../contexts/GameContext';
import { useSceneTransition } from '../../hooks/useSceneTransition';
import { BaseBodyPartSelectionScene } from '../BaseBodyPartSelectionScene';
import { REQUIRED_CLOTHES } from '../../constants/gameData';

export const DressingSceneComponent: React.FC = () => {
  const { gameState, updateGameState, changeScene } = useGame();
  const { transition } = useSceneTransition();

  const handleWearClothing = useCallback((item: string) => {
    updateGameState({ clothes: [...gameState.clothes, item] });
  }, [gameState.clothes, updateGameState]);

  const handleComplete = useCallback(() => {
    transition(() => {
      changeScene(GameScene.MIRROR);
    });
  }, [changeScene, transition]);

  const getWearingMessage = useCallback((item: string) => {
    switch (item) {
      case '속옷':
        return '속옷을 입습니다. 깨끗하고 편안해요!';
      case '티셔츠':
        return '티셔츠를 입습니다. 오늘도 좋은 하루!';
      case '바지':
        return '바지를 입습니다. 딱 맞네요!';
      case '양말':
        return '양말을 신습니다. 포근해요!';
      default:
        return `${item}을(를) 입습니다.`;
    }
  }, []);

  return (
    <BaseBodyPartSelectionScene
      title="옷을 입습니다."
      subtitle="서랍에서 옷을 꺼내 입어요."
      items={REQUIRED_CLOTHES}
      completedItems={gameState.clothes}
      completedLabel="입은 옷"
      remainingLabel="남은 옷"
      actionVerb="을(를) 입는다"
      onItemSelect={handleWearClothing}
      onComplete={handleComplete}
      customMessage={getWearingMessage}
      itemIcon="👕"
    />
  );
};