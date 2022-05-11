import { GraphData } from "force-graph";
import { DirectedGraph } from "./DirectedGraph";

export function getGraph(g: DirectedGraph | null): GraphData {
  if (g == null) {
    return {
      nodes: [],
      links: [],
    };
  }
  const nodes = g.nodes.map(({ label }) => ({
    id: label,
  }));
  const links = g.edges_list.map(({ from_id, to_id }) => {
    return { source: g.nodes[from_id].label, target: g.nodes[to_id].label };
  });
  return {
    nodes,
    links,
  };
}
