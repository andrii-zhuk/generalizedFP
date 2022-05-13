import { NodeObject } from "force-graph";

function basicCanvas(
  node: NodeObject,
  ctx: CanvasRenderingContext2D,
  globalScale: number,
  label: string,
  bkgColor: string,
  textColor: string
) {
  let fontSize = 18 / globalScale;
  ctx.font = `${fontSize}px Sans-Serif`;
  let textWidth = ctx.measureText(label).width;
  const bckgDimensions = [textWidth, fontSize].map((n) => n + fontSize * 0.3); // some padding

  if (bkgColor != null) {
    ctx.fillStyle = bkgColor;
    ctx.fillRect(
      node.x - bckgDimensions[0] / 2,
      node.y - bckgDimensions[1] / 2,
      bckgDimensions[0],
      bckgDimensions[1]
    );
  }

  ctx.textAlign = "center";
  ctx.textBaseline = "middle";
  ctx.fillStyle = textColor;
  ctx.fillText(label, node.x, node.y);
}

function writeExcessLabelAbove(
  node: NodeObject,
  ctx: CanvasRenderingContext2D,
  globalScale: number,
  excess: number
) {
  let label = `+ ${excess.toFixed(2)}`;
  let textColor = "green";
  if (excess < 0) {
    label = `- ${(-excess).toFixed(2)}`;
    textColor = "red";
  }
  let fontSize = 18 / globalScale;
  ctx.font = `${fontSize}px Sans-Serif`;
  let textWidth = ctx.measureText(label).width;
  const bckgDimensions = [textWidth, fontSize].map((n) => n + fontSize * 0.3); // some padding

  ctx.fillStyle = "rgba(255, 255, 255, 0.7)";
  ctx.fillRect(
    node.x - bckgDimensions[0] / 2,
    node.y - (3 * bckgDimensions[1]) / 2,
    bckgDimensions[0],
    bckgDimensions[1]
  );
  ctx.textAlign = "center";
  ctx.textBaseline = "middle";
  ctx.fillStyle = textColor;
  ctx.fillText(label, node.x, node.y - bckgDimensions[1]);
}

export function defaultNodeCanvas(
  node: NodeObject,
  ctx: CanvasRenderingContext2D,
  globalScale: number
) {
  basicCanvas(
    node,
    ctx,
    globalScale,
    node.id.toString(),
    "rgba(255, 255, 255, 0.8)",
    "black"
  );
}

export function sourceNodeCanvas(
  node: NodeObject,
  ctx: CanvasRenderingContext2D,
  globalScale: number,
  excess?: number
) {
  basicCanvas(node, ctx, globalScale, "Source", "lightblue", "white");
  if (excess !== undefined) {
    writeExcessLabelAbove(node, ctx, globalScale, excess);
  }
}

export function sinkNodeCanvas(
  node: NodeObject,
  ctx: CanvasRenderingContext2D,
  globalScale: number,
  excess?: number
) {
  basicCanvas(node, ctx, globalScale, "Sink", "purple", "white");
  if (excess !== undefined) {
    writeExcessLabelAbove(node, ctx, globalScale, excess);
  }
}

export function inactiveNodeCanvas(
  node: NodeObject,
  ctx: CanvasRenderingContext2D,
  globalScale: number
) {
  basicCanvas(node, ctx, globalScale, node.id.toString(), "lightgray", "white");
}

export function excessChangedNodeCanvas(
  node: NodeObject,
  ctx: CanvasRenderingContext2D,
  globalScale: number,
  excess: number
) {
  if (excess < 0) {
    basicCanvas(
      node,
      ctx,
      globalScale,
      node.id.toString(),
      "lightgreen",
      "black"
    );
  } else {
    basicCanvas(
      node,
      ctx,
      globalScale,
      node.id.toString(),
      "rgba(255, 255, 255, 0.8)",
      "black"
    );
  }
  writeExcessLabelAbove(node, ctx, globalScale, excess);
}
