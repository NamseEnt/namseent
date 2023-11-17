import { Box, Divider, Grow } from "@mui/material";
import React, { useEffect, useMemo } from "react";
import { Typo } from "../components/typography";

export const ResponsePage: React.FC = () => {
  const startTime = useMemo(() => Date.now(), []);
  const [timeDiffMs, setTimeDiffMs] = React.useState(0);
  const content = `
    저희 팀에서는 당신이 일주일 동안 웹디자인 작업을 훌륭하게 수행해주셨음에 감사드립니다. 여러 어려운 과제들을 효과적으로 해결하고, 창의적이고 혁신적인 디자인 아이디어로 프로젝트에 새로운 빛을 불어넣어 주셨습니다.

  당신의 전문성과 헌신적인 태도는 팀 전체에 영감을 주었고, 결과물은 예상을 뛰어넘는 수준의 완성도를 보여주었습니다. 당신의 기술적 능력과 커뮤니케이션 기술은 프로젝트의 원활한 진행에 큰 기여를 했습니다.

  우리는 당신이 함께 일할 수 있어서 행운이었고, 미래에도 높은 수준의 협업을 기대하고 있습니다. 당신의 뛰어난 기여에 다시 한번 감사의 인사를 드립니다.

  좋은 작업에 감사드리며, 앞으로의 프로젝트에서도 함께 성장해 나가길 기대합니다.

  감사합니다.

  좋은 하루 되세요!`;

  useEffect(() => {
    let id = requestAnimationFrame(function tick() {
      setTimeDiffMs(Date.now() - startTime);
      id = requestAnimationFrame(tick);
    });

    return () => {
      cancelAnimationFrame(id);
    };
  }, [startTime]);

  const contentComponents = content
    .split(" ")
    .flatMap((x) => [x, " "])
    .slice(0, -1)
    .flatMap((x) =>
      x
        .split("\n")
        .flatMap((y) => [y, "\n"])
        .slice(0, -1),
    )
    .flatMap((word, index) => {
      if (word === "\n") {
        return <br key={index} />;
      }
      const span = (word: string, key?: React.Key) => (
        <span
          key={key}
          style={{
            display: "inline-block",
            whiteSpace: "pre-wrap",
          }}
        >
          {word}
        </span>
      );
      if (word === " ") {
        return span(word, index);
      }

      const timeForOneWord = 100;
      const timeGap = timeDiffMs - index * timeForOneWord;
      const isWordIn = timeGap > 0;
      return word.split("").map((letter, letterIndex) => {
        const isLetterIn =
          isWordIn && timeGap > timeForOneWord * (letterIndex / word.length);
        return (
          <Grow key={`${index}-${letterIndex}`} in={isLetterIn} timeout={1000}>
            {span(letter)}
          </Grow>
        );
      });
    });
  return (
    <Box>
      <Typo.H4 align="center">고생하셨습니다!</Typo.H4>
      <Typo.H6 align="center">『웹 디자인』</Typo.H6>
      <Divider variant="middle" />
      <Box m={4}>{contentComponents}</Box>
    </Box>
  );
};
