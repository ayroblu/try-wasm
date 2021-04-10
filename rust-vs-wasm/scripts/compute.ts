const path = [
  {
    id: 0,
    point: {
      x: 98877,
      y: 81007,
    },
  },
  {
    id: 4,
    point: {
      x: 5324,
      y: 79475,
    },
  },
  {
    id: 1,
    point: {
      x: 65637,
      y: 49794,
    },
  },
  {
    id: 5,
    point: {
      x: 40309,
      y: 73254,
    },
  },
  {
    id: 3,
    point: {
      x: 22902,
      y: 77883,
    },
  },
  {
    id: 6,
    point: {
      x: 61921,
      y: 22686,
    },
  },
  {
    id: 2,
    point: {
      x: 36151,
      y: 29682,
    },
  },
  {
    id: 7,
    point: {
      x: 30675,
      y: 24617,
    },
  },
];
const path2 = [
  {
    id: 0,
    point: {
      x: 98877,
      y: 81007,
    },
  },
  {
    id: 1,
    point: {
      x: 65637,
      y: 49794,
    },
  },
  {
    id: 6,
    point: {
      x: 61921,
      y: 22686,
    },
  },
  {
    id: 7,
    point: {
      x: 30675,
      y: 24617,
    },
  },
  {
    id: 2,
    point: {
      x: 36151,
      y: 29682,
    },
  },
  {
    id: 5,
    point: {
      x: 40309,
      y: 73254,
    },
  },
  {
    id: 3,
    point: {
      x: 22902,
      y: 77883,
    },
  },
  {
    id: 4,
    point: {
      x: 5324,
      y: 79475,
    },
  },
];
function getLength(myPath) {
  return myPath.slice(1).reduce((a, n, i) => {
    return a + calculateDistanceBetweenPoints(n.point, myPath[i].point);
  }, 0);
}

function calculateDistanceBetweenPoints(p1, p2) {
  return Math.sqrt((p1.x - p2.x) ** 2 + (p1.y - p2.y) ** 2);
}
console.log("wasm path", getLength(path));
console.log("normal path", getLength(path2));
