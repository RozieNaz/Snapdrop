export function createSelection(startX, startY, endX, endY) {
  const x = Math.min(startX, endX);
  const y = Math.min(startY, endY);
  const width = Math.abs(endX - startX);
  const height = Math.abs(endY - startY);

  return { x, y, width, height };
}

export function isValidSelection(selection) {
  return selection.width >= 5 && selection.height >= 5;
}
