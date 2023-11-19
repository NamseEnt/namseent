import { Box, Divider } from "@mui/material";
import React from "react";
import { FarFromDeadlineBody } from "./FarFromDeadlineBody";
import { NearToDeadlineBody } from "./NearToDeadlineBody";
import { Typo } from "../../components/typography";

export const InProgressPage: React.FC = () => {
  const body = parseInt("0") ? <NearToDeadlineBody /> : <FarFromDeadlineBody />;
  return (
    <Box>
      <Typo.H4 align="center">하는 일은 잘 되고 있나요?</Typo.H4>
      <Typo.H6 align="center">
        나는 『웹 디자인』을 하는 중... (7일 남음)
      </Typo.H6>
      <Divider variant="middle" />
      <Box m={4}>{body}</Box>
    </Box>
  );
};
