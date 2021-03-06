import * as React from "react";
import { styled, useTheme } from "@mui/material/styles";
import Divider from "@mui/material/Divider";
import IconButton from "@mui/material/IconButton";
import ChevronLeftIcon from "@mui/icons-material/ChevronLeft";
import ChevronRightIcon from "@mui/icons-material/ChevronRight";
import Drawer from "@mui/material/Drawer";
import TreeView from "@mui/lab/TreeView";
import TreeItem from "@mui/lab/TreeItem";
import ButtonGroup from "@mui/material/ButtonGroup";
import Button from "@mui/material/Button";
import { Container } from "@mui/material";
import { AlgorithmStep } from "../../types/Algorithm";

const DrawerHeader = styled("div")(({ theme }) => ({
  display: "flex",
  alignItems: "center",
  padding: theme.spacing(0, 1),
  ...theme.mixins.toolbar,
  justifyContent: "flex-start",
}));

export default function AlgorithmInfoView(
  width: number,
  selectedAlgorithmStep: AlgorithmStep | null,
  toStartStepButton: () => void,
  prevStepButton: () => void,
  nextStepButton: () => void
): {
  expandAlgorithmInfo: () => void;
  algorithmInfoButton: JSX.Element;
  algorithmInfo: JSX.Element;
} {
  const drawerWidth = width;

  const theme = useTheme();
  const [open, setOpen] = React.useState(false);

  const handleDrawerOpen = () => {
    setOpen(true);
  };

  const handleDrawerClose = () => {
    setOpen(false);
  };

  return {
    expandAlgorithmInfo: handleDrawerOpen,
    algorithmInfoButton: (
      <IconButton
        color="inherit"
        aria-label="open drawer"
        edge="end"
        onClick={handleDrawerOpen}
        sx={{ ...(open && { display: "none" }) }}
      >
        <ChevronLeftIcon />
      </IconButton>
    ),
    algorithmInfo: (
      <Drawer
        sx={{
          width: drawerWidth,
          flexShrink: 0,
          "& .MuiDrawer-paper": {
            width: drawerWidth,
          },
        }}
        variant="persistent"
        anchor="right"
        open={open}
      >
        <DrawerHeader>
          <IconButton onClick={handleDrawerClose}>
            {theme.direction === "rtl" ? (
              <ChevronLeftIcon />
            ) : (
              <ChevronRightIcon />
            )}
          </IconButton>
        </DrawerHeader>
        <Container>
          <ButtonGroup variant="contained">
            <Button
              disabled={
                selectedAlgorithmStep === null ||
                selectedAlgorithmStep === AlgorithmStep.AlgorithmStart
              }
              onClick={toStartStepButton}
            >
              To start
            </Button>
            <Button disabled={true} onClick={prevStepButton}>
              Prev step
            </Button>
            <Button
              disabled={
                selectedAlgorithmStep === null ||
                selectedAlgorithmStep === AlgorithmStep.AlgorithmEnd
              }
              onClick={nextStepButton}
            >
              Next step
            </Button>
          </ButtonGroup>
        </Container>
        <Container sx={{ paddingTop: "20", paddingBottom: "20" }}>
          <TreeView
            expanded={[
              AlgorithmStep.HasAugmentingPath,
              AlgorithmStep.FindCycles,
            ]}
            selected={[selectedAlgorithmStep]}
            sx={{ maxWidth: "100%" }}
          >
            <TreeItem
              nodeId={AlgorithmStep.AlgorithmStart}
              sx={{ paddingY: "2" }}
              label="Start of algorithm"
            />
            <TreeItem
              nodeId={AlgorithmStep.HasAugmentingPath}
              sx={{ paddingY: "2" }}
              label="while(hasAugmentingPath())"
            >
              <TreeItem
                nodeId={AlgorithmStep.FindCycles}
                sx={{ paddingY: "2" }}
                label="while(findCycle())"
              >
                <TreeItem
                  nodeId={AlgorithmStep.CancelCycles}
                  sx={{ paddingY: "2" }}
                  label="cancelCycle()"
                />
              </TreeItem>
              <TreeItem
                nodeId={AlgorithmStep.FindPath}
                sx={{ paddingY: "2" }}
                label="findWidestFlowPath()"
              />
            </TreeItem>
            <TreeItem
              nodeId={AlgorithmStep.AlgorithmEnd}
              sx={{ paddingY: "2" }}
              label="End of algorithm"
            />
          </TreeView>
        </Container>
        <Divider />
      </Drawer>
    ),
  };
}
