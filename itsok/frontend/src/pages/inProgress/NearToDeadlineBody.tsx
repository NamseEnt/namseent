import { Box, Button, Divider, Slider, TextField } from "@mui/material";
import React from "react";
import { Typo } from "../../components/typography";
import { Space } from "../../components/Space";

export const NearToDeadlineBody: React.FC = () => {
  return (
    <Box>
      <Typo.H5>마무리 해볼까요?</Typo.H5>
      <Typo.Body1>
        서포터가 늦지 않게 확인할 수 있도록 3일 안에 꼭 응답해주세요.
      </Typo.Body1>
      <Space />
      <Divider variant="middle" />

      <Box m={2}>
        <Typo.H6>1. 『웹 디자인』 - 잘 진행하셨나요?</Typo.H6>
        <Box m={2}>
          <Typo.Body1>한번 점수를 매겨봅시다.</Typo.Body1>
          <Space />
          <Slider
            defaultValue={20}
            step={1}
            valueLabelDisplay="auto"
            marks={[
              {
                value: 0,
                label: "0점",
              },
              {
                value: 100,
                label: "100점",
              },
            ]}
          />
        </Box>
      </Box>

      <Box
        sx={{
          m: 2,
        }}
      >
        <Typo.H6>2. 어떻게 진행했었는지 한번 회고해볼까요?</Typo.H6>
        <Box m={2}>
          <Typo.Body1>
            무슨 말을 써야할지 모르겠다면 아래를 참고해보세요!
          </Typo.Body1>
          <Box ml={2}>
            <Typo.Body2>- 중간에 인상에 깊이 남은 일은?</Typo.Body2>
            <Typo.Body2>- 하면서 힘들었던 것은?</Typo.Body2>
            <Typo.Body2>- 잘했다고 느낀 점은?</Typo.Body2>
            <Typo.Body2>
              - 다음에 또 비슷한 것을 한다면 어떻게 하고 싶은지?
            </Typo.Body2>
          </Box>
          <Space />
          <TextField
            multiline
            fullWidth
            minRows={4}
            placeholder="회고를 입력하기"
            spellCheck={false}
          />
        </Box>
      </Box>

      <Box textAlign="right">
        <Button variant="contained">제출하기</Button>
      </Box>
    </Box>
  );
};
