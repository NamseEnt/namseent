import fileSystem from "../../fileSystem/fileSystem";

export async function removeSequence(title: string) {
  await fileSystem.remove(`/sequence/${title}.json`);
  return;
}
