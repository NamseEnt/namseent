import React, { useCallback, useMemo } from 'react';
import { GameScene } from '../../types';
import { useGame } from '../../contexts/GameContext';
import { useSceneTransition } from '../../hooks/useSceneTransition';
import { Narrative } from '../Narrative';
import { Choices } from '../Choices';
import { HAIR_DRY_TARGETS } from '../../constants/gameData';
import { getDryerEffectiveness } from '../../utils/gameLogic';

export const HairDrySceneComponent: React.FC = () => {
  const { gameState, updateGameState, changeScene } = useGame();
  const { isTransitioning, transition } = useSceneTransition();

  const remainingTargets = useMemo(() => 
    HAIR_DRY_TARGETS.filter(part => !gameState.hairDryParts.includes(part)),
    [gameState.hairDryParts]
  );

  const handleDryAttempt = useCallback((target: string) => {
    updateGameState({ clickCount: gameState.clickCount + 1 });
    const effectiveness = getDryerEffectiveness(gameState.clickCount);
    
    if (effectiveness === 'close') {
      updateGameState({
        hairDryParts: [...gameState.hairDryParts, target]
      });
      
      const newRemaining = HAIR_DRY_TARGETS.filter(part => 
        ![...gameState.hairDryParts, target].includes(part)
      );
      
      if (newRemaining.length === 0) {
        transition(() => changeScene(GameScene.DRESSING));
      }
    }
  }, [gameState, updateGameState, changeScene, transition]);

  const getNarrative = useCallback(() => {
    if (remainingTargets.length === 0) {
      return '모든 부위를 말렸습니다! 이제 옷을 입을 차례입니다.';
    }

    const lastAttempt = HAIR_DRY_TARGETS.find(target => 
      !gameState.hairDryParts.includes(target) && gameState.clickCount > 0
    );

    if (lastAttempt && gameState.clickCount > 0) {
      const effectiveness = getDryerEffectiveness(gameState.clickCount - 1);
      
      if (effectiveness === 'close') {
        return `${lastAttempt}에 드라이어를 아주 가까이 댑니다. 효과적입니다!${
          lastAttempt === '머리카락' ? '\n\n머리카락이 바람에 휘날립니다~' : ''
        }`;
      } else if (effectiveness === 'medium') {
        return `${lastAttempt}에 드라이어를 적당한 거리에서 사용합니다.\n\n조금 더 가까이 대보세요.`;
      } else {
        return `${lastAttempt}에 드라이어가 너무 멉니다. 바람이 잘 닿지 않습니다.${
          gameState.clickCount > 10 ? '\n\n너무 뜨거워요! 조심하세요!' : ''
        }`;
      }
    }

    return `드라이어로 특정 부위를 말립니다.

🔥 남은 부위:
${remainingTargets.map(part => `   • ${part}`).join('\n')}

드라이어를 가까이 대면 더 빨리 마릅니다.`;
  }, [remainingTargets, gameState.hairDryParts, gameState.clickCount]);

  const choices = useMemo(() => {
    if (isTransitioning || remainingTargets.length === 0) return [];

    return remainingTargets.map(target => ({
      text: `${target}에 드라이어를 댄다`,
      action: () => handleDryAttempt(target)
    }));
  }, [remainingTargets, isTransitioning, handleDryAttempt]);

  return (
    <>
      <Narrative text={getNarrative()} />
      <Choices choices={choices} disabled={isTransitioning} />
    </>
  );
};