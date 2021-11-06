export type ImageFilenameObject = {
  character: string;
  pose: string;
  emotion: string;
  extension: string;
};

export function convertImageFilenameObjectToUrl(
  filenameObject: ImageFilenameObject,
): string {
  const imageFilename = `${filenameObject.character}-${filenameObject.pose}-${filenameObject.emotion}.${filenameObject.extension}`;
  return `resources/images/${imageFilename}`;
}
