import { GraphData } from "force-graph";
import { DirectedGraph } from "./DirectedGraph";

export function getGraph(g: DirectedGraph | null): {
  data: GraphData;
  nodeTranslator: Map<string, number>;
  linkTranslator: Map<string, number>;
} {
  const nodeTranslator = new Map<string, number>();
  const linkTranslator = new Map<string, number>();
  if (g == null) {
    return {
      data: {
        nodes: [],
        links: [],
      },
      nodeTranslator,
      linkTranslator,
    };
  }
  const nodes = g.nodes.map(({ id, label }) => {
    nodeTranslator.set(label, id);
    return {
      id: label,
    };
  });
  const links = g.edges_list.map(({ from_id, to_id }, edge_id) => {
    const result = {
      source: g.nodes[from_id].label,
      target: g.nodes[to_id].label,
    };
    linkTranslator.set(JSON.stringify(result), edge_id);
    return result;
  });
  return {
    data: {
      nodes,
      links,
    },
    nodeTranslator,
    linkTranslator,
  };
}
