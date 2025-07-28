import React, { useCallback } from 'react';
import { GameScene } from '../../types';
import { useGame } from '../../contexts/GameContext';
import { useSceneTransition } from '../../hooks/useSceneTransition';
import { BaseBodyPartSelectionScene } from '../BaseBodyPartSelectionScene';
import { BODY_PARTS } from '../../constants/gameData';

export const WetBodySceneComponent: React.FC = () => {
  const { gameState, updateGameState, changeScene } = useGame();
  const { transition } = useSceneTransition();

  const handleWetPart = useCallback((part: string) => {
    updateGameState({ wetParts: [...gameState.wetParts, part] });
  }, [gameState.wetParts, updateGameState]);

  const handleComplete = useCallback(() => {
    transition(() => {
      updateGameState({ 
        currentSoapPart: 0,
        soapProgress: BODY_PARTS.reduce((acc, p) => ({ ...acc, [p]: 0 }), {})
      });
      changeScene(GameScene.SOAP_BODY);
    });
  }, [updateGameState, changeScene, transition]);

  return (
    <BaseBodyPartSelectionScene
      title="물을 몸에 끼얹습니다."
      subtitle="아직 젖지 않은 부위들이 반짝이고 있습니다."
      items={BODY_PARTS}
      completedItems={gameState.wetParts}
      completedLabel="젖은 부위"
      remainingLabel="남은 부위"
      actionVerb="에 물을 끼얹는다"
      onItemSelect={handleWetPart}
      onComplete={handleComplete}
      customMessage={(part) => `${part}에 물을 끼얹었습니다!`}
      itemIcon="💧"
    />
  );
};