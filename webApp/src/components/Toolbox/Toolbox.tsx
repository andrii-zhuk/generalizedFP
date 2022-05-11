import * as React from "react";
import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import useMenuDropdown from "./MenuDropdown";
import useAlgorithmInfoView from "./AlgorithmInfoView";
import { AlgorithmStep } from "../../types/Algorithm";

export default function ToolboxView(
  currentAlgorithmStep: AlgorithmStep | null,
  prevStepButton: () => void,
  nextStepButton: () => void
): {
  expandAlgorithmInfo: () => void;
  toolbox: JSX.Element;
} {
  const menuDropdown = useMenuDropdown();
  const { expandAlgorithmInfo, algorithmInfoButton, algorithmInfo } =
    useAlgorithmInfoView(currentAlgorithmStep, prevStepButton, nextStepButton);
  return {
    expandAlgorithmInfo,
    toolbox: (
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
        {algorithmInfo}
      </>
    ),
  };
}
