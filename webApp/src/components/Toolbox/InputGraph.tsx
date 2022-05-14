import * as React from "react";
import { styled, useTheme } from "@mui/material/styles";
import IconButton from "@mui/material/IconButton";
import ChevronLeftIcon from "@mui/icons-material/ChevronLeft";
import ChevronRightIcon from "@mui/icons-material/ChevronRight";
import Drawer from "@mui/material/Drawer";
import Button from "@mui/material/Button";
import { Box, Container, Grid, TextField } from "@mui/material";
import SendIcon from "@mui/icons-material/Send";

const DrawerHeader = styled("div")(({ theme }) => ({
  display: "flex",
  alignItems: "center",
  padding: theme.spacing(0, 1),
  ...theme.mixins.toolbar,
  justifyContent: "flex-end",
}));

export default function InputGraph(
  width: number,
  updateGraph: (graph_input: string) => void
): {
  expandInputGraph: () => void;
  inputGraphSection: JSX.Element;
  setGraphText: React.Dispatch<React.SetStateAction<string>>;
} {
  const theme = useTheme();
  const [open, setOpen] = React.useState<boolean>(false);
  const [inputText, setInputText] = React.useState<string>("");

  const handleDrawerOpen = () => {
    setOpen(true);
  };

  const handleDrawerClose = () => {
    setOpen(false);
  };

  return {
    expandInputGraph: handleDrawerOpen,
    setGraphText: setInputText,
    inputGraphSection: (
      <Drawer
        sx={{
          width,
          flexShrink: 0,
          "& .MuiDrawer-paper": {
            width,
          },
        }}
        variant="temporary"
        anchor="left"
        open={open}
      >
        <Box
          sx={{
            display: "flex",
            flexDirection: "column",
            p: 1,
            m: 1,
            bgcolor: "background.paper",
            borderRadius: 1,
            height: 1,
          }}
        >
          <DrawerHeader>
            <IconButton onClick={handleDrawerClose}>
              {theme.direction === "rtl" ? (
                <ChevronRightIcon />
              ) : (
                <ChevronLeftIcon />
              )}
            </IconButton>
          </DrawerHeader>
          <Box sx={{ width: 1, height: 1, maxHeight: 1 }}>
            <TextField
              id="input-graph-text-field"
              multiline
              value={inputText}
              maxRows={32}
              fullWidth
              onChange={(event) => {
                setInputText(event.target.value);
              }}
            />
          </Box>
          <Box
            width="100%"
            sx={{
              display: "flex",
              justifyContent: "flex-end",
              paddingTop: 2,
            }}
          >
            <Button
              variant="contained"
              disabled={false}
              onClick={() => {
                updateGraph(inputText);
                handleDrawerClose();
              }}
              endIcon={<SendIcon />}
            >
              Draw graph
            </Button>
          </Box>
        </Box>
      </Drawer>
    ),
  };
}
