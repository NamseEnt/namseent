import React, { useMemo } from 'react';

interface NarrativeProps {
  text?: string;
  children?: React.ReactNode;
}

export const Narrative: React.FC<NarrativeProps> = ({ text, children }) => {
  const lines = useMemo(() => text ? text.split('\n') : [], [text]);
  
  return (
    <div className="narrative">
      {children ? children : (
        lines.map((line, index) => (
          <React.Fragment key={index}>
            {line}
            {index < lines.length - 1 && <br />}
          </React.Fragment>
        ))
      )}
    </div>
  );
};