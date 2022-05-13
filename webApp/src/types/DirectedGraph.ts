export interface Edge {
  from_id: number;
  to_id: number;
  flow: number;
  capacity: number;
  amplification: number;
}

export interface Node {
  id: number;
  label: string;
  excess: number;
  reachable_from_source: boolean;
}

export interface DirectedGraph {
  sink: number;
  source: number;
  nodes: Array<Node>;
  adj_lists: Array<number[]>;
  edges_list: Array<Edge>;
  reverse_edge_ids: Array<number>;
}
