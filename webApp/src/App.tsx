import React, { useEffect } from "react";
import { DirectedGraph } from "./types/DirectedGraph";
import DisplayGraph from "./components/DisplayGraph/DisplayGraph";
import useToolbox from "./components/Toolbox/Toolbox";
import Box from "@mui/material/Box";
import { Algorithm } from "./types/Algorithm";
import useAlgorithmInfoView from "./components/Toolbox/AlgorithmInfoView";
import useWindowSize from "./components/WindowSize/windowSizeHook";
import useInputGraph from "./components/Toolbox/InputGraph";

const rust = import("../pkg");

function App() {
  const windowSize = useWindowSize();

  const [directedGraph, setDirectedGraph] = React.useState<DirectedGraph>(null);
  const [algorithm, setAlgorithm] = React.useState<Algorithm>(null);
  const [algorithmStep, setAlgorithmStep] = React.useState<number>(0);
  const updateGraphFromText = async (graph_input: string) => {
    setGraphText(graph_input);
    await rust
      .then((m: any) => {
        const result = JSON.parse(m.find_generalized_flow(graph_input));
        if (result.error !== "") {
          console.log("Error in graph parser/algorithm");
        } else {
          setAlgorithmStep(0);
          setDirectedGraph(result.initial_graph);
          setAlgorithm(result.algorithm_steps);
          expandAlgorithmInfo();
        }
      })
      .catch(console.error);
  };

  const stepType =
    algorithm === null
      ? null
      : algorithm.steps.length === 0
      ? null
      : algorithm.steps[algorithmStep].step_type;
  const { expandAlgorithmInfo, algorithmInfoButton, algorithmInfo } =
    useAlgorithmInfoView(
      Math.min(windowSize.width / 4, 350),
      stepType,
      () => {
        setAlgorithmStep(0);
      },
      () => {},
      () => {
        setAlgorithmStep(algorithmStep + 1);
      }
    );

  const { setGraphText, expandInputGraph, inputGraphSection } = useInputGraph(
    Math.min(windowSize.width / 3, 500),
    updateGraphFromText
  );
  const toolbox = useToolbox(
    updateGraphFromText,
    algorithmInfoButton,
    expandInputGraph
  );

  const getData = async () => {
    const example_graph_input = `
    7 1 4
    1 2 5 2.0
    1 3 10 0.005
    3 12 15 2.0
    12 14 5 0.9
    14 3 4 2.0
    14 4 50 1.0
    2 4 8 1.0`;
    await updateGraphFromText(example_graph_input);
  };
  useEffect(() => {
    getData();
  }, []);

  return (
    <Box sx={{ maxHeight: "100%" }}>
      {toolbox}
      {algorithmInfo}
      {inputGraphSection}
      <input
        onChange={(event: React.ChangeEvent<HTMLInputElement>): void => {
          const file = event.target.files[0];
          if (!file) {
            return;
          }
          const reader = new FileReader();
          reader.onload = function (event) {
            updateGraphFromText(event.target.result.toString());
          };
          reader.readAsText(file);
        }}
        id="upload-graph-menu-option"
        type="file"
        multiple={false}
        hidden
      />
      <DisplayGraph
        graph={directedGraph}
        algorithm={algorithm}
        step={algorithmStep}
        width={windowSize.width}
        height={windowSize.height}
      />
    </Box>
  );
}

export default App;
