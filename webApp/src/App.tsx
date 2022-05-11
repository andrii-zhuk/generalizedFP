import React, { useEffect, useRef } from "react";
import graph_static from "../../static/result_graph.json";
// import algorithm_static from "../../static/result_graph2.json";
import { DirectedGraph } from "./types/DirectedGraph";
import DisplayGraph from "./components/DisplayGraph/DisplayGraph";
import useToolbox from "./components/Toolbox/Toolbox";
import Box from "@mui/material/Box";
import Container from "@mui/material/Container";
import { AlgorithmStep, Algorithm } from "./types/Algorithm";
graph_static as any;
// algorithm_static as any;

function App() {
  const [directedGraph, setDirectedGraph] =
    React.useState<DirectedGraph | null>(null);
  const [algorithm, setAlgorithm] = React.useState<Algorithm | null>(null);
  const [algorithmStep, setAlgorithmStep] = React.useState<number>(0);

  const stepType =
    algorithmStep === 0
      ? AlgorithmStep.AlgorithmStart
      : AlgorithmStep.FindCycles;
  // algorithm === null
  //   ? null
  //   : algorithm.steps.length === 0
  //   ? null
  //   : algorithm.steps[algorithmStep].step_type;
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
    const graph_response = await fetch("../../static/result_graph.json");
    if (graph_response.ok) {
      const graph = await graph_response.json();
      setDirectedGraph(graph);
    } else {
      console.log("Error while loading graph");
      return;
    }
    const algorithm_response = await fetch("../../static/result_graph.json");
    if (algorithm_response.ok) {
      const algorithm = await algorithm_response.json();
      setAlgorithm(algorithm);
      setAlgorithmStep(0);
      expandAlgorithmInfo();
    } else {
      console.log("Error while loading algorithm result");
      return;
    }
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
          algorithm={null}
          step={algorithmStep}
        />
      </Container>
    </Box>
  );
}

export default App;
