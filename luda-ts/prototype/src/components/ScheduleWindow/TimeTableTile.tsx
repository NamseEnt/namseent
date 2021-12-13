import { Badge, Card, CardContent, Typography } from "@material-ui/core";
import React from "react";

type TimeTableTileProps = {
  row: number;
  column: number;
  duration: number;
  title: string;
  content: string;
  clickHandler?: () => void;
  badge?: string;
  disabled?: boolean;
  thumbnail?: string;
};

const stripes = `repeating-linear-gradient(
  -45deg,
  rgba(255, 255, 255, 0),
  rgba(255, 255, 255, 0) 10px,
  #000000 10px,
  #000000 20px
)`;

export default function TimeTableTile(props: TimeTableTileProps) {
  const {
    row,
    column,
    duration,
    title,
    content,
    clickHandler,
    badge,
    disabled,
    thumbnail,
  } = props;
  return (
    <Card
      style={{
        gridRowStart: row,
        gridRowEnd: row + duration,
        gridColumnStart: column,
        gridColumnEnd: column + 1,
        cursor: clickHandler && !disabled ? "pointer" : "default",
        position: "relative",
        backgroundImage: thumbnail ? `url(${thumbnail})` : undefined,
        backgroundRepeat: "no-repeat",
        backgroundSize: "contain",
        backgroundOrigin: "border-box",
      }}
      onClick={disabled ? undefined : clickHandler}
    >
      <CardContent
        style={{
          marginLeft: thumbnail ? "4rem" : "0px",
          background:
            "linear-gradient(to right, rgba(255, 255, 255, 0), #FFF 20%",
        }}
      >
        <Typography variant="h5" align="center">
          <Badge invisible={!badge} badgeContent={badge} color="primary">
            {title}
          </Badge>
        </Typography>
        <Typography color="textSecondary">{content}</Typography>
      </CardContent>
      <div
        style={{
          display: disabled ? "block" : "none",
          position: "absolute",
          top: 0,
          right: 0,
          bottom: 0,
          left: 0,
          background: stripes,
          opacity: 0.1,
        }}
      ></div>
    </Card>
  );
}
