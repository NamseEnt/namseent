import { ListItem, ListItemIcon, ListItemText } from "@material-ui/core";
import { CheckCircle, Done, Error, Info, Warning } from "@material-ui/icons";
import { VariantType } from "notistack";
import React from "react";
import { NotificationData } from "../../notification/type";

type NotificationItemProps = {
  notificationData: NotificationData;
};

function getIcon(variant?: VariantType) {
  switch (variant) {
    case "error": {
      return <Error color="error" />;
    }
    case "info": {
      return <Info color="primary" />;
    }
    case "success": {
      return <CheckCircle color="action" />;
    }
    case "warning": {
      return <Warning color="secondary" />;
    }
    default:
      return undefined;
  }
}

export default function NotificationItem(props: NotificationItemProps) {
  const { title, content, options } = props.notificationData;
  return (
    <ListItem>
      <ListItemIcon>{getIcon(options?.variant)}</ListItemIcon>
      <ListItemText primary={title} secondary={content} />
    </ListItem>
  );
}
