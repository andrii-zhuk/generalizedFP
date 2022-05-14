import React from "react";
import { ForceGraph2D } from "react-force-graph";
import { DirectedGraph } from "../../types/DirectedGraph";
import {
  getGraph,
  stringifyLink,
  stringifyNode,
} from "../../types/HighlightedGraph";
import {
  excessChangedNodeCanvas,
  defaultNodeCanvas,
  inactiveNodeCanvas,
  sinkNodeCanvas,
  sourceNodeCanvas,
} from "../CanvasObjects/nodeCanvas";
import { Algorithm } from "../../types/Algorithm";
import { GraphData, LinkObject, NodeObject } from "force-graph";
import { defaultLinkCanvas } from "../CanvasObjects/linkCanvas";
export default function useDisplayGraph(props: {
  height: number;
  width: number;
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

  const [highlightNodes, setHighlightNodes] = React.useState(
    new Map<number, number>()
  );

  const [highlightLinks, setHighlightLinks] = React.useState(
    new Map<number, { flow: number; amplification: number }>()
  );
  const updateHighlight = () => {
    setHighlightNodes(highlightNodes);
    setHighlightLinks(highlightLinks);
  };

  React.useEffect(() => {
    setDirectedGraph(props.graph);
    setGraphData(getGraph(props.graph));
  }, [props.graph]);

  if (props.algorithm !== algorithm) {
    highlightNodes.clear();
    highlightLinks.clear();
    updateHighlight();
    setAlgorithm(props.algorithm);
  }

  function isHighlightedLink(link: LinkObject): boolean {
    const id = graphData.linkTranslator.get(stringifyLink(link));
    if (id !== undefined) {
      return highlightLinks.has(id);
    }
    return false;
  }

  function isHighlightedNode(node: NodeObject): boolean {
    const id = graphData.nodeTranslator.get(stringifyNode(node));
    if (id !== undefined) {
      return highlightNodes.has(id);
    }
    return false;
  }

  if (props.step !== prevStep && props.step !== null) {
    if (directedGraph !== null && algorithm !== null) {
      highlightNodes.clear();
      highlightLinks.clear();

      if (props.step === 0) {
        directedGraph.nodes.forEach((node) => {
          node.excess = 0.0;
        });
        directedGraph.edges_list.forEach((edge) => {
          edge.flow = 0.0;
        });
      } else {
        algorithm.steps[props.step].nodes_affected.forEach((node) => {
          directedGraph.nodes[node.node_id].excess += node.excess_amount;
          highlightNodes.set(node.node_id, node.excess_amount);
        });
        algorithm.steps[props.step].edges_affected.forEach((e) => {
          const edge = directedGraph.edges_list[e.edge_id];
          edge.flow += e.flow_amount;

          const reverse_edge_id = directedGraph.reverse_edge_ids[e.edge_id];
          const reverse_edge = directedGraph.edges_list[reverse_edge_id];
          reverse_edge.flow -= e.flow_amount * edge.amplification;

          highlightLinks.set(e.edge_id, {
            flow: e.flow_amount,
            amplification: edge.amplification,
          });
        });
      }
      updateHighlight();
      setPrevStep(props.step);
    }
  }

  return (
    <ForceGraph2D
      ref={forceGraphRef}
      graphData={graphData.data}
      height={props.height}
      width={props.width}
      linkWidth={(link) => (isHighlightedLink(link) ? 4 : 2)}
      linkLineDash={(link) => {
        const id = graphData.linkTranslator.get(stringifyLink(link));
        if (id !== undefined) {
          const { capacity } = directedGraph.edges_list[id];
          if (capacity !== 0) {
            return [0];
          }
        }
        return [1];
      }}
      linkDirectionalParticles={4}
      linkDirectionalParticleWidth={(link) => (isHighlightedLink(link) ? 4 : 0)}
      linkCurvature={0.08}
      linkDirectionalArrowLength={3}
      linkLabel={(link: LinkObject) => {
        const id = graphData.linkTranslator.get(stringifyLink(link));
        if (id !== undefined) {
          const { flow, capacity, amplification } =
            directedGraph.edges_list[id];
          if (capacity !== 0) {
            return `${flow}/${capacity} x ${amplification}`;
          }
        }
        return "";
      }}
      linkCanvasObject={(
        link: LinkObject,
        ctx: CanvasRenderingContext2D,
        globalScale: number
      ) => {
        const id = graphData.linkTranslator.get(stringifyLink(link));
        if (id !== undefined) {
          const edge = highlightLinks.get(id);
          if (edge !== undefined) {
            defaultLinkCanvas(
              link,
              ctx,
              globalScale,
              edge.flow,
              edge.amplification
            );
          }
        }
      }}
      linkCanvasObjectMode={() => "after"}
      // warmupTicks={5000}
      nodeCanvasObject={(
        node: NodeObject,
        ctx: CanvasRenderingContext2D,
        globalScale: number
      ) => {
        const id = graphData.nodeTranslator.get(stringifyNode(node));
        if (id === directedGraph.source) {
          const excess = highlightNodes.get(id);
          if (excess !== undefined) {
            sourceNodeCanvas(node, ctx, globalScale, excess);
          } else {
            sourceNodeCanvas(node, ctx, globalScale);
          }
          return;
        }
        if (id === directedGraph.sink) {
          const excess = highlightNodes.get(id);
          if (excess !== undefined) {
            sinkNodeCanvas(node, ctx, globalScale, excess);
          } else {
            sinkNodeCanvas(node, ctx, globalScale);
          }
          return;
        }
        if (!directedGraph.nodes[id].reachable_from_source) {
          inactiveNodeCanvas(node, ctx, globalScale);
          return;
        }
        if (isHighlightedNode(node)) {
          const excess = highlightNodes.get(id);
          // otherwise we will use default canvas
          if (excess !== undefined) {
            excessChangedNodeCanvas(node, ctx, globalScale, excess);
            return;
          }
        }
        defaultNodeCanvas(node, ctx, globalScale);
      }}
      nodeLabel={(node: NodeObject) => {
        const id = graphData.nodeTranslator.get(stringifyNode(node));
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
