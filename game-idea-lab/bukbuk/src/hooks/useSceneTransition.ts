import { useState, useCallback } from 'react';

export const useSceneTransition = (duration: number = 1500) => {
  const [isTransitioning, setIsTransitioning] = useState(false);
  
  const transition = useCallback((callback: () => void, customDuration?: number) => {
    setIsTransitioning(true);
    setTimeout(() => {
      callback();
      setIsTransitioning(false);
    }, customDuration || duration);
  }, [duration]);
  
  const immediateTransition = useCallback((callback: () => void) => {
    setIsTransitioning(true);
    callback();
    setIsTransitioning(false);
  }, []);
  
  return { isTransitioning, transition, immediateTransition, setIsTransitioning };
};