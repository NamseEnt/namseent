import { Box, Button } from "@mui/material";
import Grid from "@mui/material/Unstable_Grid2";
import React from "react";

export const FarFromDeadlineBody: React.FC = () => {
  return (
    <Box>
      <Grid container spacing={2}>
        <Grid xs={6}>
          <Button size="medium" fullWidth variant="outlined" color="primary">
            잘 되가고 있어요!
          </Button>
        </Grid>
        <Grid xs={6}>
          <Button size="medium" fullWidth variant="outlined" color="primary">
            별로 신통치 못해요
          </Button>
        </Grid>
        <Grid xs={6}>
          <Button size="medium" fullWidth variant="outlined" color="primary">
            이미 다 끝냈어요
          </Button>
        </Grid>
        <Grid xs={6}>
          <Button size="medium" fullWidth variant="outlined" color="primary">
            문제가 생겼어요
          </Button>
        </Grid>
      </Grid>
    </Box>
  );
};
