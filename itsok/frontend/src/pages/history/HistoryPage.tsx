import { Box } from "@mui/material";
import React, { useEffect } from "react";
import { Typo } from "../../components/typography";
import { Space } from "../../components/Space";
import { HistoryCard } from "./HistoryCard";

export const HistoryPage: React.FC = () => {
  const cards = [1, 2, 3];

  const cardList = cards.map((_card, index) => {
    const isLastCard = cards.length - 1 === index;
    return (
      <React.Fragment key={index}>
        <IntersectObservedHistoryCard
          onIntersect={() => {
            if (!isLastCard) {
              return;
            }

            console.log("last card intersected");
          }}
        ></IntersectObservedHistoryCard>

        <Space />
      </React.Fragment>
    );
  });

  return (
    <Box>
      <Typo.H4 align="center">지난 과거 돌아보기</Typo.H4>
      스크롤 기능을 넣어서 카드를 여러개 보여줄 수 있었으면 좋겠어. 지금 당장
      무한 스크롤이 필요하진 않을 수 있지.
      <Space />
      {cardList}
    </Box>
  );
};

const IntersectObservedHistoryCard: React.FC<{
  onIntersect: () => void;
}> = ({ onIntersect }) => {
  const onIntersectRef = React.useRef(onIntersect);
  useEffect(() => {
    onIntersectRef.current = onIntersect;
  }, [onIntersect]);

  const intersectObserver = React.useMemo(
    () =>
      new IntersectionObserver((entries) => {
        if (entries.length >= 2) {
          throw new Error("entries.length should be less than 2");
        }
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            onIntersectRef.current();
          }
        });
      }),
    [],
  );

  return (
    <HistoryCard
      ref={(element) => {
        if (element) {
          intersectObserver.observe(element);
        }
      }}
    />
  );
};
