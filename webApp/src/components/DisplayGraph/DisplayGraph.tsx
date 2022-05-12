import React, { useEffect, useRef } from "react";
import { ForceGraph2D } from "react-force-graph";
import { genRandomTree } from "../../datasets/random-data";
import { DirectedGraph } from "../../types/DirectedGraph";
import { getGraph } from "../../types/HighlightedGraph";
import { getNodeCanvas } from "../CanvasObjects/nodeCanvas";
import { Algorithm } from "../../types/Algorithm";
import { GraphData, LinkObject, NodeObject } from "force-graph";
export default function useDisplayGraph(props: {
  // height: number;
  // width: number;
  graph: DirectedGraph;
  algorithm: Algorithm;
  step: number;
}) {
  const [directedGraph, setDirectedGraph] = React.useState<DirectedGraph>(null);
  const [graphData, setGraphData] = React.useState<GraphData>(getGraph(null));
  const [algorithm, setAlgorithm] = React.useState<Algorithm>({ steps: [] });
  const [prevStep, setPrevStep] = React.useState<number>(null);
  const forceGraphRef = React.useRef();
  React.useEffect(() => {
    setDirectedGraph(props.graph);
    setGraphData(getGraph(props.graph));
  }, [props.graph]);
  React.useEffect(() => {
    setAlgorithm(props.algorithm);
  }, [props.algorithm]);

  const [highlightNodes, setHighlightNodes] = React.useState(new Set());
  const [highlightLinks, setHighlightLinks] = React.useState(new Set());

  const updateHighlight = () => {
    setHighlightNodes(highlightNodes);
    setHighlightLinks(highlightLinks);
  };

  if (props.step !== prevStep && props.step !== null) {
    if (directedGraph !== null && algorithm !== null) {
      highlightNodes.clear();
      highlightLinks.clear();

      algorithm.steps[props.step].nodes_affected.forEach((node) =>
        highlightNodes.add(directedGraph.nodes[node.node_id].label)
      );
      algorithm.steps[props.step].edges_affected.forEach((e) => {
        const edge = directedGraph.edges_list[e.edge_id];
        highlightLinks.add(
          JSON.stringify({
            source: directedGraph.nodes[edge.from_id].label,
            target: directedGraph.nodes[edge.to_id].label,
          })
        );
      });
      updateHighlight();
      setPrevStep(props.step);
    }
  }

  return (
    <ForceGraph2D
      ref={forceGraphRef}
      graphData={graphData}
      // height={props.height}
      // width={props.width}
      nodeLabel={(node: any) => node.id}
      nodeCanvasObject={getNodeCanvas}
      linkWidth={(link) =>
        highlightLinks.has(
          JSON.stringify({ source: link.source.id, target: link.target.id })
        )
          ? 5
          : 1
      }
      linkDirectionalParticles={4}
      linkDirectionalParticleWidth={(link) =>
        highlightLinks.has(
          JSON.stringify({ source: link.source.id, target: link.target.id })
        )
          ? 4
          : 0
      }
      nodeCanvasObjectMode={(node) =>
        highlightNodes.has(`${node.id}`) ? "replace" : "after"
      }
      linkCurvature={0.05}
      linkDirectionalArrowLength={3}
      warmupTicks={10000}
    />
  );
}
