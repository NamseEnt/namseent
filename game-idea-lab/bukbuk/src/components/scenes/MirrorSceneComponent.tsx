import React, { useCallback } from 'react';
import { GameScene } from '../../types';
import { useGame } from '../../contexts/GameContext';
import { useSceneTransition } from '../../hooks/useSceneTransition';
import { Narrative } from '../Narrative';
import { Choices } from '../Choices';

export const MirrorSceneComponent: React.FC = () => {
  const { updateGameState, changeScene } = useGame();
  const { isTransitioning, transition } = useSceneTransition();

  const handleChoice = useCallback((choice: string, message: string) => {
    updateGameState({ finalChoice: choice });
    transition(() => changeScene(GameScene.ENDING), 2000);
  }, [updateGameState, changeScene, transition]);

  const getNarrative = () => {
    if (isTransitioning) {
      const messages: Record<string, string> = {
        '잘 갔다오자!': '"잘 갔다오자!" 라고 거울에 비친 자신에게 말합니다.\n\n오늘도 좋은 하루가 될 거예요!',
        '오늘도 화이팅!': '"오늘도 화이팅!" 이라고 활기차게 외칩니다.\n\n힘차게 하루를 시작해요!',
        '완벽해!': '"완벽해!" 라고 만족스럽게 웃습니다.\n\n준비 완료! 멋진 하루를 보내세요!'
      };
      
      const lastChoice = localStorage.getItem('lastMirrorChoice');
      return messages[lastChoice || ''] || '';
    }

    return `거울 앞에 섰습니다.
깨끗하게 씻고 새 옷을 입은 당신의 모습이 보입니다.

무엇이라고 말할까요?`;
  };

  const choices = isTransitioning ? [] : [
    {
      text: '잘 갔다오자!',
      action: () => {
        localStorage.setItem('lastMirrorChoice', '잘 갔다오자!');
        handleChoice('잘 갔다오자!', '오늘도 좋은 하루가 될 거예요!');
      }
    },
    {
      text: '오늘도 화이팅!',
      action: () => {
        localStorage.setItem('lastMirrorChoice', '오늘도 화이팅!');
        handleChoice('오늘도 화이팅!', '힘차게 하루를 시작해요!');
      }
    },
    {
      text: '완벽해!',
      action: () => {
        localStorage.setItem('lastMirrorChoice', '완벽해!');
        handleChoice('완벽해!', '준비 완료! 멋진 하루를 보내세요!');
      }
    }
  ];

  return (
    <>
      <Narrative text={getNarrative()} />
      <Choices choices={choices} disabled={isTransitioning} />
    </>
  );
};