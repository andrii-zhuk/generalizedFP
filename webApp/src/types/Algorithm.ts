export enum AlgorithmStep {
  AlgorithmStart = "0",
  HasAugmentingPath = "1",
  FindCycles = "2",
  CancelCycles = "3",
  FindPath = "4",
  AlgorithmEnd = "5",
}

export interface AffectedNode {
  node_id: number;
  excess_amount: number;
}

export interface AffectedEdge {
  edge_id: number;
  flow_amount: number;
}

export interface Action {
  step_type: AlgorithmStep;
  result: boolean;
  pushed_flow: number;
  nodes_affected: AffectedNode[];
  edges_affected: AffectedEdge[];
}

export interface Algorithm {
  steps: Array<Action>;
}
