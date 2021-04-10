const numPoints = process.argv[2] ?? 20;
console.log("numPoints", numPoints);

const points = getPoints();

printTsPoints(points);
printRustPoints(points);

function getRandomPoint() {
  return {
    x: Math.round(Math.random() * 100_000),
    y: Math.round(Math.random() * 100_000),
  };
}
function getPoints() {
  return Array(numPoints)
    .fill("")
    .map(() => getRandomPoint());
}
function printTsPoints(points: { x: number; y: number }[]) {
  console.log(JSON.stringify(points, null, 2));
}
function printRustPoints(points: { x: number; y: number }[]) {
  console.log("[");
  console.log(
    points
      .map(({ x, y }) => {
        return `Point { x: ${x}, y: ${y} }`;
      })
      .join(",\n")
  );
  console.log("]");
}
