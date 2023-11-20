import { Box, Button, Grid, Slider, TextField } from "@mui/material";
import React from "react";
import { Typo } from "../components/typography";
import { Space } from "../components/Space";

export const NewGoalPage: React.FC = () => {
  return (
    <Box>
      <Typo.H4 align="center">새로운 목표 정하기</Typo.H4>
      <Typo.H6 align="center">
        최대 30일 안에 할 수 있는 목표를 정해봅시다.
      </Typo.H6>
      <Box m={2}>
        <Typo.H6>1. 어떤 목표를 정해볼까요?</Typo.H6>
        <Box m={2}>
          <TextField
            fullWidth
            placeholder="예시) 책 한 권 다 읽기"
            spellCheck={false}
          />
        </Box>
      </Box>
      <Space />
      <Box m={2}>
        <Typo.H6>2. 목표를 정한 목적은 무엇인가요?</Typo.H6>
        <Box m={2}>
          <Typo.Body1>
            목적이 명확하면 명확할수록 목표를 달성하려는 힘이 강해집니다. 한번
            생각해보세요.
          </Typo.Body1>
          <Space />
          <TextField
            fullWidth
            multiline
            minRows={2}
            placeholder="예시) 책을 읽으면서 새로운 지식을 얻고, 책을 읽는 습관을 기르고 싶어요."
            spellCheck={false}
          />
        </Box>
      </Box>
      <Space />
      <Box m={2}>
        <Typo.H6>3. 며칠동안 진행할까요?</Typo.H6>
        <Box m={2}>
          <Typo.Body1>일찍 끝내도 괜찮습니다. 여유있게 잡아봅시다.</Typo.Body1>
          <Space />
          <Grid container spacing={2}>
            <Grid item>
              <Typo.Body1 textAlign="center">7 일 동안 진행</Typo.Body1>
            </Grid>
            <Grid item xs>
              <Slider
                defaultValue={7}
                step={1}
                valueLabelDisplay="auto"
                valueLabelFormat={(value) => `${value}일`}
                min={1}
                max={30}
              />
            </Grid>
          </Grid>
        </Box>
      </Box>
      <Space />
      <Box m={2} textAlign="right">
        <Button variant="contained">등록하기</Button>
      </Box>
    </Box>
  );
};
