import React, { useState, useCallback, useMemo, useEffect } from 'react';
import { Narrative } from './Narrative';
import { Choices } from './Choices';
import { Choice } from '../types';

interface BaseBodyPartSelectionSceneProps {
  title: string;
  subtitle?: string;
  items: string[];
  completedItems: string[];
  completedLabel: string;
  remainingLabel: string;
  actionVerb: string; // 예: "물을 끼얹는다", "닦는다", "입는다"
  onItemSelect: (item: string) => void;
  onComplete?: () => void;
  customMessage?: (item: string) => string;
  itemIcon?: string;
  hideChoicesOnComplete?: boolean;
}

export const BaseBodyPartSelectionScene: React.FC<BaseBodyPartSelectionSceneProps> = ({
  title,
  subtitle,
  items,
  completedItems,
  completedLabel,
  remainingLabel,
  actionVerb,
  onItemSelect,
  onComplete,
  customMessage,
  itemIcon = '✨',
  hideChoicesOnComplete = false
}) => {
  const [lastSelectedItem, setLastSelectedItem] = useState<string>('');
  const [showAnimation, setShowAnimation] = useState(false);

  const remainingItems = useMemo(() => 
    items.filter(item => !completedItems.includes(item)),
    [items, completedItems]
  );

  // 아이템 선택 시 애니메이션
  useEffect(() => {
    if (lastSelectedItem) {
      setShowAnimation(true);
      const timer = setTimeout(() => {
        setShowAnimation(false);
      }, 500);
      return () => clearTimeout(timer);
    }
  }, [lastSelectedItem]);

  // 모든 아이템 완료 시
  useEffect(() => {
    if (remainingItems.length === 0 && completedItems.length > 0 && onComplete) {
      const timer = setTimeout(() => {
        onComplete();
      }, 1000);
      return () => clearTimeout(timer);
    }
  }, [remainingItems.length, completedItems.length, onComplete]);

  const handleItemSelect = useCallback((item: string) => {
    setLastSelectedItem(item);
    onItemSelect(item);
  }, [onItemSelect]);

  const getNarrative = useCallback(() => {
    if (remainingItems.length === 0 && completedItems.length > 0) {
      return <div className="completion-message">모든 {remainingLabel}를 완료했습니다! 🎉</div>;
    }

    return (
      <div className="selection-scene">
        <div className="scene-title">{title}</div>
        {subtitle && <div className="scene-subtitle">{subtitle}</div>}
        
        {lastSelectedItem && showAnimation && (
          <div className="action-message fade-in" style={{
            color: '#4a90e2',
            fontWeight: 'bold',
            margin: '15px 0',
            padding: '10px',
            backgroundColor: 'rgba(74, 144, 226, 0.1)',
            borderRadius: '5px',
            animation: 'fadeIn 0.5s ease-out'
          }}>
            {itemIcon} {customMessage ? customMessage(lastSelectedItem) : `${lastSelectedItem}을(를) ${actionVerb}했습니다!`}
          </div>
        )}
        
        <div className="item-status" style={{ marginTop: '20px' }}>
          <div className="completed-section">
            <div style={{ fontWeight: 'bold', marginBottom: '10px' }}>✅ {completedLabel}:</div>
            {completedItems.length > 0 ? (
              <div className="item-list">
                {completedItems.map((item, index) => (
                  <div 
                    key={index} 
                    className={`item ${item === lastSelectedItem && showAnimation ? 'highlight' : ''}`}
                    style={{ 
                      marginLeft: '20px',
                      padding: '5px',
                      animation: item === lastSelectedItem && showAnimation ? 'highlight 0.5s ease-out' : 'none'
                    }}
                  >
                    • {item}
                  </div>
                ))}
              </div>
            ) : (
              <div style={{ marginLeft: '20px', color: '#999' }}>아직 없음</div>
            )}
          </div>
          
          {remainingItems.length > 0 && (
            <div className="remaining-section" style={{ marginTop: '15px' }}>
              <div style={{ fontWeight: 'bold', marginBottom: '10px' }}>⭕ {remainingLabel}:</div>
              <div className="item-list">
                {remainingItems.map((item, index) => (
                  <div key={index} style={{ marginLeft: '20px', padding: '5px' }}>
                    • {item}
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      </div>
    );
  }, [title, subtitle, completedItems, remainingItems, lastSelectedItem, showAnimation, 
      completedLabel, remainingLabel, actionVerb, itemIcon, customMessage]);

  const choices = useMemo<Choice[]>(() => {
    if (remainingItems.length === 0 || hideChoicesOnComplete) return [];

    return remainingItems.map(item => ({
      text: `${item}${actionVerb}`,
      action: () => handleItemSelect(item)
    }));
  }, [remainingItems, actionVerb, handleItemSelect, hideChoicesOnComplete]);

  return (
    <>
      <style>{`
        @keyframes highlight {
          0% {
            background-color: rgba(74, 144, 226, 0.3);
            transform: translateX(-10px);
          }
          100% {
            background-color: transparent;
            transform: translateX(0);
          }
        }
        
        @keyframes fadeIn {
          from {
            opacity: 0;
            transform: translateY(-10px);
          }
          to {
            opacity: 1;
            transform: translateY(0);
          }
        }

        .selection-scene {
          line-height: 1.6;
        }

        .scene-title {
          font-size: 1.1em;
          margin-bottom: 5px;
        }

        .scene-subtitle {
          color: #666;
          margin-bottom: 15px;
        }

        .completion-message {
          font-size: 1.2em;
          text-align: center;
          padding: 20px;
          color: #4a90e2;
          font-weight: bold;
        }

        .fade-in {
          animation: fadeIn 0.5s ease-out;
        }

        .item.highlight {
          animation: highlight 0.5s ease-out;
        }
      `}</style>
      <Narrative>
        {getNarrative()}
      </Narrative>
      <Choices choices={choices} />
    </>
  );
};