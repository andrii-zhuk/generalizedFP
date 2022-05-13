import { GraphData, LinkObject, NodeObject } from "force-graph";
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

export function stringifyLink(link: LinkObject): string {
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

export function stringifyNode(node: NodeObject): string {
  return node.id.toString();
}
