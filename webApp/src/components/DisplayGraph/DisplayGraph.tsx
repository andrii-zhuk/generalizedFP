import React, { useEffect, useRef } from "react";
import { ForceGraph2D } from "react-force-graph";
import { genRandomTree } from "../../datasets/random-data";
import { DirectedGraph } from "../../types/DirectedGraph";
import { getGraph } from "../../types/HighlightedGraph";
import {
  excessChangedNodeCanvas,
  defaultNodeCanvas,
  inactiveNodeCanvas,
  sinkNodeCanvas,
  sourceNodeCanvas,
} from "../CanvasObjects/nodeCanvas";
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
  const [graphData, setGraphData] = React.useState<{
    data: GraphData;
    nodeTranslator: Map<string, number>;
    linkTranslator: Map<string, number>;
  }>(getGraph(null));
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
  const [excessGain, setExcessGain] = React.useState(new Map());

  const [highlightLinks, setHighlightLinks] = React.useState(new Set());

  function stringify(link: LinkObject): string {
    const src = link.source;
    const trg = link.target;
    const source_id =
      typeof src === "number" || typeof src === "string"
        ? src.toString()
        : src.id.toString();
    const target_id =
      typeof trg === "number" || typeof trg === "string"
        ? trg.toString()
        : trg.id.toString();
    return JSON.stringify({ source: source_id, target: target_id });
  }

  function isHighlightedLink(link: LinkObject): boolean {
    return highlightLinks.has(stringify(link));
  }

  function isHighlightedNode(node: NodeObject): boolean {
    return highlightNodes.has(node.id.toString());
  }

  const updateHighlight = () => {
    setHighlightNodes(highlightNodes);
    setHighlightLinks(highlightLinks);
    setExcessGain(excessGain);
  };

  if (props.step !== prevStep && props.step !== null) {
    if (directedGraph !== null && algorithm !== null) {
      highlightNodes.clear();
      highlightLinks.clear();
      excessGain.clear();

      algorithm.steps[props.step].nodes_affected.forEach((node) => {
        directedGraph.nodes[node.node_id].excess += node.excess_amount;
        highlightNodes.add(directedGraph.nodes[node.node_id].label);
        excessGain.set(
          directedGraph.nodes[node.node_id].label,
          node.excess_amount
        );
      });
      algorithm.steps[props.step].edges_affected.forEach((e) => {
        const edge = directedGraph.edges_list[e.edge_id];
        edge.flow += e.flow_amount;
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
      graphData={graphData.data}
      // height={props.height}
      // width={props.width}
      linkWidth={(link) => (isHighlightedLink(link) ? 4 : 2)}
      linkDirectionalParticles={4}
      linkDirectionalParticleWidth={(link) => (isHighlightedLink(link) ? 4 : 0)}
      linkCurvature={0.08}
      linkDirectionalArrowLength={3}
      linkLabel={(link: LinkObject) => {
        const id = graphData.linkTranslator.get(stringify(link));
        if (id !== undefined) {
          const { flow, capacity, amplification } =
            directedGraph.edges_list[id];
          if (capacity !== 0) return `${flow}/${capacity} x ${amplification}`;
        }
        return "";
      }}
      linkCanvasObject={(
        link: LinkObject,
        ctx: CanvasRenderingContext2D,
        globalScale: number
      ) => {
        // todo()
      }}
      linkCanvasObjectMode="after"
      // warmupTicks={5000}
      nodeCanvasObject={(
        node: NodeObject,
        ctx: CanvasRenderingContext2D,
        globalScale: number
      ) => {
        const id = graphData.nodeTranslator.get(node.id.toString());
        if (id === directedGraph.source) {
          sourceNodeCanvas(node, ctx, globalScale);
          return;
        }
        if (id === directedGraph.sink) {
          sinkNodeCanvas(node, ctx, globalScale);
          return;
        }
        if (!directedGraph.nodes[id].reachable_from_source) {
          inactiveNodeCanvas(node, ctx, globalScale);
          return;
        }
        if (isHighlightedNode(node)) {
          const excess = excessGain.get(node.id);
          // otherwise we will use default canvas
          if (excess !== undefined) {
            excessChangedNodeCanvas(node, ctx, globalScale, excess);
            return;
          }
        }
        defaultNodeCanvas(node, ctx, globalScale);
      }}
      nodeLabel={(node: NodeObject) => {
        const id = graphData.nodeTranslator.get(node.id.toString());
        if (id === directedGraph.source) {
          return "Source";
        }
        if (id === directedGraph.sink) {
          return "Sink";
        }
        if (!directedGraph.nodes[id].reachable_from_source) return "";
        return `Excess: ${directedGraph.nodes[id].excess}`;
      }}
    />
  );
}
