import { LinkObject } from "force-graph";

function basicCanvas(
  link: LinkObject,
  ctx: CanvasRenderingContext2D,
  globalScale: number,
  label: string,
  bkgColor: string,
  textColor: string
) {
  if (
    typeof link.source === "string" ||
    typeof link.source === "number" ||
    typeof link.target === "string" ||
    typeof link.target === "number"
  )
    return;
  const [x, y] = [
    link.source.x + link.target.x,
    link.source.y + link.target.y,
  ].map((n) => n / 2);

  let fontSize = 18 / globalScale;
  ctx.font = `${fontSize}px Sans-Serif`;
  let textWidth = ctx.measureText(label).width;
  const bkgDimensions = [textWidth, fontSize].map((n) => n + fontSize * 0.3); // some padding

  if (bkgColor != null) {
    ctx.fillStyle = bkgColor;
    ctx.fillRect(
      x - bkgDimensions[0] / 2,
      y - bkgDimensions[1] / 2,
      bkgDimensions[0],
      bkgDimensions[1]
    );
  }

  ctx.textAlign = "center";
  ctx.textBaseline = "middle";
  ctx.fillStyle = textColor;
  ctx.fillText(label, x, y);
}

export function defaultLinkCanvas(
  link: LinkObject,
  ctx: CanvasRenderingContext2D,
  globalScale: number,
  flow: number,
  amplification: number
) {
  basicCanvas(
    link,
    ctx,
    globalScale,
    `${flow.toFixed(2)} x ${amplification.toFixed(2)}`,
    "rgba(255, 255, 255, 0.7)",
    "green"
  );
}
