import * as React from "react";
import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import useMenuDropdown from "./MenuDropdown";

export default function ToolboxView(
  updateGraph: (graph_input: string) => void,
  algorithmInfoButton: JSX.Element,
  expandInputGraph: () => void
): JSX.Element {
  const menuDropdown = useMenuDropdown(updateGraph, expandInputGraph);

  return (
    <>
      <AppBar position="fixed">
        <Toolbar>
          {menuDropdown}
          <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
            Generalized FP
          </Typography>
          {algorithmInfoButton}
        </Toolbar>
      </AppBar>
    </>
  );
}
