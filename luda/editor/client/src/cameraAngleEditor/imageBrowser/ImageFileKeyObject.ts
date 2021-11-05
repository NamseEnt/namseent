export type ImageFileKeyObject = {
  character: string;
  pose: string;
  emotion: string;
};

export function convertImageFileKeyObjectToUrl(
  keyObject: ImageFileKeyObject,
): string {
  const imageFilename = `${keyObject.character}-${keyObject.pose}-${keyObject.emotion}`;
  return `resources/images/${imageFilename}`;
}
