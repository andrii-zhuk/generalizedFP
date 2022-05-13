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
    const graph_response = await fetch("../../static/result_graph.json");
    if (graph_response.ok) {
      const graph = await graph_response.json();
      setDirectedGraph(graph);
    } else {
      console.log("Error while loading graph");
      return;
    }
    const algorithm_response = await fetch(
      "../../static/result_algorithm.json"
    );
    if (algorithm_response.ok) {
      const data = await algorithm_response.json();
      setAlgorithm(data);
      setAlgorithmStep(0);
      expandAlgorithmInfo();
    } else {
      console.log("Error while loading algorithm result");
      return;
    }
  };
  useEffect(() => {
    getData();
    rust.then((m) => console.log(m.add(4, 7))).catch(console.error);
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
