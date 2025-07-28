import React, { useState, useCallback, useEffect } from 'react';
import { GameScene } from '../../types';
import { useGame } from '../../contexts/GameContext';
import { useSceneTransition } from '../../hooks/useSceneTransition';
import { Narrative } from '../Narrative';
import { Choices } from '../Choices';

export const BedroomSceneComponent: React.FC = () => {
  const { gameState, changeScene } = useGame();
  const { isTransitioning, transition } = useSceneTransition();
  const [additionalText, setAdditionalText] = useState('');

  // 씬이 변경될 때 추가 텍스트 초기화
  useEffect(() => {
    setAdditionalText('');
  }, [gameState.scene]);

  const getBaseNarrative = () => {
    switch (gameState.scene) {
      case GameScene.BEDROOM_START:
        return `아침 햇살이 창문으로 들어옵니다.
당신은 포근한 이불 속에 누워 있습니다.
파란색 D자 모양의 이불이 당신을 덮고 있습니다.`;
      
      case GameScene.BEDROOM_BLANKET:
        return `이불을 아래로 끌어내립니다...
천천히... 더 아래로...

이불이 충분히 내려갔습니다! 이제 침대에 앉아 있습니다.`;
      
      case GameScene.BEDROOM_STANDING:
        return `일어섰습니다!
욕실 문이 저쪽에 보입니다. 노란색 빛이 문 주위를 감싸고 있습니다.`;
      
      default:
        return '';
    }
  };

  const getNarrative = () => {
    const base = getBaseNarrative();
    return additionalText || base;
  };

  const handleBlanketPull = useCallback(() => {
    changeScene(GameScene.BEDROOM_BLANKET);
  }, [changeScene]);

  const handleStayInBed = useCallback(() => {
    const base = getBaseNarrative();
    setAdditionalText(base + '\n\n하지만 오늘은 중요한 날입니다. 일어나야 해요!');
  }, []);

  const handleStandUp = useCallback(() => {
    changeScene(GameScene.BEDROOM_STANDING);
  }, [changeScene]);

  const handleStretch = useCallback(() => {
    const base = getBaseNarrative();
    setAdditionalText(base + '\n\n으으음~ 기지개를 켜니 몸이 좀 풀리는 것 같습니다.');
  }, []);

  const handleGoToBathroom = useCallback(() => {
    setAdditionalText('욕실로 걸어갑니다... 걸음마다 발소리가 들립니다.');
    transition(() => {
      changeScene(GameScene.BATHROOM_DOOR);
    }, 2000);
  }, [changeScene, transition]);

  const handleLookOutWindow = useCallback(() => {
    const base = getBaseNarrative();
    setAdditionalText(base + '\n\n밖은 화창한 날씨입니다. 하지만 먼저 씻어야겠네요.');
  }, []);

  const getChoices = () => {
    if (isTransitioning) return [];

    switch (gameState.scene) {
      case GameScene.BEDROOM_START:
        return [
          { text: '이불을 걷어낸다', action: handleBlanketPull },
          { text: '5분만 더...', action: handleStayInBed }
        ];
      
      case GameScene.BEDROOM_BLANKET:
        return [
          { text: '일어선다', action: handleStandUp },
          { text: '기지개를 켠다', action: handleStretch }
        ];
      
      case GameScene.BEDROOM_STANDING:
        return [
          { text: '욕실로 간다', action: handleGoToBathroom },
          { text: '창문을 본다', action: handleLookOutWindow }
        ];
      
      default:
        return [];
    }
  };

  return (
    <>
      <Narrative text={getNarrative()} />
      <Choices choices={getChoices()} disabled={isTransitioning} />
    </>
  );
};