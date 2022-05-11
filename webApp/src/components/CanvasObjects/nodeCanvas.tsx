import { NodeObject } from "force-graph";

export function getNodeCanvas(
  node: NodeObject,
  ctx: CanvasRenderingContext2D,
  globalScale: number
) {
  const label = `${node.id}`;
  let fontSize = 15 / globalScale;
  ctx.font = `${fontSize}px Sans-Serif`;
  let textWidth = ctx.measureText(label).width;
  const bckgDimensions = [textWidth, fontSize].map((n) => n + fontSize * 0.2); // some padding
  // textWidth += fontSize * 0.2;
  // fontSize += fontSize * 0.2;

  ctx.fillStyle = "rgba(255, 255, 255, 0.8)";
  ctx.fillRect(
    node.x - bckgDimensions[0] / 2,
    node.y - bckgDimensions[1] / 2,
    textWidth,
    fontSize
  );

  ctx.textAlign = "center";
  ctx.textBaseline = "middle";
  ctx.fillStyle = "black";
  ctx.fillText(label, node.x, node.y);

  // node.__bckgDimensions = bckgDimensions; // to re-use in nodePointerAreaPaint
}
