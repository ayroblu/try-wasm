import { points } from "./tsp-def";
import { inspect } from "util";

function log(o: any) {
  console.log(inspect(o, false, null, true /* enable colors */));
}

type Point = {
  x: number;
  y: number;
};
type ResultPath = {
  path: { id: number; point: Point }[];
  traveledPoints: Set<number>;
  cost: number;
};
run();

function tsp() {
  const edges = computeEdges();
  const path: ResultPath = {
    path: [{ id: 0, point: points[0] }],
    traveledPoints: new Set([0]),
    cost: 0,
  };
  return runTsp(edges, path);
}
function runTsp(edges: number[][], path: ResultPath): ResultPath {
  let workingArr = [path];
  let minPath: ResultPath | null = null;
  log(edges);
  while (workingArr.length > 0) {
    const newWorkingArr: ResultPath[] = [];
    workingArr.forEach((path) => {
      points.forEach((p, i) => {
        if (!path.traveledPoints.has(i)) {
          const newPath = {
            ...path,
            cost: path.cost + edges[path.path.slice(-1)[0].id][i],
            path: path.path.concat({
              id: i,
              point: p,
            }),
            traveledPoints: new Set([...path.traveledPoints, i]),
          };
          // log(newPath);
          if (newPath.path.length === points.length) {
            if (!minPath) {
              minPath = newPath;
            } else if (minPath.cost > newPath.cost) {
              minPath = newPath;
            }
          } else {
            if (minPath && newPath.cost >= minPath.cost) {
              return;
            }
            newWorkingArr.push(newPath);
          }
        }
      });
    });
    workingArr = newWorkingArr;
  }
  return minPath!;
}

function computeEdges() {
  return points.map((p1) => {
    return points.map((p2) => calculateDistanceBetweenPoints(p1, p2));
  });
}
function calculateDistanceBetweenPoints(p1: Point, p2: Point) {
  return Math.sqrt((p1.x - p2.x) ** 2 + (p1.y - p2.y) ** 2);
}

function run() {
  console.log("TSP starting");
  const result = tsp();
  console.log(JSON.stringify(result, null, 2));
  console.log("traveledPoints", result.traveledPoints);
}
