import React, { useEffect, useRef } from "react";
import graph_static from "../../static/result_graph.json";
import algorithm_static from "../../static/result_algorithm.json";
import { DirectedGraph } from "./types/DirectedGraph";
import DisplayGraph from "./components/DisplayGraph/DisplayGraph";
import useToolbox from "./components/Toolbox/Toolbox";
import Box from "@mui/material/Box";
import Container from "@mui/material/Container";
import { Algorithm } from "./types/Algorithm";

const rust = import("../pkg");

graph_static as any;
algorithm_static as any;

function App() {
  const [directedGraph, setDirectedGraph] = React.useState<DirectedGraph>(null);
  const [algorithm, setAlgorithm] = React.useState<Algorithm>(null);
  const [algorithmStep, setAlgorithmStep] = React.useState<number>(0);
  const updateGraphFromText = (graph_input: string) => {
    const result = JSON.parse(m.find_generalized_flow(graph_input));
    if (result.error !== "") {
      console.log("Error in graph parser/algorithm");
    } else {
      setDirectedGraph(result.initial_graph);
      setAlgorithm(result.algorithm_steps);
      setAlgorithmStep(0);
      expandAlgorithmInfo();
    }
  };
  const stepType =
    algorithm === null
      ? null
      : algorithm.steps.length === 0
      ? null
      : algorithm.steps[algorithmStep].step_type;
  const { expandAlgorithmInfo, toolbox } = useToolbox(
    stepType,
    () => {
      setAlgorithmStep(0);
    },
    () => {
      setAlgorithmStep(algorithmStep + 1);
    }
  );

  const getData = async () => {
    const graph_input = `8 1 4
    1 2 5 2.0
    1 3 10 0.005
    3 12 15 2.0
    12 14 5 0.9
    14 3 4 2.0
    14 4 50 1.0
    2 4 8 1.0
    16 4 100 1.9
    `;
    await rust
      .then((m) => {
        updateGraphFromText(graph_input);
      })
      .catch(console.error);
  };
  useEffect(() => {
    getData();
  }, []);

  return (
    <Box sx={{ maxHeight: "100%" }}>
      {toolbox}
      <Container>
        <DisplayGraph
          graph={directedGraph}
          algorithm={algorithm}
          step={algorithmStep}
        />
      </Container>
    </Box>
  );
}

export default App;
