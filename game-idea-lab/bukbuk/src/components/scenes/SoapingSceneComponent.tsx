import React, { useCallback, useMemo, useEffect } from 'react';
import { GameScene } from '../../types';
import { useGame } from '../../contexts/GameContext';
import { useSceneTransition } from '../../hooks/useSceneTransition';
import { Narrative } from '../Narrative';
import { Choices } from '../Choices';
import { BODY_PARTS } from '../../constants/gameData';
import { getSoapProgressIncrease, createProgressBar } from '../../utils/gameLogic';

export const SoapingSceneComponent: React.FC = () => {
  const { gameState, updateGameState, changeScene } = useGame();
  const { isTransitioning, transition, setIsTransitioning } = useSceneTransition();

  const currentPart = useMemo(() => 
    BODY_PARTS[gameState.currentSoapPart],
    [gameState.currentSoapPart]
  );

  const progress = useMemo(() => 
    gameState.soapProgress[currentPart] || 0,
    [gameState.soapProgress, currentPart]
  );

  useEffect(() => {
    if (!currentPart && !isTransitioning) {
      transition(() => {
        changeScene(GameScene.RINSE_BODY);
      });
    } else if (currentPart && progress >= 100 && !isTransitioning) {
      setIsTransitioning(true);
      const nextPartIndex = gameState.currentSoapPart + 1;
      const nextPart = BODY_PARTS[nextPartIndex];
      
      updateGameState({
        soapedParts: [...gameState.soapedParts, currentPart],
        currentSoapPart: nextPartIndex,
        soapProgress: {
          ...gameState.soapProgress,
          [nextPart]: 0  // 다음 부위의 진행도를 0으로 초기화
        }
      });
      setTimeout(() => setIsTransitioning(false), 1000);
    }
  }, [currentPart, progress, gameState, updateGameState, changeScene, transition, isTransitioning, setIsTransitioning]);

  const handleSoap = useCallback((intense: boolean) => {
    const increase = getSoapProgressIncrease(intense);
    const newProgress = Math.min(100, progress + increase);
    
    updateGameState({
      soapProgress: {
        ...gameState.soapProgress,
        [currentPart]: newProgress
      }
    });
  }, [currentPart, progress, gameState.soapProgress, updateGameState]);

  const getNarrative = useCallback(() => {
    if (!currentPart) {
      return '모든 부위를 비누칠했습니다! 이제 헹궈낼 차례입니다.';
    }

    const progressBar = createProgressBar(progress);
    const previousParts = BODY_PARTS.slice(0, gameState.currentSoapPart);
    const nextParts = BODY_PARTS.slice(gameState.currentSoapPart + 1);

    return `${currentPart}을(를) 비누칠하는 중입니다.

🧼 진행도: ${progressBar} ${Math.round(progress)}%

비누칠을 하면 거품이 생깁니다. 클릭하여 문지르세요!

📝 진행 순서: ${previousParts.join(' → ')}${previousParts.length > 0 ? ' → ' : ''}👉 ${currentPart} ${nextParts.length > 0 ? '→ ' + nextParts.join(' → ') : ''}`;
  }, [currentPart, progress, gameState.currentSoapPart]);

  const choices = useMemo(() => {
    if (!currentPart || progress >= 100 || isTransitioning) return [];

    return [
      { text: '문지른다 (클릭!)', action: () => handleSoap(false) },
      { text: '세게 문지른다', action: () => handleSoap(true) }
    ];
  }, [currentPart, progress, isTransitioning, handleSoap]);

  return (
    <>
      <Narrative text={getNarrative()} />
      <Choices choices={choices} disabled={isTransitioning} />
    </>
  );
};