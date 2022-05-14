import * as React from "react";
import IconButton from "@mui/material/IconButton";
import MenuIcon from "@mui/icons-material/Menu";
import Menu from "@mui/material/Menu";
import MenuItem from "@mui/material/MenuItem";
import { getRandomGraph } from "../../datasets/random-data";

export default function MenuDropdown(
  updateGraph: (graph_input: string) => void,
  expandInputGraph: () => void
): [JSX.Element, JSX.Element] {
  const [anchorEl, setAnchorEl] = React.useState<null | HTMLElement>(null);
  const open = Boolean(anchorEl);
  const handleClick = (event: React.MouseEvent<HTMLButtonElement>) => {
    setAnchorEl(event.currentTarget);
  };
  const handleClose = () => {
    setAnchorEl(null);
  };

  return [
    <IconButton
      size="large"
      edge="start"
      color="inherit"
      aria-label="menu"
      sx={{ mr: 2 }}
      id="basic-button"
      aria-controls={open ? "basic-menu" : undefined}
      aria-haspopup="true"
      aria-expanded={open ? "true" : undefined}
      onClick={handleClick}
    >
      <MenuIcon />
    </IconButton>,
    <Menu
      id="basic-menu"
      anchorEl={anchorEl}
      open={open}
      onClose={handleClose}
      MenuListProps={{
        "aria-labelledby": "basic-button",
      }}
    >
      <MenuItem key={0}>
        <label onClick={handleClose} htmlFor="upload-graph-menu-option">
          Import graph
        </label>
      </MenuItem>
      <MenuItem
        key={1}
        onClick={() => {
          handleClose();
          const N = Math.round(Math.random() * 3 + 7);
          const graph = getRandomGraph(N);
          updateGraph(graph);
        }}
      >
        Random graph
      </MenuItem>
      <MenuItem
        key={2}
        onClick={() => {
          handleClose();
          expandInputGraph();
        }}
      >
        Describe graph
      </MenuItem>
      <MenuItem key={3} onClick={handleClose}>
        Guide
      </MenuItem>
    </Menu>,
  ];
}
