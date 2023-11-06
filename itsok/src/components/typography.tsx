import { Typography } from "@mui/material";
import { Variant } from "@mui/material/styles/createTypography";
import React from "react";

function typography(
  variant: Variant,
): React.FC<Omit<React.ComponentProps<typeof Typography>, "variant">> {
  return (props) => <Typography variant={variant} {...props} />;
}

/* eslint-disable react-refresh/only-export-components */

export const H1 = typography("h1");
export const H2 = typography("h2");
export const H3 = typography("h3");
export const H4 = typography("h4");
export const H5 = typography("h5");
export const H6 = typography("h6");

export const Subtitle1 = typography("subtitle1");
export const Subtitle2 = typography("subtitle2");

export const Body1 = typography("body1");
export const Body2 = typography("body2");

export const Typo = {
  H1,
  H2,
  H3,
  H4,
  H5,
  H6,
  Subtitle1,
  Subtitle2,
  Body1,
  Body2,
};
