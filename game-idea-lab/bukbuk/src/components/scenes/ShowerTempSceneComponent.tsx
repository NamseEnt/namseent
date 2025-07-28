import React, { useEffect, useRef, useCallback } from 'react';
import { GameScene } from '../../types';
import { useGame } from '../../contexts/GameContext';
import { useSceneTransition } from '../../hooks/useSceneTransition';
import { Narrative } from '../Narrative';
import { Choices } from '../Choices';
import { 
  getWaterColor, 
  calculateBodyTempChange, 
  isPerfectTemperature, 
  isBodyTempDangerous,
  getRandomTempChange 
} from '../../utils/gameLogic';
import { PERFECT_TEMP_DURATION } from '../../constants/gameData';

export const ShowerTempSceneComponent: React.FC = () => {
  const { gameState, updateGameState, changeScene } = useGame();
  const { isTransitioning, transition } = useSceneTransition();
  const intervalRef = useRef<NodeJS.Timeout | null>(null);
  

  const getNarrative = useCallback(() => {
    const { showerTemp, bodyTemp, perfectTempTime } = gameState;
    const waterColor = getWaterColor(showerTemp);
    
    let text = `삐걱거리는 수도꼭지를 조절합니다.
현재 물 온도: ${showerTemp}°C (${waterColor}색 물)
체온: ${bodyTemp.toFixed(1)}°C

적정 온도(35-45°C)를 3초간 유지하세요!`;

    if (isPerfectTemperature(showerTemp)) {
      text += `\n\n완벽한 온도! ${(PERFECT_TEMP_DURATION - perfectTempTime)}초만 더!`;
    } else if (showerTemp < 35) {
      text += '\n\n너무 차가워요! 몸이 떨립니다...';
    } else if (showerTemp > 45) {
      text += '\n\n너무 뜨거워요! 화상을 입을 것 같아요...';
    }

    if (perfectTempTime >= PERFECT_TEMP_DURATION) {
      return '완벽한 온도를 찾았습니다!';
    }

    return text;
  }, [gameState]);

  const handleTempAdjust = useCallback((direction: 'cold' | 'hot') => {
    const change = getRandomTempChange();
    const newTemp = direction === 'cold' 
      ? Math.max(10, gameState.showerTemp - change)
      : Math.min(80, gameState.showerTemp + change);
    
    updateGameState({ showerTemp: newTemp });
  }, [gameState.showerTemp, updateGameState]);

  // 온도 업데이트 인터벌
  useEffect(() => {
    
    const intervalId = setInterval(() => {
      updateGameState((prev: GameState) => {

        const tempChange = calculateBodyTempChange(prev.bodyTemp, prev.showerTemp);
        let newBodyTemp = prev.bodyTemp + tempChange;
        let newPerfectTime = prev.perfectTempTime;

        if (isPerfectTemperature(prev.showerTemp)) {
          newPerfectTime++;
        } else {
          newPerfectTime = 0;
        }

        if (isBodyTempDangerous(newBodyTemp)) {
          newBodyTemp = 36.5;
          newPerfectTime = 0;
        }

        return {
          bodyTemp: newBodyTemp,
          perfectTempTime: newPerfectTime
        };
      });
    }, 1000);
    
    intervalRef.current = intervalId;

    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    };
  }, []); // 컴포넌트 마운트 시 한 번만 실행

  // 완료 체크 및 씬 전환
  useEffect(() => {
    if (gameState.perfectTempTime >= PERFECT_TEMP_DURATION && !isTransitioning) {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
        intervalRef.current = null;
      }
      
      transition(() => {
        changeScene(GameScene.WET_BODY);
      });
    }
  }, [gameState.perfectTempTime, changeScene, transition, isTransitioning]);

  const choices = gameState.perfectTempTime >= PERFECT_TEMP_DURATION || isTransitioning ? [] : [
    { text: '차갑게 돌린다 ◀', action: () => handleTempAdjust('cold') },
    { text: '뜨겁게 돌린다 ▶', action: () => handleTempAdjust('hot') }
  ];

  return (
    <>
      <Narrative text={getNarrative()} />
      <Choices choices={choices} disabled={isTransitioning} />
    </>
  );
};