import React, { useEffect, useRef } from "react";
import { ForceGraph2D } from "react-force-graph";
import { genRandomTree } from "../../datasets/random-data";
import { DirectedGraph } from "../../types/DirectedGraph";
import { getGraph } from "../../types/HighlightedGraph";
import { getNodeCanvas } from "../CanvasObjects/nodeCanvas";

export default function useDisplayGraph(props: {
  // height: number;
  // width: number;
  graph: DirectedGraph | null;
  algorithm: Algorithm | null;
  step: number;
}) {
  const [directedGraph, setDirectedGraph] =
    React.useState<DirectedGraph | null>(null);

  React.useEffect(() => {}, [props.graph, props.algorithm]);

  return (
    <ForceGraph2D
      graphData={getGraph(directedGraph)}
      // height={props.height}
      // width={props.width}
      nodeLabel={(node: any) => node.id}
      nodeVisibility={true}
      nodeCanvasObject={getNodeCanvas}
    />
  );
}
