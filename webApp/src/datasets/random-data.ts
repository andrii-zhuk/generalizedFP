function shuffle(array: number[]) {
  let currentIndex = array.length,
    randomIndex;

  while (currentIndex != 0) {
    randomIndex = Math.floor(Math.random() * currentIndex);
    currentIndex--;

    [array[currentIndex], array[randomIndex]] = [
      array[randomIndex],
      array[currentIndex],
    ];
  }

  return array;
}

function randomFloat(left: number, right: number) {
  let value = Math.random() * (right - left) + left;
  return Math.round(value * 100) / 100;
}

function randomLink(
  nodes: number[],
  banned: number[]
): [number, number] | null {
  const bannedSet = new Set<number>(banned);
  const candidates = nodes.filter((x) => !bannedSet.has(x));
  shuffle(candidates);
  if (candidates.length < 2) {
    return null;
  }
  return [candidates[0], candidates[1]];
}

function randomLinkFrom(
  nodes: number[],
  banned: number[],
  from: number
): [number, number] | null {
  const bannedSet = new Set<number>(banned);
  const candidates = nodes.filter((x) => !bannedSet.has(x));
  shuffle(candidates);
  if (candidates.length < 1) {
    return null;
  }
  return [from, candidates[0]];
}

function randomLinkTo(
  nodes: number[],
  banned: number[],
  to: number
): [number, number] | null {
  const bannedSet = new Set<number>(banned);
  const candidates = nodes.filter((x) => !bannedSet.has(x));
  shuffle(candidates);
  if (candidates.length < 1) {
    return null;
  }
  return [candidates[0], to];
}

export function getRandomGraph(N = 10, reverse = false) {
  let nodes = [];
  for (let i = 0; i < N; ++i) nodes.push(i);
  const source = 0;
  const sink = N - 1;
  const M = Math.round((N * (N - 1)) / 4);

  let edges: string[] = [];
  for (let i = 0; i < M - 6; ++i) {
    const [from, to] = randomLink(nodes, [source, sink]);
    const capacity = randomFloat(10, 30);
    const amplification = randomFloat(0.5, 2);
    edges.push(`${from} ${to} ${capacity} ${amplification}`);
  }
  for (let i = 0; i < 3; ++i) {
    const [from, to] = randomLinkFrom(nodes, [source, sink], source);
    const capacity = randomFloat(10, 30);
    const amplification = randomFloat(0.5, 2);
    edges.push(`${from} ${to} ${capacity} ${amplification}`);
  }
  for (let i = 0; i < 3; ++i) {
    const [from, to] = randomLinkTo(nodes, [source, sink], sink);
    const capacity = randomFloat(10, 30);
    const amplification = randomFloat(0.5, 2);
    edges.push(`${from} ${to} ${capacity} ${amplification}`);
  }

  return `${M} ${source} ${sink}\n` + edges.join("\n");
}
