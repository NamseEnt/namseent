import React, { useCallback } from 'react';
import { GameScene } from '../../types';
import { useGame } from '../../contexts/GameContext';
import { useSceneTransition } from '../../hooks/useSceneTransition';
import { BaseBodyPartSelectionScene } from '../BaseBodyPartSelectionScene';
import { BODY_PARTS } from '../../constants/gameData';

export const DryingSceneComponent: React.FC = () => {
  const { gameState, updateGameState, changeScene } = useGame();
  const { transition } = useSceneTransition();

  const handleDryPart = useCallback((part: string) => {
    updateGameState({ driedParts: [...gameState.driedParts, part] });
  }, [gameState.driedParts, updateGameState]);

  const handleComplete = useCallback(() => {
    transition(() => {
      updateGameState({ 
        hairDryParts: []
      });
      changeScene(GameScene.HAIR_DRY);
    });
  }, [updateGameState, changeScene, transition]);

  const getDryingMessage = useCallback((part: string) => {
    if (part === '머리') {
      return `${part}을(를) 천천히 닦습니다. 머리는 물기가 많아서 시간이 걸립니다.`;
    } else if (['팔', '다리'].includes(part)) {
      return `${part}을(를) 빠르게 닦습니다. 금방 마릅니다!`;
    } else {
      return `${part}을(를) 꼼꼼히 닦습니다.`;
    }
  }, []);

  return (
    <BaseBodyPartSelectionScene
      title="수건으로 물기를 닦습니다."
      subtitle="보송보송한 수건이 물기를 흡수합니다."
      items={BODY_PARTS}
      completedItems={gameState.driedParts}
      completedLabel="닦은 부위"
      remainingLabel="젖은 부위"
      actionVerb="을(를) 닦는다"
      onItemSelect={handleDryPart}
      onComplete={handleComplete}
      customMessage={getDryingMessage}
      itemIcon="🧺"
    />
  );
};