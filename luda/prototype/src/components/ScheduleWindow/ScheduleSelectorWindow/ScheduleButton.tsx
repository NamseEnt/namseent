import { Card, CardContent, Grid, Typography } from "@material-ui/core";
import React from "react";

type ScheduleButtonProps = {
  title: string;
  content: string;
  thumbnail: string;
  clickHandler?: () => void;
};

export default function ScheduleButton(props: ScheduleButtonProps) {
  const { title, content, thumbnail, clickHandler } = props;
  console.log(thumbnail);
  return (
    <Grid item xs={6}>
      <Card
        onClick={clickHandler}
        style={{
          cursor: clickHandler ? "pointer" : "default",
          backgroundImage: `url(${thumbnail})`,
          backgroundRepeat: "no-repeat",
          backgroundSize: "contain",
          backgroundOrigin: "border-box",
        }}
      >
        <CardContent
          style={{
            marginLeft: "30%",
            background:
              "linear-gradient(to right, rgba(255, 255, 255, 0), #FFF 20%",
          }}
        >
          <Typography variant="h5">{title}</Typography>
          <Typography color="textSecondary">{content}</Typography>
        </CardContent>
      </Card>
    </Grid>
  );
}
