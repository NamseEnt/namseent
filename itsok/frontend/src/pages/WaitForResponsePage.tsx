import { Box, Divider } from "@mui/material";
import React from "react";
import { Typo } from "../components/typography";

export const WaitForResponsePage: React.FC = () => {
  return (
    <Box>
      <Typo.H4 align="center">고생하셨습니다!</Typo.H4>
      <Typo.H6 align="center">『웹 디자인』</Typo.H6>
      <Divider variant="middle" />
      <Box m={4}>
        <Typo.Body1 align="center">
          아직 서포터의 응답을 준비하고 있습니다.
        </Typo.Body1>
        <Typo.Body1 align="center">
          준비가 완료되면 알림 메시지를 보내드리겠습니다!
        </Typo.Body1>
      </Box>
    </Box>
  );
};
