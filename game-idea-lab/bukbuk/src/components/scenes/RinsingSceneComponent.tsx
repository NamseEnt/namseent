import React, { useState, useCallback, useMemo } from 'react';
import { GameScene } from '../../types';
import { useGame } from '../../contexts/GameContext';
import { useSceneTransition } from '../../hooks/useSceneTransition';
import { Narrative } from '../Narrative';
import { Choices } from '../Choices';

export const RinsingSceneComponent: React.FC = () => {
  const { gameState, updateGameState, changeScene } = useGame();
  const { isTransitioning, transition } = useSceneTransition();
  const [attemptMessage, setAttemptMessage] = useState('');

  const foamParts = useMemo(() => 
    gameState.soapedParts.filter(part => !gameState.rinsedParts.includes(part)),
    [gameState.soapedParts, gameState.rinsedParts]
  );

  const canRinse = foamParts[0];
  const otherParts = foamParts.slice(1);

  const handleRinse = useCallback(() => {
    updateGameState({
      rinsedParts: [...gameState.rinsedParts, canRinse]
    });

    const newFoamParts = gameState.soapedParts.filter(part => 
      ![...gameState.rinsedParts, canRinse].includes(part)
    );

    if (newFoamParts.length === 0) {
      transition(() => changeScene(GameScene.DRYING));
    }
  }, [canRinse, gameState, updateGameState, changeScene, transition]);

  const handleWrongAttempt = useCallback((part: string) => {
    setAttemptMessage(`\n\n위쪽을 먼저 헹궈야 합니다!`);
    setTimeout(() => setAttemptMessage(''), 2000);
  }, []);

  const getNarrative = useCallback(() => {
    if (foamParts.length === 0) {
      return '모든 거품을 헹궈냈습니다! 깨끗해졌네요.';
    }

    if (isTransitioning && canRinse) {
      return `${canRinse}의 거품이 물과 함께 흘러내립니다...`;
    }

    return `거품을 헹궈냅니다. 위에서부터 차례대로 헹궈야 합니다.

💧 현재 헹굴 수 있는 부위:
   → ${canRinse}

🧼 거품이 남은 부위:
${otherParts.length > 0 ? otherParts.map(part => `   • ${part}`).join('\n') : '   없음'}${attemptMessage}`;
  }, [foamParts, canRinse, otherParts, attemptMessage, isTransitioning]);

  const choices = useMemo(() => {
    if (isTransitioning || foamParts.length === 0) return [];

    const choices = [
      { text: `${canRinse}의 거품을 헹군다`, action: handleRinse }
    ];

    if (otherParts.length > 0) {
      choices.push({
        text: `${otherParts[0]}을 헹구려고 시도한다`,
        action: () => handleWrongAttempt(otherParts[0])
      });
    }

    return choices;
  }, [canRinse, otherParts, foamParts.length, isTransitioning, handleRinse, handleWrongAttempt]);

  return (
    <>
      <Narrative text={getNarrative()} />
      <Choices choices={choices} disabled={isTransitioning} />
    </>
  );
};